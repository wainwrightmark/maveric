use std::{marker::PhantomData, ops::Add, time::Duration};

use crate::prelude::*;
use bevy::prelude::*;

pub trait ComponentVelocity: PartialEq + Clone + Send + Sync + 'static {
    type C: Component + PartialEq + Clone;

    /// Advance the component towards the destination
    fn advance(&self, destination: &Self::C, delta_seconds: f32, component: &mut Self::C);

    /// How long it will take to get from the start to the destination
    fn duration(&self, destination: &Self::C, start: &Self::C) -> Duration;
}