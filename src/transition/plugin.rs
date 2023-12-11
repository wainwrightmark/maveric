use std::time::Duration;

use crate::transition::prelude::*;
use bevy::{ecs::component::ComponentId, prelude::*, utils::HashSet};

pub trait CanRegisterTransition {
    fn register_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Component,
        L::Value: Tweenable;
}

impl CanRegisterTransition for App {
    fn register_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Component,
        L::Value: Tweenable,
    {
        self.add_systems(PreUpdate, step_transition::<L>);

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
}

/// Inner type used for transition stepping
enum StepResult<L: Lens + GetValueLens + SetValueLens>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    Continue,
    Finished,
    Advance(Transition<L>),
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
    for (entity, mut transition, mut object) in query.iter_mut() {
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
