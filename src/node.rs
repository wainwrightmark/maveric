use bevy::ecs::system::EntityCommands;

use crate::prelude::*;

pub trait MavericNode: Send + Sync + Sized + PartialEq + 'static {
    type Context<'w, 's>: MavericContext;
    const DELETER: &'static dyn Deleter = &NodeDeleter::<Self>::new();

    fn set_components(commands: SetComponentCommands<Self, Self::Context<'_, '_>>);
    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context<'_, '_>, R>);

    fn on_deleted(&self, _commands: &mut ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }

    #[allow(unused_variables)]
    /// Do something when the node changes
    fn on_changed(
        &self,
        previous: &Self,
        context: &Self::Context<'_, '_>,
        world: &World,
        entity_commands: &mut EntityCommands,
    ) {
    }

    #[allow(unused_variables)]
    fn on_created(
        &self,
        context: &Self::Context<'_, '_>,
        world: &World,
        entity_commands: &mut EntityCommands,
    ) {
    }

    /// Should this node be deleted and recreated
    fn should_recreate(&self, _previous: &Self, _context: &Self::Context<'_, '_>) -> bool {
        false
    }
}
