use std::marker::PhantomData;

use crate::transition::prelude::*;
use bevy::prelude::*;

pub struct TransitionPlugin<L: Lens + GetValueLens>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    phantom: PhantomData<L>,
}

impl<L: Lens + GetValueLens> Default for TransitionPlugin<L>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn default() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

impl<L: Lens + GetValueLens + SetValueLens> Plugin for TransitionPlugin<L>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, step_transition::<L>);
    }
}

fn step_transition<L: Lens + GetValueLens + SetValueLens>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TransitionPathComponent<L>, &mut L::Object)>,
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
                } else {
                    <L as SetValueLens>::set(component, tp.step.destination.clone());
                    if !tp.try_go_to_next_step() {
                        commands
                            .entity(entity)
                            .remove::<TransitionPathComponent<L>>();
                        return;
                    }
                }
            }
        };

        let from = L::get_value(&component);

        let new_value =
            Tweenable::transition_towards(&from, &tp.step.destination, &speed, &delta_seconds);

        if tp.step.destination.approx_eq(&new_value) {
            if !tp.try_go_to_next_step() {
                commands
                    .entity(entity)
                    .remove::<TransitionPathComponent<L>>();
            }
        }

        <L as SetValueLens>::set(component, new_value);
    }
}
