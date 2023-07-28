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
    pub(crate) fn new(node: N) -> Self {
        Self { node }
    }
}

#[derive( Component)]
pub(crate) struct HierarchyChild<R: StateTreeRoot> {
    pub key: ChildKey,
    pub deleter: &'static dyn Deleter,
    phantom: PhantomData<R>,
}

impl<R: StateTreeRoot> Clone for HierarchyChild<R> {
    fn clone(&self) -> Self {
        Self { key: self.key.clone(), deleter: self.deleter.clone(), phantom: self.phantom.clone() }
    }
}



impl<R: StateTreeRoot> HierarchyChild<R> {
    pub(crate) fn new<N: StateTreeNode>(key: ChildKey) -> Self {
        Self {
            key,
            phantom: PhantomData,
            deleter: N::DELETER
        }
    }
}

#[derive(Debug, Component)]
pub(crate) struct ScheduledForDeletion {
    pub timer: Timer,
}
