use std::marker::PhantomData;

use crate::{prelude::*, DeletionPolicy};
use bevy::{
    ecs::system::{StaticSystemParam, SystemParam},
    prelude::*,
};

pub trait StateTreeRoot: StateTreeNode + Default {
    type ContextParam<'c>: SystemParam;

    fn get_context<'a, 'c, 'w: 'c, 's>(
        param: StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
    ) -> Self::Context<'c>;
}

pub trait StateTreeNode: Eq + Send + Sync + 'static  {
    type Context<'c>: HasDetectChanges;

    fn get_components<'c>(
        &self,
        context: &Self::Context<'c>,
        component_commands: &mut impl ComponentCommands,
    );

    fn get_children<'c>(
        &self,
        context: &Self::Context<'c>,
        child_commands: &mut impl ChildCommands,
    );

    fn on_deleted(&self, component_commands: &mut impl ComponentCommands) -> DeletionPolicy;
}

pub(crate) trait CanDelete {
    const DELETER: &'static dyn Deleter;
}

impl<N: StateTreeNode> CanDelete for N {
    const DELETER: &'static dyn Deleter = &NodeDeleter::<Self>::new();
}

struct NodeDeleter<N: StateTreeNode> {
    phantom: PhantomData<N>,
}

impl<N: StateTreeNode> NodeDeleter<N> {
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<N: StateTreeNode> Deleter for NodeDeleter<N> {
    fn on_deleted(
        &self,
        er: &EntityRef,
        component_commands: &mut ComponentUpdateCommands,
    ) -> DeletionPolicy {
        if let Some(node) = er.get::<HierarchyNode<N>>() {
            node.node.on_deleted(component_commands)
        } else {
            DeletionPolicy::DeleteImmediately
        }
    }
}

pub(crate) trait Deleter: Send + Sync + 'static {
    fn on_deleted(
        &self,
        er: &EntityRef,
        component_commands: &mut ComponentUpdateCommands,
    ) -> DeletionPolicy;
}
