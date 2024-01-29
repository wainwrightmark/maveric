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
    L::Object: Component,
    L::Value: Tweenable,
{
    /// Returns remaining duration, or none if this is infinite
    pub fn remaining_duration(&self, start: &L::Value) -> Option<Duration> {
        let mut total: Duration = Duration::default();
        let mut current_value: &L::Value = start;
        let mut current_step = self;

        loop {
            match current_step {
                Transition::SetValue { value, next } => {
                    current_value = value;
                    match next {
                        Some(next) => current_step = next,
                        None => return Some(total),
                    }
                }
                Transition::TweenValue {
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
                Transition::Wait { remaining, next } => {
                    total += *remaining;
                    match next {
                        Some(next) => current_step = next,
                        None => return Some(total),
                    }
                }
                Transition::Loop(_) => return None,
                Transition::EaseValue {
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
                Transition::ThenEase {
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

    pub fn destination(&self) -> Option<&L::Value> {
        let mut next: Option<&Transition<L>> = Some(self);
        let mut result: Option<&L::Value> = None;

        while let Some(current) = next {
            next = match current {
                Transition::SetValue { value, next } => {
                    result = Some(value);
                    match next {
                        Some(b) => Some(&b),
                        None => None,
                    }
                }
                Transition::TweenValue {
                    destination,
                    speed: _,
                    next,
                } => {
                    result = Some(destination);
                    match next {
                        Some(b) => Some(&b),
                        None => None,
                    }
                }
                Transition::Wait { remaining: _, next } => match next {
                    Some(b) => Some(&b),
                    None => None,
                },
                Transition::Loop(_) => return None,
                Transition::EaseValue {
                    destination,
                    next,
                    ..

                } => {
                    result = Some(destination);
                    match next {
                        Some(b) => Some(&b),
                        None => None,
                    }
                }
                Transition::ThenEase {
                    destination,
                    next,
                    ..
                } => {
                    result = Some(destination);
                    match next {
                        Some(b) => Some(&b),
                        None => None,
                    }
                }
            }
        }

        result
    }
}
