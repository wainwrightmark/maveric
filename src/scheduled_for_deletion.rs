use bevy::prelude::*;
use std::time::Duration;

/// a Component that will be deleted when the timer runs out
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, Default)]
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

#[derive(Debug, Default)]
pub struct ScheduleForDeletionPlugin;

impl Plugin for ScheduleForDeletionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, handle_scheduled_for_deletion);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_scheduled_for_deletion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ScheduledForDeletion)>,
) {
    let mut _count: usize = 0;
    for (entity, mut schedule) in query.iter_mut() {
        match schedule.remaining.checked_sub(time.delta()) {
            Some(new_remaining) => schedule.remaining = new_remaining,
            None => {
                #[cfg(feature = "tracing")]
                {
                    _count += 1;
                }

                commands.entity(entity).despawn_recursive();
            }
        }
    }

    #[cfg(feature = "tracing")]
    {
        if _count > 0 {
            crate::tracing::SCHEDULED_DELETIONS
                .fetch_add(_count, std::sync::atomic::Ordering::Relaxed);
        }
    }
}
