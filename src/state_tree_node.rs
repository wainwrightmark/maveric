use crate::{prelude::*, ChildDeletionPolicy};
use either::Either;
use bevy::{
    ecs::system::{StaticSystemParam, SystemParam},
    prelude::*,
};

pub type ChildKey = Either<u32, &'static str>; //TODO either

pub trait StateTreeRoot: StateTreeNode + Default {
    type ContextParam<'c>: SystemParam;

    fn get_context<'a, 'c, 'w : 'c, 's> (param: StaticSystemParam<'w,'s, Self::ContextParam<'a>>) -> Self::Context<'c>;
}

pub trait StateTreeNode: Eq + Send + Sync + 'static {
    type Context<'c>: HasDetectChanges;

    fn get_components<'c>(
        &self,
        context: &Self::Context<'c>,
        component_commands: &mut impl ComponentCommands,
    );

    fn get_children<'c>(&self, context: &Self::Context<'c>, child_commands: &mut impl ChildCommands);

    fn get_child_deletion_policy<'c>(&self, context: &Self::Context<'c>) -> ChildDeletionPolicy;
    //fn are_children_ordered(&self) -> bool;
}
