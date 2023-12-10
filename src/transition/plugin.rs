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

#[allow(clippy::needless_pass_by_value)]
fn step_transition<L: Lens + GetValueLens + SetValueLens>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transition<L>, &mut L::Object)>,
) where
    L::Object: Component,
    L::Value: Tweenable,
{
    let delta_seconds = time.delta_seconds();

    for (entity, mut tp, mut t) in query.iter_mut() {
        let component = t.as_mut();

        let speed = {
            loop {
                if let Some(speed) = tp.step.speed {
                    break speed;
                }
                <L as SetValueLens>::try_set(component, tp.step.destination.clone());
                if !tp.try_go_to_next_step() {
                    commands.entity(entity).remove::<Transition<L>>();
                    return;
                }
            }
        };

        // println!(
        //     "Transition: {lens:?} {delta_seconds:?}",
        //     lens = std::any::type_name::<L>()
        // );

        let from = L::try_get_value(component);

        let Some(from) = from else {
            return;
        };

        let new_value =
            Tweenable::transition_towards(&from, &tp.step.destination, &speed, &delta_seconds);

        //info!("Transition from {from:?} to {new_value:?}");

        if tp.step.destination.eq(&new_value) && !tp.try_go_to_next_step() {
            commands.entity(entity).remove::<Transition<L>>();
        }

        <L as SetValueLens>::try_set(component, new_value);
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
