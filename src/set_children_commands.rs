use std::marker::PhantomData;

use bevy::ecs::system::EntityCommands;

use crate::prelude::*;
pub struct SetChildrenCommands<
    'n,
    'p,
    'c1,
    'c2,
    'world,
    'ec,
    'w,
    's,
    'a,
    'alloc,
    N: PartialEq,
    C: NodeContext,
    R: MavericRoot,
> {
    args: NodeArgs<'n, 'p, 'c1, 'c2, N, C>,
    world: &'world World,
    ec: &'ec mut EntityCommands<'w, 's, 'a>,
    alloc: &'alloc mut Allocator,
    phantom: PhantomData<R>,
}

impl<
        'n,
        'p,
        'c1,
        'c2,
        'world,
        'ec,
        'w,
        's,
        'a,
        'alloc,
        N: PartialEq,
        C: NodeContext,
        R: MavericRoot,
    > SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, N, C, R>
{
    pub(crate) fn new(
        args: NodeArgs<'n, 'p, 'c1, 'c2, N, C>,
        world: &'world World,
        ec: &'ec mut EntityCommands<'w, 's, 'a>,
        alloc: &'alloc mut Allocator,
    ) -> Self {
        Self {
            args,
            world,
            ec,
            alloc,
            phantom: PhantomData,
        }
    }

    #[must_use] pub fn ignore_node(
        self,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, (), C, R> {
        self.map_args(|_| &())
    }

    #[must_use] pub fn ignore_context(
        self,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, N, NoContext, R>
    {
        self.map_context(|_| &())
    }

    pub fn map_args<N2: PartialEq>(
        self,
        map: impl Fn(&N) -> &N2,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, N2, C, R> {
        SetChildrenCommands {
            args: self.args.map_node(map),
            phantom: self.phantom,
            world: self.world,
            ec: self.ec,
            alloc: self.alloc,
        }
    }

    pub fn map_context<C2: NodeContext>(
        self,
        map: impl FnOnce(&'c1 C::Wrapper<'c2>) -> &'c1 C2::Wrapper<'c2>,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, N, C2, R> {
        SetChildrenCommands {
            args: self.args.map_context(map),
            phantom: self.phantom,
            world: self.world,
            ec: self.ec,
            alloc: self.alloc,
        }
    }
}

impl<
        'n,
        'p,
        'c1,
        'c2,
        'world,
        'ec,
        'w,
        's,
        'a,
        'alloc,
        N: ChildTuple<Context = C>,
        C: NodeContext,
        R: MavericRoot,
    > SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, N, C, R>
{
    pub fn add_children(self) {
        self.unordered_children_with_node_and_context(|args, context, commands| {
            args.add_children(context, commands);
        });
    }
}

impl<
        'n,
        'p,
        'c1,
        'c2,
        'world,
        'ec,
        'w,
        's,
        'a,
        'alloc,
        N: PartialEq,
        C: NodeContext,
        R: MavericRoot,
    > SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, N, C, R>
{
    pub const fn no_children(self) {}

    pub fn ordered_children_with_node_and_context(
        self,

        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>, &mut OrderedChildCommands<R>),
    ) {
        self.ordered(|a, cc| f(a.node, a.context, cc));
    }

    pub fn unordered_children_with_node_and_context(
        self,

        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered(|a, cc| f(a.node, a.context, cc));
    }

    /// Gives you full access to args and commands
    /// You must add children if you call this, even if not hot
    pub fn ordered(
        self,

        f: impl FnOnce(&NodeArgs<'n, 'p, 'c1, 'c2, N, C>, &mut OrderedChildCommands<R>),
    ) {
        if self.args.is_hot() {
            let mut occ = OrderedChildCommands::<R>::new(self.ec, self.world, self.alloc);
            f(&self.args, &mut occ);
            occ.finish();
        }
    }

    /// Gives you full access to args and commands
    /// You must add children if you call this, even if not hot
    pub fn unordered(
        self,

        f: impl FnOnce(&NodeArgs<'n, 'p, 'c1, 'c2, N, C>, &mut UnorderedChildCommands<R>),
    ) {
        if self.args.is_hot() {
            let mut ucc = UnorderedChildCommands::<R>::new(self.ec, self.world, self.alloc);
            f(&self.args, &mut ucc);
            ucc.finish();
        }
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, (), NoContext, R>
{
    pub fn ordered_children(self, f: impl FnOnce(&mut OrderedChildCommands<R>)) {
        self.ordered_children_with_node_and_context(|_, _, cc| f(cc));
    }

    pub fn unordered_children(self, f: impl FnOnce(&mut UnorderedChildCommands<R>)) {
        self.unordered_children_with_node_and_context(|_, _, cc| f(cc));
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, N: PartialEq, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, N, NoContext, R>
{
    pub fn ordered_children_with_node(self, f: impl FnOnce(&'n N, &mut OrderedChildCommands<R>)) {
        self.ordered_children_with_node_and_context(|n, _, cc| f(n, cc));
    }

    pub fn unordered_children_with_node(
        self,

        f: impl FnOnce(&'n N, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_with_node_and_context(|n, _, cc| f(n, cc));
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, C: NodeContext, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, 'alloc, (), C, R>
{
    pub fn ordered_children_with_context(
        self,

        f: impl FnOnce(&'c1 C::Wrapper<'c2>, &mut OrderedChildCommands<R>),
    ) {
        self.ordered_children_with_node_and_context(|_n, c, cc| f(c, cc));
    }

    pub fn unordered_children_with_context(
        self,

        f: impl FnOnce(&'c1 C::Wrapper<'c2>, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_with_node_and_context(|_n, c, cc| f(c, cc));
    }
}
