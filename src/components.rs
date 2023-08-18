use std::marker::PhantomData;

use bevy::prelude::*;

use crate::prelude::*;

#[derive(Debug, Default, Component)]
pub(crate) struct HierarchyNodeComponent<N: HierarchyNode> {
    pub node: N,
}

impl<N: HierarchyNode> HierarchyNodeComponent<N> {
    pub(crate) fn new(node: N) -> Self {
        Self { node }
    }
}

#[derive(Component)]
pub(crate) struct HierarchyChildComponent<R: HierarchyRootChildren> {
    pub key: ChildKey,
    pub deleter: &'static dyn Deleter,
    phantom: PhantomData<R>,
}

impl<R: HierarchyRootChildren> HierarchyChildComponent<R> {
    pub(crate) fn new<N: HierarchyNode>(key: ChildKey) -> Self {
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
