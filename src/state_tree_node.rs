use crate::{prelude::*, ChildDeletionPolicy};

use bevy::{
    ecs::system::{StaticSystemParam, SystemParam},
    prelude::*,
};

pub type ChildKey = u32; //TODO either

pub trait StateTreeRoot: StateTreeNode + Default {
    type ContextParam: SystemParam;

    fn get_context(param: StaticSystemParam<Self::ContextParam>) -> Self::Context;
}

pub trait StateTreeNode: Eq + Send + Sync + 'static {
    type Context: DetectChanges;

    fn get_components(
        &self,
        context: &Self::Context,
        component_commands: &mut impl ComponentCommands,
    );

    fn get_children(&self, context: &Self::Context, child_commands: &mut impl ChildCommands);

    fn get_child_deletion_policy(&self, context: &Self::Context) -> ChildDeletionPolicy;
    //fn are_children_ordered(&self) -> bool;
}
