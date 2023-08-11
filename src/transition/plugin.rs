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
                } else {
                    <L as SetValueLens>::try_set(component, tp.step.destination.clone());
                    if !tp.try_go_to_next_step() {
                        commands.entity(entity).remove::<Transition<L>>();
                        return;
                    }
                }
            }
        };

        // info!(
        //     "Transition: {lens:?} {delta_seconds:?}",
        //     lens = std::any::type_name::<L>()
        // );

        let from = L::try_get_value(&component);

        let Some(from) = from else {return;};

        let new_value =
            Tweenable::transition_towards(&from, &tp.step.destination, &speed, &delta_seconds);

        //info!("Transition from {from:?} to {new_value:?}");

        if tp.step.destination.approx_eq(&new_value) && !tp.try_go_to_next_step() {
            commands.entity(entity).remove::<Transition<L>>();
        }

        <L as SetValueLens>::try_set(component, new_value);
    }
}
