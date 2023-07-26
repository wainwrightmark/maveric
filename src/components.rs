use std::marker::PhantomData;

use bevy::prelude::*;

use crate::prelude::*;

#[derive(Debug, Default, Component)]
pub(crate) struct HierarchyRoot<R: StateTreeRoot>(PhantomData<R>);

#[derive(Debug, Default, Component)]
pub(crate) struct HierarchyNode<N: StateTreeNode> {
    pub node: N,
}

impl<N: StateTreeNode> HierarchyNode<N> {
    pub(crate) fn new(node: N) -> Self { Self { node } }
}

#[derive(Debug,  Component)]
pub(crate) struct HierarchyChild<R: StateTreeRoot> {
    pub key: ChildKey,
    phantom: PhantomData<R>
}

impl<R: StateTreeRoot> HierarchyChild<R> {
    pub(crate) fn new(key: ChildKey) -> Self { Self { key, phantom: PhantomData } }
}

#[derive(Debug, Component)]
pub(crate) struct ScheduledForDeletion {
    pub timer: Timer,
}
