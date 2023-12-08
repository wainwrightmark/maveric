use bevy::ecs::system::EntityCommands;

use crate::prelude::*;

pub trait MavericNode: Send + Sync + Sized + PartialEq + 'static {
    type Context: NodeContext;
    const DELETER: &'static dyn Deleter = &NodeDeleter::<Self>::new();

    fn set_components(commands: SetComponentCommands<Self, Self::Context>);
    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context, R>);

    fn on_deleted(&self, _commands: &mut ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }

    /// Do something when the node changes
    fn on_changed(&self, previous: &Self, context: &<Self::Context as NodeContext>::Wrapper<'_>,  world: &World, entity_commands: &mut EntityCommands ){

    }

    fn on_created(&self,context: &<Self::Context as NodeContext>::Wrapper<'_>,  world: &World, entity_commands: &mut EntityCommands ){

    }
}
