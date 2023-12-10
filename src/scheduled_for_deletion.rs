use std::time::Duration;
use bevy::prelude::*;

/// a Component that will be deleted when the timer runs out
#[derive(Debug, Component)]
pub struct ScheduledForDeletion {
    pub remaining: Duration,
}

impl ScheduledForDeletion {
    pub fn from_secs(seconds: f32) -> Self {
        Self {
            remaining: Duration::from_secs_f32(seconds),
        }
    }
}
