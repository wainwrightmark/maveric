use std::{any::type_name, marker::PhantomData, rc::Rc};

use crate::{prelude::*, DeletionPolicy};
use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};

pub(crate) struct CreationCommands<
    'w,
    's,
    'a,
    'b,
    'd,
    'r,
    R: HierarchyRoot,
    NParent: ChildrenAspect,
> {
    ec: &'b mut EntityCommands<'w, 's, 'a>,
    context: &'d <NParent::Context as NodeContext>::Wrapper<'r>,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'a, 'b, 'd, 'r, R: HierarchyRoot, NParent: ChildrenAspect> ChildCommands<NParent>
    for CreationCommands<'w, 's, 'a, 'b, 'd, 'r, R, NParent>
{
    fn add_child<'c, NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
    ) where
        NParent: HasChild<NChild>,
    {
        self.ec.with_children(|cb| {
            let key = key.into();
            let mut cec = cb.spawn_empty();

            let child_context = <NParent as HasChild<NChild>>::convert_context(self.context);
            create_recursive::<R, NParent, NChild>(&mut cec, child, &child_context, key);
        });
    }
}

impl<'w, 's, 'a, 'b, 'c, 'r, R: HierarchyRoot, NParent: ChildrenAspect>
    CreationCommands<'w, 's, 'a, 'b, 'c, 'r, R, NParent>
{
    pub(crate) fn new(
        ec: &'b mut EntityCommands<'w, 's, 'a>,
        context: &'c <NParent::Context as NodeContext>::Wrapper<'r>,
    ) -> Self {
        Self {
            ec,
            context,
            phantom: PhantomData,
        }
    }
}

impl<'w, 's, 'a, 'b, 'c, 'r, R: HierarchyRoot, NParent: ChildrenAspect> ComponentCommands
    for CreationCommands<'w, 's, 'a, 'b, 'c, 'r, R, NParent>
{
    fn insert<T: Bundle>(&mut self, bundle: T) {
        self.ec.insert(bundle);
    }

    fn remove<T: Bundle>(&mut self) {}

    fn get<T: Component>(&self) -> Option<&T> {
        None
    }
}

