use crate::transition::prelude::*;
use bevy::prelude::*;
use std::{sync::Arc, time::Duration};

#[derive(Component, Clone)]
pub enum Transition<L: Lens + GetValueLens + SetValueLens>
where
    // L::Object: Component,
    L::Value: Tweenable,
{
    /// Set the lens value
    SetValue {
        value: L::Value,
        next: Option<Box<Self>>,
    },
    /// Gradually transition the lens value
    TweenValue {
        destination: L::Value,
        speed: <L::Value as Tweenable>::Speed,
        next: Option<Box<Self>>,
    },

    EaseValue {
        start: L::Value,
        destination: L::Value,
        elapsed: Duration,
        total: Duration,
        ease: Ease,
        next: Option<Box<Self>>,
    },

    ThenEase {
        destination: L::Value,
        speed: <L::Value as Tweenable>::Speed,
        ease: Ease,
        next: Option<Box<Self>>,
    },

    /// Wait a particular amount of time
    Wait {
        remaining: Duration,
        next: Option<Box<Self>>,
    },
    /// Begin (or continue) a loop
    Loop(Arc<dyn TransitionBuilderTrait<L>>),
}

impl<L: Lens + GetValueLens + SetValueLens> PartialEq for Transition<L>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::SetValue {
                    value: l_value,
                    next: l_next,
                },
                Self::SetValue {
                    value: r_value,
                    next: r_next,
                },
            ) => l_value == r_value && l_next == r_next,
            (
                Self::TweenValue {
                    destination: l_destination,
                    speed: l_speed,
                    next: l_next,
                },
                Self::TweenValue {
                    destination: r_destination,
                    speed: r_speed,
                    next: r_next,
                },
            ) => l_destination == r_destination && l_speed == r_speed && l_next == r_next,
            (
                Self::Wait {
                    remaining: l_remaining,
                    next: l_next,
                },
                Self::Wait {
                    remaining: r_remaining,
                    next: r_next,
                },
            ) => l_remaining == r_remaining && l_next == r_next,
            (Self::Loop(l0), Self::Loop(r0)) => Arc::ptr_eq(l0, r0), //TODO improve this somehow
            _ => false,
        }
    }
}

impl<L: Lens + GetValueLens + SetValueLens> Transition<L>
where
    L::Value: Tweenable,
{
    /// Steps this transition, returns whether the transition is finished and should be deleted
    #[allow(clippy::too_many_lines)]
    pub fn step(&mut self, object: &mut L::Object, mut remaining_delta: Duration) -> bool {
        enum StepResult<L: Lens + GetValueLens + SetValueLens>
        where
            L::Value: Tweenable,
        {
            Continue,
            Finished,
            Advance(Transition<L>),
        }

        loop {
            let step_result: StepResult<L> = match self {
                Self::SetValue { value, next } => {
                    L::try_set(object, value.clone()); //TODO avoid this clone
                    match std::mem::take(next) {
                        Some(b) => StepResult::Advance(*b),
                        None => StepResult::Finished,
                    }
                }
                Self::TweenValue {
                    destination,
                    speed,
                    next,
                } => {
                    if let Some(mut from) = L::try_get_value(object) {
                        let transition_result = from.transition_towards(
                            destination,
                            speed,
                            remaining_delta.as_secs_f32(),
                        );
                        L::try_set(object, from);
                        if let Some(remaining_seconds) = transition_result {
                            remaining_delta =
                                Duration::try_from_secs_f32(remaining_seconds).unwrap_or_default();
                            match std::mem::take(next) {
                                Some(b) => StepResult::Advance(*b),
                                None => StepResult::Finished,
                            }
                        } else {
                            StepResult::Continue
                        }
                    } else {
                        StepResult::Finished
                    }
                }
                Self::Wait { remaining, next } => {
                    if let Some(new_remaining_delta) = remaining_delta.checked_sub(*remaining) {
                        // The wait is over
                        remaining_delta = new_remaining_delta;
                        match std::mem::take(next) {
                            Some(b) => StepResult::Advance(*b),
                            None => StepResult::Finished,
                        }
                    } else {
                        *remaining = remaining.saturating_sub(remaining_delta);
                        StepResult::Continue
                    }
                }
                Self::Loop(a) => {
                    let cloned = a.clone();
                    let next = a.build_with_next(Self::Loop(cloned));
                    StepResult::Advance(next)
                }
                Self::EaseValue {
                    start,
                    destination,
                    elapsed,
                    total,
                    ease,
                    next,
                } => {
                    let remaining = *total - *elapsed;
                    if let Some(new_remaining_delta) = remaining_delta.checked_sub(remaining) {
                        // The easing is over
                        remaining_delta = new_remaining_delta;
                        L::try_set(object, destination.clone());
                        match std::mem::take(next) {
                            Some(b) => StepResult::Advance(*b),
                            None => StepResult::Finished,
                        }
                    } else {
                        *elapsed += remaining_delta;

                        let proportion = elapsed.as_secs_f32() / total.as_secs_f32();

                        let s = ease.ease(proportion);

                        let new_value = start.lerp_value(destination, s);
                        L::try_set(object, new_value);

                        StepResult::Continue
                    }
                }
                Self::ThenEase {
                    destination,
                    speed,
                    ease,
                    next,
                } => {
                    if let Some(from) = L::try_get_value(object) {
                        if let Ok(total) = from.duration_to(destination, speed) {
                            StepResult::Advance(Self::EaseValue {
                                start: from,
                                destination: destination.clone(),
                                elapsed: Duration::ZERO,
                                total,
                                ease: *ease,
                                next: std::mem::take(next),
                            })
                        } else {
                            StepResult::Finished
                        }
                    } else {
                        StepResult::Finished
                    }
                }
            };

            match step_result {
                StepResult::Continue => {
                    return false;
                }
                StepResult::Finished => {
                    return true;
                }
                StepResult::Advance(next) => {
                    *self = next;
                }
            }
        }
    }

    /// Returns remaining duration, or none if this is infinite
    pub fn remaining_duration(&self, start: &L::Value) -> Option<Duration> {
        let mut total: Duration = Duration::default();
        let mut current_value: &L::Value = start;
        let mut current_step = self;

        loop {
            match current_step {
                Self::SetValue { value, next } => {
                    current_value = value;
                    match next {
                        Some(next) => current_step = next,
                        None => return Some(total),
                    }
                }
                Self::TweenValue {
                    destination: to,
                    speed,
                    next,
                } => {
                    total += current_value.duration_to(to, speed).ok()?;
                    current_value = to;
                    match next {
                        Some(next) => current_step = next,
                        None => return Some(total),
                    }
                }
                Self::Wait { remaining, next } => {
                    total += *remaining;
                    match next {
                        Some(next) => current_step = next,
                        None => return Some(total),
                    }
                }
                Self::Loop(_) => return None,
                Self::EaseValue {
                    destination: to,
                    elapsed,
                    total: ease_duration,
                    next,
                    ..
                } => {
                    total += *ease_duration;
                    total -= *elapsed;
                    current_value = to;
                    match next {
                        Some(next) => current_step = next,
                        None => return Some(total),
                    }
                }
                Self::ThenEase {
                    destination: to,
                    speed,
                    next,
                    ..
                } => {
                    total += current_value.duration_to(to, speed).ok()?;
                    current_value = to;
                    match next {
                        Some(next) => current_step = next,
                        None => return Some(total),
                    }
                }
            }
        }
    }
}

