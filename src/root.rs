use bevy::ecs::system::SystemParam;

use crate::prelude::*;

pub trait MavericRoot: Send + Sync + 'static {
    type Context<'w, 's>: MavericContext;

    fn set_children(
        context: &<Self::Context<'_, '_> as SystemParam>::Item<'_, '_>,
        commands: &mut impl ChildCommands,
    );
}
