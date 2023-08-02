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

        let from = L::get_value(&component);

        let new_value = Tweenable::transition_towards(
            &from,
            &tp.step.destination,
            &tp.step.speed,
            &delta_seconds,
        );

        if tp.step.destination.approx_eq(&new_value) {

            if tp.step.next.is_some(){
                let mut empty: Option<Box<TransitionStep<L>>> = None;
                let component = tp.as_mut();
                let c_s  = &mut component.step.next;
                std::mem::swap(&mut empty, c_s);
                let next = empty.unwrap();
                component.step = *next;
            }
            else{
                commands.entity(entity).remove::<TransitionPathComponent<L>>();
            }
        }

        <L as SetValueLens>::set(component, new_value);
    }
}
