use std::{sync::Arc, time::Duration};

use bevy::prelude::*;

pub use super::prelude::*;
use super::speed::calculate_speed;

pub trait DeletionPathMaker<L: Lens + GetValueLens>: Send + Sync + PartialEq + 'static
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    fn get_step(&self, previous: &L::Value) -> Option<Arc<TransitionStep<L>>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct DurationDeletionPathMaker<L: Lens + GetValueLens>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    duration: Duration,
    destination: L::Value,
}

impl<L: Lens + GetValueLens> DeletionPathMaker<L> for DurationDeletionPathMaker<L>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    fn get_step(&self, previous: &<L as Lens>::Value) -> Option<Arc<TransitionStep<L>>> {
        let out_speed = calculate_speed(previous, &self.destination, self.duration);

        Some(TransitionStep::new_arc(
            self.destination.clone(),
            Some(out_speed),
            NextStep::None,
        ))
    }
}

impl<L: Lens + GetValueLens> DurationDeletionPathMaker<L>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    pub const fn new(duration: Duration, destination: L::Value) -> Self {
        Self {
            duration,
            destination,
        }
    }
}

impl<L: Lens + GetValueLens> DeletionPathMaker<L> for ()
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    fn get_step(&self, _previous: &<L as Lens>::Value) -> Option<Arc<TransitionStep<L>>> {
        None
    }
}

impl<L: Lens + GetValueLens> DeletionPathMaker<L> for TransitionStep<L>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    fn get_step(&self, _previous: &<L as Lens>::Value) -> Option<Arc<Self>> {
        Some(Arc::new(self.clone()))
    }
}
