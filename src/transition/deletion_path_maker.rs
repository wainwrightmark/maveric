use std::time::Duration;

use bevy::prelude::*;

pub use super::prelude::*;
use super::speed::calculate_speed;

pub trait DeletionPathMaker<L: Lens + GetValueLens + SetValueLens>:
    Send + Sync + PartialEq + 'static
where
    L::Value: Tweenable,
    L::Object: Component,
{
    fn get_step(&self, previous: &L::Value) -> Option<Transition<L>>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct DurationDeletionPathMaker<L: Lens + GetValueLens>
where
    L::Value: Tweenable,
    L::Object: Component,
{
    duration: Duration,
    destination: L::Value,
}

impl<L: Lens + GetValueLens + SetValueLens> DeletionPathMaker<L> for DurationDeletionPathMaker<L>
where
    L::Value: Tweenable,
    L::Object: Component,
{
    fn get_step(&self, previous: &<L as Lens>::Value) -> Option<Transition<L>> {
        let speed = calculate_speed(previous, &self.destination, self.duration);

        Some(Transition::TweenValue {
            destination: self.destination.clone(),
            speed,
            next: None,
        })
    }
}

impl<L: Lens + GetValueLens> DurationDeletionPathMaker<L>
where
    L::Value: Tweenable,
    L::Object: Component,
{
    pub const fn new(duration: Duration, destination: L::Value) -> Self {
        Self {
            duration,
            destination,
        }
    }
}

impl<L: Lens + GetValueLens + SetValueLens> DeletionPathMaker<L> for ()
where
    L::Value: Tweenable,
    L::Object: Component,
{
    fn get_step(&self, _previous: &<L as Lens>::Value) -> Option<Transition<L>> {
        None
    }
}

impl<L: Lens + GetValueLens + SetValueLens> DeletionPathMaker<L> for Transition<L>
where
    L::Value: Tweenable,
    L::Object: Component,
{
    fn get_step(&self, _previous: &<L as Lens>::Value) -> Option<Self> {
        Some(self.clone())
    }
}
