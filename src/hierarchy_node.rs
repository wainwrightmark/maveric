use std::marker::PhantomData;

use crate::prelude::*;
use bevy::utils::HashSet;

pub trait HierarchyNode: PartialEq + Send + Sync + 'static {
    type Context: NodeContext;
    fn update<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl UpdateCommands,
    );

    fn on_undeleted<'r>(
        &self,
        _context: &<Self::Context as NodeContext>::Wrapper<'r>,
        _commands: &mut impl ComponentCommands,
    ) {
        // do nothing by default
    }

    #[allow(clippy::unused_variables)]
    fn on_deleted(
        &self,
        _commands: &mut impl ComponentCommands,
        _new_sibling_keys: &HashSet<ChildKey>,
    ) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }
}

pub(crate) trait CanDelete {
    const DELETER: &'static dyn Deleter;
}

impl<N: HierarchyNode> CanDelete for N {
    const DELETER: &'static dyn Deleter = &NodeDeleter::<Self>::new();
}

struct NodeDeleter<N: HierarchyNode> {
    phantom: PhantomData<N>,
}

impl<N: HierarchyNode> NodeDeleter<N> {
    pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<N: HierarchyNode> Deleter for NodeDeleter<N> {
    fn on_deleted(
        &self,
        component_commands: &mut ConcreteComponentCommands,
        new_sibling_keys: &HashSet<ChildKey>,
    ) -> DeletionPolicy {
        if let Some(node) = component_commands
            .entity_ref
            .get::<HierarchyNodeComponent<N>>()
        {
            node.node.on_deleted(component_commands, new_sibling_keys)
        } else {
            DeletionPolicy::DeleteImmediately
        }
    }
}

pub(crate) trait Deleter: Send + Sync + 'static {
    fn on_deleted(
        &self,
        component_commands: &mut ConcreteComponentCommands,
        new_sibling_keys: &HashSet<ChildKey>,
    ) -> DeletionPolicy;
}
