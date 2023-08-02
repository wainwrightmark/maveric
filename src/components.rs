use std::{marker::PhantomData, env::Args};

use bevy::prelude::*;

use crate::prelude::*;

#[derive(Debug, Default, Component)]
pub(crate) struct HierarchyRootComponent<R: HierarchyRoot>(PhantomData<R>);

#[derive(Debug, Default, Component)]
pub(crate) struct HierarchyNodeComponent<N: NodeBase> {
    pub args: N::Args,
}


#[derive( Component)]
pub(crate) struct HierarchyChildComponent<R: HierarchyRoot> {
    pub key: ChildKey,
    // pub deleter: &'static dyn Deleter,
    phantom: PhantomData<R>,
}

impl<R: HierarchyRoot> Clone for HierarchyChildComponent<R> {
    fn clone(&self) -> Self {
        Self { key: self.key.clone(),  phantom: self.phantom.clone() }
    }
}



impl<R: HierarchyRoot> HierarchyChildComponent<R> {
    pub(crate) fn new<N: HierarchyNode>(key: ChildKey) -> Self {
        Self {
            key,
            phantom: PhantomData,

        }
    }
}

#[derive(Debug, Component)]
pub(crate) struct ScheduledForDeletion {
    pub timer: Timer,
}
