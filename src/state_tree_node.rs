use crate::{prelude::*, ChildDeletionPolicy};
use either::Either;
use bevy::{
    ecs::system::{StaticSystemParam, SystemParam},
    prelude::*,
};

pub type ChildKey = Either<u32, &'static str>; //TODO either

pub trait StateTreeRoot: StateTreeNode + Default {
    type ContextParam<'a>: SystemParam;

    fn get_context<'a, 'b> (param: StaticSystemParam<Self::ContextParam<'a>>) -> Self::Context<'b>;
}

pub trait StateTreeNode: Eq + Send + Sync + 'static {
    type Context<'b>: DetectChanges;

    fn get_components<'b>(
        &self,
        context: &Self::Context<'b>,
        component_commands: &mut impl ComponentCommands,
    );

    fn get_children<'b>(&self, context: &Self::Context<'b>, child_commands: &mut impl ChildCommands);

    fn get_child_deletion_policy<'b>(&self, context: &Self::Context<'b>) -> ChildDeletionPolicy;
    //fn are_children_ordered(&self) -> bool;
}
