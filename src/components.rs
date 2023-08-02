use std::marker::PhantomData;

use bevy::prelude::*;

use crate::prelude::*;

#[derive(Debug, Default, Component)]
pub(crate) struct HierarchyNodeComponent<N: NodeBase> {
    pub node: N,
}

#[derive(Component)]
pub(crate) struct AncestorComponent<N: AncestorAspect> {
    pub deleter: &'static dyn ChildDeleter<N>,
}

impl<NParent: AncestorAspect> AncestorComponent<NParent> {
    pub(crate) fn new<NChild: HierarchyNode>() -> Self
    where
        NParent: HasChild<NChild>,
    {
        let deleter = <NParent as HasChild<NChild>>::DELETER;
        Self { deleter }
    }
}

#[derive(Component)]
pub(crate) struct HierarchyChildComponent<R: HierarchyRoot> {
    pub key: ChildKey,
    phantom: PhantomData<R>,
}

impl<R: HierarchyRoot> Clone for HierarchyChildComponent<R> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            phantom: self.phantom.clone(),
        }
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
