use std::marker::PhantomData;
use crate::prelude::*;

pub trait Deleter: Send + Sync + 'static {
    fn on_deleted<'r>(
        &self,
        entity: Entity,
        commands: &mut ConcreteComponentCommands,
        world: &World
    ) -> DeletionPolicy;
}

#[derive(Debug)]
pub(crate) struct NodeDeleter<N: HierarchyNode> {
    phantom: PhantomData<N>,
}

impl<N: HierarchyNode> NodeDeleter<N> {
    pub(crate) const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<N: HierarchyNode> Deleter for NodeDeleter<N> {
    fn on_deleted<'r>(
        &self,
        entity: Entity,
        commands: &mut ConcreteComponentCommands,
        world: &World
    ) -> DeletionPolicy {
        if let Some(n) = world.get::<HierarchyNodeComponent<N>>(entity) {
            N::on_deleted(&n.node, commands)
        } else {
            DeletionPolicy::DeleteImmediately
        }
    }
}
