use crate::transition::prelude::*;
use bevy::{ecs::component::ComponentId, prelude::*, utils::HashSet};
use std::time::Duration;

pub trait CanRegisterTransition {
    fn register_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Component,
        L::Value: Tweenable;

    fn register_resource_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Resource,
        L::Value: Tweenable;
}

impl CanRegisterTransition for App {
    fn register_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Component,
        L::Value: Tweenable,
    {
        self.add_systems(PreUpdate, step_transition::<L>);

        #[cfg(feature = "tracing")]
        {
            if !self.is_plugin_added::<crate::tracing::TracingPlugin>() {
                self.add_plugins(crate::tracing::TracingPlugin::default());
            }
        }

        #[cfg(debug_assertions)]
        {
            let component_id = self.world.init_component::<Transition<L>>();

            match self.world.get_resource_mut::<RegisteredTransitions>() {
                Some(mut rt) => {
                    rt.0.insert(component_id);
                }
                None => {
                    let mut set = HashSet::new();
                    set.insert(component_id);
                    self.insert_resource(RegisteredTransitions(set));
                }
            }
        }

        self
    }

    fn register_resource_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Resource,
        L::Value: Tweenable,
    {
        self.init_resource::<ResourceTransition<L>>();
        self.add_systems(
            PreUpdate,
            step_resource_transition::<L>
                .run_if(|r: Res<ResourceTransition<L>>| r.transition.is_some()),
        );

        #[cfg(feature = "tracing")]
        {
            if !self.is_plugin_added::<crate::tracing::TracingPlugin>() {
                self.add_plugins(crate::tracing::TracingPlugin::default());
            }
        }

        self
    }
}

/// Inner type used for transition stepping
enum StepResult<L: Lens + GetValueLens + SetValueLens>
where
    L::Value: Tweenable,
{
    Continue,
    Finished,
    Advance(Transition<L>),
}

#[derive(Resource, Clone)]
pub struct ResourceTransition<L: Lens + GetValueLens + SetValueLens>
where
    L::Object: Resource,
    L::Value: Tweenable,
{
    pub transition: Option<Transition<L>>,
}

impl<L: Lens + GetValueLens + SetValueLens> Default for ResourceTransition<L>
where
    L::Object: Resource,
    L::Value: Tweenable,
{
    fn default() -> Self {
        Self { transition: None }
    }
}

