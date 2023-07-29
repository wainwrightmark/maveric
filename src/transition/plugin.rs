use std::{marker::PhantomData, ops::Add, time::Duration};

use crate::prelude::*;
use crate::transition::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct TransitionPlugin<V: ComponentVelocity> {
    phantom: PhantomData<V>,
}

impl<V: ComponentVelocity> Plugin for TransitionPlugin<V> {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, step_transition::<V>);
    }
}

fn step_transition<V: ComponentVelocity>(
    time: Res<Time>,
    mut query: Query<(&mut TransitionPathComponent<V>, &mut V::C)>,
) {
    let delta_seconds = time.delta_seconds();

    for (mut tp, mut t) in query.iter_mut() {
        let Some(step) = tp.current_step() else {continue;};
        let component = t.as_mut();
        step.velocity
            .advance(&step.destination, delta_seconds, component);
        if step.destination == *t {
            tp.go_to_next_step();
        }
    }
}
