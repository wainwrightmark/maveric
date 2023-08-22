use std::marker::PhantomData;
use bevy::ecs::system::EntityCommands;
use crate::prelude::*;

pub struct SetChildrenCommands<
    'n,
    'p,
    'c1,
    'c2,
    'w,
    's,
    'a,
    'world,
    N: PartialEq,
    C: NodeContext,
    R: HierarchyRoot,
> {
    args: &'n N,
    previous: Option<&'p N>,
    context: &'c1 C::Wrapper<'c2>,
    event: SetEvent,
    commands: EntityCommands<'w, 's, 'a>,
    world: &'world World,
    phantom: PhantomData<R>,
}

impl<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N: PartialEq, C: NodeContext, R: HierarchyRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N, C, R>
{
    pub(crate) fn new(
        args: &'n N,
        previous: Option<&'p N>,
        context: &'c1 C::Wrapper<'c2>,
        event: SetEvent,
        commands: EntityCommands<'w, 's, 'a>,
        world: &'world World,
    ) -> Self {
        Self {
            args,
            previous,
            context,
            event,
            commands,
            world,
            phantom: PhantomData::default(),
        }
    }

    pub fn ignore_args(
        self,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, (), C, R> {
        self.map_args(|_| &())
    }

    pub fn ignore_context(
        self,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N, NoContext, R> {
        self.map_context(|_| &())
    }

    pub fn map_args<N2: PartialEq>(
        self,
        map: impl Fn(&N) -> &N2,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N2, C, R> {
        let new_args = map(self.args);

        SetChildrenCommands {
            args: new_args,
            previous: self.previous.map(map),
            context: self.context,
            event: self.event,
            commands: self.commands,
            world: self.world,
            phantom: self.phantom,
        }
    }

    pub fn map_context<C2: NodeContext>(
        self,
        map: impl FnOnce(&'c1 C::Wrapper<'c2>) -> &'c1 C2::Wrapper<'c2>,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N, C2, R> {
        let new_context = map(self.context);
        SetChildrenCommands {
            args: self.args,
            previous: self.previous,
            context: new_context,
            event: self.event,
            commands: self.commands,
            world: self.world,
            phantom: self.phantom,
        }
    }

    /// Returns true if this is a creation or undeletion, or if the context or args have changed
    fn is_hot(&self) -> bool {
        match self.event {
            SetEvent::Created | SetEvent::Undeleted => return true,
            SetEvent::Updated => {
                C::has_changed(self.context)
                    || self.previous.map(|p| !p.eq(self.args)).unwrap_or(true)
            }
        }
    }
}

impl<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N: PartialEq, C: NodeContext, R: HierarchyRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N, C, R>
{
    pub fn ordered_children_with_args_and_context(
        self,

        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>, &mut OrderedChildCommands<R>),
    ) {
        self.ordered_children_with_args_and_context_advanced(|n, _, c, _, cc| f(n, c, cc))
    }

    pub fn unordered_children_with_args_and_context(
        self,

        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_with_args_and_context_advanced(|n, _, c, _, cc| f(n, c, cc))
    }

    pub fn ordered_children_with_args_and_context_advanced(
        self,

        f: impl FnOnce(
            &'n N,
            Option<&'p N>,
            &'c1 C::Wrapper<'c2>,
            SetEvent,
            &mut OrderedChildCommands<R>,
        ),
    ) {
        if !self.is_hot() {
            return;
        }

        let children = self.world.get::<Children>(self.commands.id());
        let mut occ = OrderedChildCommands::<R>::new(self.commands, self.world);
        f(self.args, self.previous, self.context, self.event, &mut occ);
        occ.finish();
    }

    pub fn unordered_children_with_args_and_context_advanced(
        self,

        f: impl FnOnce(
            &'n N,
            Option<&'p N>,
            &'c1 C::Wrapper<'c2>,
            SetEvent,
            &mut UnorderedChildCommands<R>,
        ),
    ) {
        if !self.is_hot() {
            return;
        }

        let children = self.world.get::<Children>(self.commands.id());
        let mut occ = UnorderedChildCommands::<R>::new(self.commands, self.world);
        f(self.args, self.previous, self.context, self.event, &mut occ);
        occ.finish();
    }
}

impl<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, R: HierarchyRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, (), NoContext, R>
{
    pub fn ordered_children(self, f: impl FnOnce(&mut OrderedChildCommands<R>)) {
        self.ordered_children_with_args_and_context(|n, c, cc| f(cc))
    }

    pub fn unordered_children(self, f: impl FnOnce(&mut UnorderedChildCommands<R>)) {
        self.unordered_children_with_args_and_context(|n, c, cc| f(cc))
    }
}

impl<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N: PartialEq, R: HierarchyRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, N, NoContext, R>
{
    pub fn ordered_children_with_args(self, f: impl FnOnce(&'n N, &mut OrderedChildCommands<R>)) {
        self.ordered_children_with_args_and_context(|n, c, cc| f(n, cc))
    }

    pub fn unordered_children_with_args(
        self,

        f: impl FnOnce(&'n N, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_with_args_and_context(|n, c, cc| f(n, cc))
    }
}

impl<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, C: NodeContext, R: HierarchyRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, (), C, R>
{
    pub fn ordered_children_with_context(
        self,

        f: impl FnOnce(&'c1 C::Wrapper<'c2>, &mut OrderedChildCommands<R>),
    ) {
        self.ordered_children_with_args_and_context(|n, c, cc| f(c, cc))
    }

    pub fn unordered_children_with_context(
        self,

        f: impl FnOnce(&'c1 C::Wrapper<'c2>, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_with_args_and_context(|n, c, cc| f(c, cc))
    }
}
