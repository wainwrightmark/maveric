use std::{marker::PhantomData, ops::Add, time::Duration};

use crate::prelude::*;
use crate::transition::prelude::*;
use bevy::prelude::*;


#[derive(Debug, Clone, PartialEq)]
pub struct TransitionStep<V: ComponentVelocity> {
    pub destination: V::C,
    pub velocity: V,
}

#[derive(Component)]
pub(crate) struct TransitionPathComponent<V: ComponentVelocity> {
    pub path: TransitionPath<V>,
    pub index: usize,
}

impl<V: ComponentVelocity> TransitionPathComponent<V> {
    pub fn current_step(&self) -> Option<&TransitionStep<V>> {
        self.path.steps.get(self.index)
    }

    pub fn go_to_next_step(&mut self) {
        self.index += 1;
    }
}

#[derive(Debug, Component)]
pub(crate) struct SuspendedPathComponent<V: ComponentVelocity> {
    pub index: usize,
    pub phantom: PhantomData<V>,
}

#[derive(Clone, PartialEq)]
pub struct TransitionPath<V: ComponentVelocity> {
    pub steps: Vec<TransitionStep<V>>,
}

impl<V: ComponentVelocity> From<TransitionStep<V>> for TransitionPath<V> {
    fn from(value: TransitionStep<V>) -> Self {
        Self { steps: vec![value] }
    }
}

impl<V: ComponentVelocity> TransitionPath<V> {
    pub fn remaining_duration(&self, start: &V::C) -> Duration {
        let mut total: Duration = Duration::default();
        let mut current: &V::C = start;

        for step in self.steps.iter() {
            total += step.velocity.duration(&step.destination, current);
            current = &step.destination;
        }

        total
    }
}