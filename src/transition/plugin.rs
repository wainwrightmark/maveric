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
    time: Res<Time>,
    mut query: Query<(&mut TransitionPathComponent<L>, &mut L::Object)>,
) where
    L::Object: Component,
    L::Value: Tweenable,
{
    let delta_seconds = time.delta_seconds();

    for (mut tp, mut t) in query.iter_mut() {
        let Some(step) = tp.current_step() else {continue;};
        let component = t.as_mut();

        let from = L::get_value(&component);

        let new_value =
            Tweenable::transition_towards(&from, &step.destination, &step.speed, &delta_seconds);

        if step.destination.approx_eq(&new_value) {
            tp.go_to_next_step();
        }

        info!("Stepped transition of {lens} from {from:?} to {new_value:?}", lens = std::any::type_name::<L>());

        <L as SetValueLens>::set(component, new_value);
    }
}
