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

fn handle_scheduled_changes(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ScheduledChange)>,
    time: Res<Time>,
) {

    let mut count: usize = 0;

    for (entity, mut schedule) in query.iter_mut() {
        match schedule.remaining.checked_sub(time.delta()) {
            Some(new_remaining) => schedule.remaining = new_remaining,
            None => {
                #[cfg(feature="tracing")]
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
    }

    #[cfg(feature="tracing")]
    {
        if count > 0{
            crate::tracing::SCHEDULED_CHANGES.fetch_add(count, std::sync::atomic::Ordering::Relaxed);
        }
    }

}
