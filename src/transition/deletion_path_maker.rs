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

#[derive(Debug, Clone)]
pub struct DurationDeletionPathMaker<L: Lens + GetValueLens>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    duration: Duration,
    destination: L::Value,
}

impl<L: Lens + GetValueLens> PartialEq for DurationDeletionPathMaker<L>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration && self.destination.approx_eq(&other.destination)
    }
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
            None,
        ))
    }
}

impl<L: Lens + GetValueLens> DurationDeletionPathMaker<L>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    pub fn new(duration: Duration, destination: L::Value) -> Self {
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
    fn get_step(&self, _previous: &<L as Lens>::Value) -> Option<Arc<TransitionStep<L>>> {
        Some(Arc::new(self.clone()))
    }
}
