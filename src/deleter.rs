use std::marker::PhantomData;
use crate::prelude::*;

pub trait Deleter: Send + Sync + 'static {
    fn on_deleted<'r>(
        &self,
        entity: Entity,
        commands: &mut ComponentCommands,
        world: &World
    ) -> DeletionPolicy;
}

#[derive(Debug)]
pub(crate) struct NodeDeleter<N: MavericNode> {
    phantom: PhantomData<N>,
}

impl<N: MavericNode> NodeDeleter<N> {
    pub(crate) const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<N: MavericNode> Deleter for NodeDeleter<N> {
    fn on_deleted<'r>(
        &self,
        entity: Entity,
        commands: &mut ComponentCommands,
        world: &World
    ) -> DeletionPolicy {
        if let Some(n) = world.get::<MavericNodeComponent<N>>(entity) {
            N::on_deleted(&n.node, commands)
        } else {
            DeletionPolicy::DeleteImmediately
        }
    }
}