fn step_resource_transition<L: Lens + GetValueLens + SetValueLens>(
    mut resource: ResMut<L::Object>,
    mut resource_transition: ResMut<ResourceTransition<L>>,
    time: Res<Time>,
) where
    L::Object: Resource,
    L::Value: Tweenable,
{
    let mut remaining_delta = time.delta();

    if resource_transition.transition.is_some() {
        #[cfg(feature = "tracing")]
        {
            crate::tracing::TRANSITIONS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    'inner: while let Some(transition) = resource_transition.transition.as_mut() {
        let step_result: StepResult<L> = match transition {
            Transition::SetValue { value, next } => {
                L::try_set(resource.as_mut(), value.clone()); //TODO avoid this clone
                match std::mem::take(next) {
                    Some(b) => StepResult::Advance(*b),
                    None => StepResult::Finished,
                }
            }
            Transition::TweenValue {
                destination,
                speed,
                next,
            } => {
                if let Some(mut from) = L::try_get_value(resource.as_ref()) {
                    let transition_result =
                        from.transition_towards(destination, speed, &remaining_delta.as_secs_f32());
                    L::try_set(resource.as_mut(), from);
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
            Transition::Wait { remaining, next } => {
                match remaining_delta.checked_sub(*remaining) {
                    Some(new_remaining_delta) => {
                        // The wait is over
                        remaining_delta = new_remaining_delta;
                        match std::mem::take(next) {
                            Some(b) => StepResult::Advance(*b),
                            None => StepResult::Finished,
                        }
                    }
                    None => {
                        *remaining = remaining.saturating_sub(remaining_delta);
                        StepResult::Continue
                    }
                }
            }
            Transition::Loop(a) => {
                let cloned = a.clone();
                let next = a.build_with_next(Transition::Loop(cloned));
                StepResult::Advance(next)
            }
            Transition::EaseValue {
                start,
                destination,
                elapsed,
                total,
                ease,
                next,
            } => {
                let remaining = *total - *elapsed;
                match remaining_delta.checked_sub(remaining) {
                    Some(new_remaining_delta) => {
                        // The easing is over
                        remaining_delta = new_remaining_delta;
                        L::try_set(resource.as_mut(), destination.clone());
                        match std::mem::take(next) {
                            Some(b) => StepResult::Advance(*b),
                            None => StepResult::Finished,
                        }
                    }
                    None => {
                        *elapsed += remaining_delta;

                        let proportion = elapsed.as_secs_f32() / total.as_secs_f32();

                        let s = ease.ease(proportion);

                        let new_value = start.lerp_value(&destination, s);
                        L::try_set(resource.as_mut(), new_value);

                        StepResult::Continue
                    }
                }
            }
            Transition::ThenEase {
                destination,
                speed,
                ease,
                next,
            } => {
                if let Some(from) = L::try_get_value(resource.as_ref()) {
                    if let Ok(total) = from.duration_to(&destination, speed) {
                        StepResult::Advance(Transition::EaseValue {
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
                break 'inner;
            }
            StepResult::Finished => {
                resource_transition.transition = None;
                break 'inner;
            }
            StepResult::Advance(next) => {
                *transition = next;
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn step_transition<L: Lens + GetValueLens + SetValueLens>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transition<L>, &mut L::Object)>,
) where
    L::Object: Component,
    L::Value: Tweenable,
{
    let mut _count: usize = 0;

    for (entity, mut transition, mut object) in query.iter_mut() {
        #[cfg(feature = "tracing")]
        {
            _count += 1;
        }

        let mut remaining_delta = time.delta();

        'inner: loop {
            let step_result: StepResult<L> = match transition.as_mut() {
                Transition::SetValue { value, next } => {
                    L::try_set(object.as_mut(), value.clone()); //TODO avoid this clone
                    match std::mem::take(next) {
                        Some(b) => StepResult::Advance(*b),
                        None => StepResult::Finished,
                    }
                }
                Transition::TweenValue {
                    destination,
                    speed,
                    next,
                } => {
                    if let Some(mut from) = L::try_get_value(object.as_ref()) {
                        let transition_result = from.transition_towards(
                            destination,
                            speed,
                            &remaining_delta.as_secs_f32(),
                        );
                        L::try_set(object.as_mut(), from);
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
                Transition::Wait { remaining, next } => {
                    match remaining_delta.checked_sub(*remaining) {
                        Some(new_remaining_delta) => {
                            // The wait is over
                            remaining_delta = new_remaining_delta;
                            match std::mem::take(next) {
                                Some(b) => StepResult::Advance(*b),
                                None => StepResult::Finished,
                            }
                        }
                        None => {
                            *remaining = remaining.saturating_sub(remaining_delta);
                            StepResult::Continue
                        }
                    }
                }
                Transition::Loop(a) => {
                    let cloned = a.clone();
                    let next = a.build_with_next(Transition::Loop(cloned));
                    StepResult::Advance(next)
                }
                Transition::EaseValue {
                    start,
                    destination,
                    elapsed,
                    total,
                    ease,
                    next,
                } => {
                    let remaining = *total - *elapsed;
                    match remaining_delta.checked_sub(remaining) {
                        Some(new_remaining_delta) => {
                            // The easing is over
                            remaining_delta = new_remaining_delta;
                            L::try_set(object.as_mut(), destination.clone());
                            match std::mem::take(next) {
                                Some(b) => StepResult::Advance(*b),
                                None => StepResult::Finished,
                            }
                        }
                        None => {
                            *elapsed += remaining_delta;

                            let proportion = elapsed.as_secs_f32() / total.as_secs_f32();

                            let s = ease.ease(proportion);

                            let new_value = start.lerp_value(&destination, s);
                            L::try_set(object.as_mut(), new_value);

                            StepResult::Continue
                        }
                    }
                }
                Transition::ThenEase {
                    destination,
                    speed,
                    ease,
                    next,
                } => {
                    if let Some(from) = L::try_get_value(object.as_ref()) {
                        if let Ok(total) = from.duration_to(&destination, speed) {
                            StepResult::Advance(Transition::EaseValue {
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
                    break 'inner;
                }
                StepResult::Finished => {
                    commands.entity(entity).remove::<Transition<L>>();
                    break 'inner;
                }
                StepResult::Advance(next) => {
                    *transition.as_mut() = next;
                }
            }
        }
    }

    #[cfg(feature = "tracing")]
    {
        if _count > 0 {
            crate::tracing::TRANSITIONS.fetch_add(_count, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

/// A plugin that checks all transition components are registered
/// Should only be added on debug mode
pub(crate) struct CheckTransitionsPlugin;

#[derive(Debug, Resource)]
struct RegisteredTransitions(HashSet<ComponentId>);

impl Plugin for CheckTransitionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_transitions);
    }
}

fn check_transitions(
    world: &World,
    transitions: Option<Res<RegisteredTransitions>>,
    time: Res<Time>,
    mut remaining_time: Local<Duration>,
) {
    match remaining_time.checked_sub(time.delta()) {
        Some(new_remaining) => *remaining_time = new_remaining,
        None => {
            *remaining_time = Duration::from_secs(3);

            for component in world.components().iter().filter(|x| {
                x.name()
                    .starts_with("maveric::transition::step::Transition<")
            }) {
                let is_registered = match &transitions {
                    Some(r) => r.0.contains(&component.id()),
                    None => false,
                };

                if !is_registered {
                    warn!("Unregistered Transition: {}", component.name())
                }
            }
        }
    }
}
