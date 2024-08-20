use std::marker::PhantomData;

use bevy::ecs::system::EntityCommands;

use crate::prelude::*;
pub struct SetChildrenCommands<
    'n,
    'p,
    'c1,
    'world,
    'ec,
    'a,
    'alloc,
    N: PartialEq,
    C: MavericContext,
    R: MavericRoot,
> {
    args: NodeArgs<'n, 'p, 'c1, N, C>,
    world: &'world World,
    ec: &'ec mut EntityCommands<'a>,
    alloc: &'alloc Allocator,
    phantom: PhantomData<R>,
}

impl<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, N: PartialEq, C: MavericContext, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, N, C, R>
{
    pub(crate) fn new(
        args: NodeArgs<'n, 'p, 'c1, N, C>,
        world: &'world World,
        ec: &'ec mut EntityCommands<'a>,
        alloc: &'alloc Allocator,
    ) -> Self {
        Self {
            args,
            world,
            ec,
            alloc,
            phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn ignore_node(
        self,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, (), C, R> {
        self.map_args(|_| &())
    }

    #[must_use]
    pub fn ignore_context(
        self,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, N, (), R> {
        self.map_context(|_| &())
    }

    pub fn map_args<N2: PartialEq>(
        self,
        map: impl Fn(&N) -> &N2,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, N2, C, R> {
        SetChildrenCommands {
            args: self.args.map_node(map),
            phantom: self.phantom,
            world: self.world,
            ec: self.ec,
            alloc: self.alloc,
        }
    }

    pub fn map_context<C2: MavericContext>(
        self,
        map: impl FnOnce(&'c1 C) -> &'c1 C2,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, N, C2, R> {
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
        'cw,
        'cs,
        'world,
        'ec,
        'a,
        'alloc,
        N: ChildTuple<Context<'cw, 'cs> = C>,
        C: MavericContext,
        R: MavericRoot,
    > SetChildrenCommands<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, N, C, R>
{
    pub fn add_children(self) {
        match self.unordered_children_with_node_and_context() {
            Some((args, context, mut commands)) => args.add_children(context, &mut commands),
            None => {}
        }
    }
}

impl<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, N: PartialEq, C: MavericContext, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, N, C, R>
{
    pub const fn no_children(self) {}

    pub fn ordered_children_with_node_and_context(
        self,
    ) -> Option<(
        &'n N,
        &'c1 C,
        OrderedChildCommands<'ec, 'a, 'world, 'alloc, R>,
    )> {
        self.ordered()
            .map(|(node_args, commands)| (node_args.node, node_args.context, commands))
    }

    pub fn unordered_children_with_node_and_context(
        self,
    ) -> Option<(
        &'n N,
        &'c1 C,
        UnorderedChildCommands<'ec, 'a, 'world, 'alloc, R>,
    )> {
        self.unordered()
            .map(|(node_args, commands)| (node_args.node, node_args.context, commands))
    }

    /// Gives you full access to args and commands
    pub fn ordered(
        self,
    ) -> Option<(
        NodeArgs<'n, 'p, 'c1, N, C>,
        OrderedChildCommands<'ec, 'a, 'world, 'alloc, R>,
    )> {
        if self.args.is_hot() {
            let occ = OrderedChildCommands::<R>::new(self.ec, self.world, self.alloc);

            Some((self.args, occ))
        } else {
            None
        }
    }

    /// Gives you full access to args and commands
    pub fn unordered(
        self,
    ) -> Option<(
        NodeArgs<'n, 'p, 'c1, N, C>,
        UnorderedChildCommands<'ec, 'a, 'world, 'alloc, R>,
    )> {
        if self.args.is_hot() {
            let ucc = UnorderedChildCommands::<R>::new(self.ec, self.world, self.alloc);
            Some((self.args, ucc))
        } else {
            None
        }
    }
}

impl<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, (), (), R>
{
    pub fn ordered_children(self) -> Option<OrderedChildCommands<'ec, 'a, 'world, 'alloc, R>> {
        self.ordered().map(|(_, commands)| commands)
    }

    pub fn unordered_children(self) -> Option<UnorderedChildCommands<'ec, 'a, 'world, 'alloc, R>> {
        self.unordered().map(|(_, commands)| commands)
    }
}

impl<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, N: PartialEq, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, N, (), R>
{
    pub fn ordered_children_with_node(
        self,
    ) -> Option<(&'n N, OrderedChildCommands<'ec, 'a, 'world, 'alloc, R>)> {
        self.ordered_children_with_node_and_context()
            .map(|(node, _, commands)| (node, commands))
    }

    pub fn unordered_children_with_node(
        self,
    ) -> Option<(&'n N, UnorderedChildCommands<'ec, 'a, 'world, 'alloc, R>)> {
        self.unordered_children_with_node_and_context()
            .map(|(node, _, commands)| (node, commands))
    }
}

impl<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, C: MavericContext, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'world, 'ec, 'a, 'alloc, (), C, R>
{
    pub fn ordered_children_with_context(
        self,
    ) -> Option<(&'c1 C, OrderedChildCommands<'ec, 'a, 'world, 'alloc, R>)> {
        self.ordered_children_with_node_and_context()
            .map(|(_n, c, cc)| (c, cc))
    }

    pub fn unordered_children_with_context(
        self,
    ) -> Option<(&'c1 C, UnorderedChildCommands<'ec, 'a, 'world, 'alloc, R>)> {
        self.unordered_children_with_node_and_context()
            .map(|(_n, c, cc)| (c, cc))
    }
}
