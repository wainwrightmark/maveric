use crate::transition::prelude::*;
use bevy::prelude::*;
use std::{
    marker::PhantomData,
    time::{Duration, TryFromFloatSecsError},
};

#[derive(Clone)]
pub struct TransitionStep<L: Lens>
where
    L::Value: Tweenable,
{
    pub destination: L::Value,
    pub speed: Option<<L::Value as Tweenable>::Speed>,
    phantom: PhantomData<L>,
    pub next: Option<Box<Self>>,
}

impl<L: Lens> TransitionStep<L>
where
    L::Value: Tweenable,
{
    pub fn remaining_duration(&self, start: &L::Value) -> Result<Duration, TryFromFloatSecsError> {
        let mut total: Duration = Duration::default();
        let mut current_value: &L::Value = start;
        let mut current_step = self;

        'l: loop {
            if let Some(speed) = current_step.speed {
                let step_duration = current_value.duration_to(&current_step.destination, &speed)?;

                total += step_duration;
                current_value = &current_step.destination;
            }

            match &current_step.next {
                Some(x) => current_step = x.as_ref(),
                None => break 'l,
            }
        }

        Ok(total)
    }
}

impl<L: Lens> std::fmt::Debug for TransitionStep<L>
where
    L::Value: Tweenable,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransitionStep")
            .field("destination", &self.destination)
            .field("speed", &self.speed)
            .finish()
    }
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
    pub fn new(
        destination: L::Value,
        speed: Option<<L::Value as Tweenable>::Speed>,
        next: Option<Box<Self>>,
    ) -> Self {
        Self {
            destination,
            speed,
            next,
            phantom: PhantomData,
        }
    }
}

#[derive(Component)]
pub(crate) struct TransitionPathComponent<L: Lens>
where
    L::Value: Tweenable,
{
    pub step: TransitionStep<L>,
}

impl<L: Lens> TransitionPathComponent<L>
where
    L::Value: Tweenable,
{
    pub fn try_go_to_next_step(&mut self)-> bool{
        let mut empty: Option<Box<TransitionStep<L>>> = None;
        let c_s  = &mut self.step.next;
        std::mem::swap(&mut empty, c_s);

        if let Some(next) = empty{
            self.step = *next;
            true
        }
        else{
            false
        }
    }
}

impl<L: Lens> TransitionPathComponent<L> where L::Value: Tweenable {}
