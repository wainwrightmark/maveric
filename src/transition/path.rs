use crate::transition::prelude::*;
use bevy::prelude::*;
use std::{
    marker::PhantomData,
    time::{Duration, TryFromFloatSecsError},
};

#[derive(Debug, Clone)]
pub struct TransitionStep<L: Lens>
where
    L::Value: Tweenable,
{
    pub destination: L::Value,
    pub speed: <L::Value as Tweenable>::Speed,
    phantom: PhantomData<L>,
}

impl<L: Lens> PartialEq for TransitionStep<L>
where
    L::Value: Tweenable,
{
    fn eq(&self, other: &Self) -> bool {
        self.destination.approx_eq(&other.destination)
            && self.speed == other.speed
            && self.phantom == other.phantom
    }
}

impl<L: Lens> TransitionStep<L>
where
    L::Value: Tweenable,
{
    pub fn new(destination: L::Value, speed: <L::Value as Tweenable>::Speed) -> Self {
        Self {
            destination,
            speed,
            phantom: PhantomData,
        }
    }
}

#[derive(Component)]
pub(crate) struct TransitionPathComponent<L: Lens>
where
    L::Value: Tweenable,
{
    pub path: TransitionPath<L>,
    pub index: usize,
}

impl<L: Lens> TransitionPathComponent<L>
where
    L::Value: Tweenable,
{
    pub fn current_step(&self) -> Option<&TransitionStep<L>> {
        self.path.steps.get(self.index)
    }

    pub fn go_to_next_step(&mut self) {
        self.index += 1;
    }
}

#[derive(Debug, Component)]
pub(crate) struct SuspendedPathComponent<L: Lens> {
    pub index: usize,
    pub phantom: PhantomData<L>,
}

#[derive(Clone)]
pub struct TransitionPath<L: Lens>
where
    L::Value: Tweenable,
{
    pub steps: Vec<TransitionStep<L>>,
}

impl<L: Lens> PartialEq for TransitionPath<L>
where
    L::Value: Tweenable,
{
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps
    }
}

impl<L: Lens> From<TransitionStep<L>> for TransitionPath<L>
where
    L::Value: Tweenable,
{
    fn from(value: TransitionStep<L>) -> Self {
        Self { steps: vec![value] }
    }
}

impl<L: Lens> TransitionPath<L>
where
    L::Value: Tweenable,
{
    pub fn remaining_duration(&self, start: &L::Value) -> Result<Duration, TryFromFloatSecsError> {
        let mut total: Duration = Duration::default();
        let mut current: &L::Value = start;

        for step in self.steps.iter() {
            let step_duration = current.duration_to(&step.destination, &step.speed)?;

            total += step_duration;
            current = &step.destination;
        }

        Ok(total)
    }
}