impl<L: Lens + GetValueLens + SetValueLens> std::fmt::Debug for Transition<L>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SetValue { value, next } => f
                .debug_struct("SetValue")
                .field("value", value)
                .field("next", next)
                .finish(),
            Self::TweenValue {
                destination,
                speed,
                next,
            } => f
                .debug_struct("TweenValue")
                .field("destination", destination)
                .field("speed", speed)
                .field("next", next)
                .finish(),
            Self::EaseValue {
                start,
                destination,
                elapsed,
                total,
                ease,
                next,
            } => f
                .debug_struct("EaseValue")
                .field("start", start)
                .field("destination", destination)
                .field("elapsed", elapsed)
                .field("total", total)
                .field("ease", ease)
                .field("next", next)
                .finish(),
            Self::ThenEase {
                destination,
                speed,
                ease,
                next,
            } => f
                .debug_struct("ThenEase")
                .field("destination", destination)
                .field("speed", speed)
                .field("ease", ease)
                .field("next", next)
                .finish(),
            Self::Wait { remaining, next } => f
                .debug_struct("Wait")
                .field("remaining", remaining)
                .field("next", next)
                .finish(),
            Self::Loop(..) => f.debug_tuple("Loop").finish(),
        }
    }
}

impl<L: Lens + GetValueLens + SetValueLens> Transition<L>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    pub fn same_destination(&self, other: &Self) -> bool {
        self.destination()
            .is_some_and(|x| Some(x) == other.destination())
    }

    pub const fn destination(&self) -> Option<&L::Value> {
        let mut next: Option<&Self> = Some(self);
        let mut result: Option<&L::Value> = None;

        while let Some(current) = next {
            next = match current {
                Self::SetValue { value, next } => {
                    result = Some(value);
                    match next {
                        Some(b) => Some(b),
                        None => None,
                    }
                }
                Self::TweenValue {
                    destination,
                    speed: _,
                    next,
                }
                | Self::EaseValue {
                    destination, next, ..
                }
                | Self::ThenEase {
                    destination, next, ..
                } => {
                    result = Some(destination);
                    match next {
                        Some(b) => Some(b),
                        None => None,
                    }
                }
                Self::Wait { remaining: _, next } => match next {
                    Some(b) => Some(b),
                    None => None,
                },
                Self::Loop(_) => return None,
            }
        }

        result
    }
}
