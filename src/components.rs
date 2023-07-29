use std::marker::PhantomData;

use bevy::prelude::*;

use crate::prelude::*;

#[derive(Debug, Default, Component)]
pub(crate) struct HierarchyRootComponent<R: HierarchyRoot>(PhantomData<R>);

#[derive(Debug, Default, Component)]
pub(crate) struct HierarchyNodeComponent<N: HierarchyNode> {
    pub node: N,
}

impl<N: HierarchyNode> HierarchyNodeComponent<N> {
    pub(crate) fn new(node: N) -> Self {
        Self { node }
    }
}

#[derive( Component)]
pub(crate) struct HierarchyChildComponent<R: HierarchyRoot> {
    pub key: ChildKey,
    pub deleter: &'static dyn Deleter,
    phantom: PhantomData<R>,
}

impl<R: HierarchyRoot> Clone for HierarchyChildComponent<R> {
    fn clone(&self) -> Self {
        Self { key: self.key.clone(), deleter: self.deleter.clone(), phantom: self.phantom.clone() }
    }
}



impl<R: HierarchyRoot> HierarchyChildComponent<R> {
    pub(crate) fn new<N: HierarchyNode>(key: ChildKey) -> Self {
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
