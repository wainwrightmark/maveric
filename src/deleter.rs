use crate::prelude::*;
use std::marker::PhantomData;

pub trait Deleter: Send + Sync + 'static {
    fn on_deleted(
        &self,
        entity: Entity,
        commands: &mut ComponentCommands,
        world: &World,
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
    fn on_deleted(
        &self,
        entity: Entity,
        commands: &mut ComponentCommands,
        world: &World,
    ) -> DeletionPolicy {
        world
            .get::<MavericNodeComponent<N>>(entity)
            .map_or(DeletionPolicy::DeleteImmediately, |n| {
                N::on_deleted(&n.node, commands)
            })
    }
}
