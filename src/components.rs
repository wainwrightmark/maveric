use std::marker::PhantomData;

use bevy::prelude::*;

use crate::prelude::*;

#[derive(Debug, Default, Component)]
pub(crate) struct HierarchyRoot<R: StateTreeRoot>(PhantomData<R>);

#[derive(Debug, Default, Component)]
pub(crate) struct HierarchyNode1<N: StateTreeNode> {
    pub node: N,
}

#[derive(Debug, Default, Component)]
pub(crate) struct HierarchyChild1<R: StateTreeRoot> {
    pub key: ChildKey,
    phantom: PhantomData<R>
}

#[derive(Debug, Component)]
pub(crate) struct ScheduledForDeletion {
    pub timer: Timer,
}
