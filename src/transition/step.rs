use crate::transition::prelude::*;
use bevy::prelude::*;
use std::{marker::PhantomData, sync::Arc, time::Duration};

#[derive(Clone, PartialEq)]
pub struct TransitionStep<L: Lens>
where
    L::Value: Tweenable,
{
    pub destination: L::Value,
    pub speed: Option<<L::Value as Tweenable>::Speed>,
    phantom: PhantomData<L>,
    pub next: NextStep<L>,
}

impl<L: Lens> TransitionStep<L>
where
    L::Value: Tweenable,
{
    /// Returns remaining duration, or none if this is infinite
    pub fn remaining_duration(&self, start: &L::Value) -> Option<Duration> {
        let mut total: Duration = Duration::default();
        let mut current_value: &L::Value = start;
        let mut current_step = self;

        'l: loop {
            if let Some(speed) = current_step.speed {
                let step_duration = current_value
                    .duration_to(&current_step.destination, &speed)
                    .ok()?;

                total += step_duration;
                current_value = &current_step.destination;
            }

            match &current_step.next {
                NextStep::None => break 'l,
                NextStep::Step(arc) => current_step = arc.as_ref(),
                NextStep::Cycle(..) => return None,
            }
        }

        Some(total)
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

impl<L: Lens> TransitionStep<L>
where
    L::Value: Tweenable,
{
    pub fn new_arc(
        destination: L::Value,
        speed: Option<<L::Value as Tweenable>::Speed>,
        next: NextStep<L>,
    ) -> Arc<Self> {
        Arc::new(Self {
            destination,
            speed,
            next,
            phantom: PhantomData,
        })
    }

    /// # Panics
    /// If `steps` is empty
    pub fn new_cycle(
        steps: impl ExactSizeIterator
            + DoubleEndedIterator<Item = (L::Value, <L::Value as Tweenable>::Speed)>,
    ) -> Arc<Self> {
        Arc::new_cyclic(move |weak| {
            let mut next = NextStep::Cycle(weak.clone());

            for (index, (destination, speed)) in steps.enumerate().rev() {
                let step = Self {
                    destination,
                    speed: Some(speed),
                    phantom: PhantomData,
                    next,
                };
                if index == 0 {
                    return step;
                }
                next = NextStep::Step(Arc::new(step));
            }
            panic!("cannot create transition cycle with no steps")
        })
    }
}

#[derive(Component)]
pub struct Transition<L: Lens>
where
    L::Value: Tweenable,
{
    pub(crate) step: Arc<TransitionStep<L>>,
    start: Arc<TransitionStep<L>>,
}

impl<L: Lens> Transition<L>
where
    L::Value: Tweenable,
{
    pub fn new(step: Arc<TransitionStep<L>>) -> Self {
        Self {
            start: step.clone(),
            step,
        }
    }

    pub fn starts_with(&self, step: &TransitionStep<L>) -> bool {
        self.start.as_ref().eq(step)
    }

    pub(crate) fn try_go_to_next_step(&mut self) -> bool {
        match &self.step.next {
            NextStep::None => {
                //info!("No next step");
                false
            }
            NextStep::Step(step) => {
                //info!("Moved to next step");
                self.step = step.clone();
                true
            }
            NextStep::Cycle(weak) => match weak.upgrade() {
                Some(step) => {
                    self.step = step.clone();
                    true
                }
                None => false,
            },
        }
    }
}

impl<L: Lens> Transition<L> where L::Value: Tweenable {}
