use std::marker::PhantomData;

use bevy::prelude::*;

use crate::prelude::*;

#[derive(Debug, Default, Component)]
pub(crate) struct MavericNodeComponent<N: MavericNode> {
    pub node: N,
}

impl<N: MavericNode> MavericNodeComponent<N> {
    pub(crate) fn new(node: N) -> Self {
        Self { node }
    }
}

#[derive(Component)]
pub(crate) struct MavericChildComponent<R: MavericRootChildren> {
    pub key: ChildKey,
    pub deleter: &'static dyn Deleter,
    phantom: PhantomData<R>,
}

impl<R: MavericRootChildren> MavericChildComponent<R> {
    pub(crate) fn new<N: MavericNode>(key: ChildKey) -> Self {
        let deleter = N::DELETER;
        Self {
            key,
            deleter,
            phantom: PhantomData,
        }
    }
}

#[derive(Debug, Component)]
pub(crate) struct ScheduledForDeletion {
    pub timer: Timer,
}
