use bevy::{ecs::system::EntityCommands, prelude::*};
use std::time::Duration;

#[derive(Component)]
pub struct ScheduledChange {
    pub remaining: Duration,
    pub boxed_change: Box<dyn FnOnce(&mut EntityCommands) + 'static + Sync + Send>,
}

pub struct ScheduledChangePlugin;

impl Plugin for ScheduledChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, handle_scheduled_changes);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_scheduled_changes(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ScheduledChange)>,
    time: Res<Time>,
) {
    #[cfg(feature = "tracing")]
    let mut count: usize = 0;

    for (entity, mut schedule) in &mut query {
        if let Some(new_remaining) = schedule.remaining.checked_sub(time.delta()) {
            schedule.remaining = new_remaining;
        } else {
            #[cfg(feature = "tracing")]
            {
                count += 1;
            }

            let mut ec = commands.entity(entity);
            ec.remove::<ScheduledChange>();

            let mut f: Box<
                dyn FnOnce(&mut bevy::ecs::system::EntityCommands) + 'static + Sync + Send,
            > = Box::new(|_| {});

            std::mem::swap(&mut f, &mut schedule.boxed_change);
            f(&mut ec);
        }
    }

    #[cfg(feature = "tracing")]
    {
        if count > 0 {
            crate::tracing::SCHEDULED_CHANGES
                .fetch_add(count, std::sync::atomic::Ordering::Relaxed);
        }
    }
}
