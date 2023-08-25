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
    N: PartialEq,
    C: NodeContext,
    R: MavericRoot,
> {
    args: &'n N,
    previous: Option<&'p N>,
    context: &'c1 C::Wrapper<'c2>,
    event: SetEvent,
    world: &'world World,
    ec: &'ec mut EntityCommands<'w, 's, 'a>,
    phantom: PhantomData<R>,
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, C: NodeContext, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C, R>
{
    pub(crate) fn new(
        args: &'n N,
        previous: Option<&'p N>,
        context: &'c1 C::Wrapper<'c2>,
        event: SetEvent,
        world: &'world World,
        ec: &'ec mut EntityCommands<'w, 's, 'a>,
    ) -> Self {
        Self {
            args,
            previous,
            context,
            event,
            world,
            ec,
            phantom: PhantomData,
        }
    }

    pub fn ignore_args(
        self,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), C, R> {
        self.map_args(|_| &())
    }

    pub fn ignore_context(
        self,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext, R> {
        self.map_context(|_| &())
    }

    pub fn map_args<N2: PartialEq>(
        self,
        map: impl Fn(&N) -> &N2,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N2, C, R> {
        let new_args = map(self.args);

        SetChildrenCommands {
            args: new_args,
            previous: self.previous.map(map),
            context: self.context,
            event: self.event,
            phantom: self.phantom,
            world: self.world,
            ec: self.ec,
        }
    }

    pub fn map_context<C2: NodeContext>(
        self,
        map: impl FnOnce(&'c1 C::Wrapper<'c2>) -> &'c1 C2::Wrapper<'c2>,
    ) -> SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C2, R> {
        let new_context = map(self.context);
        SetChildrenCommands {
            args: self.args,
            previous: self.previous,
            context: new_context,
            event: self.event,
            phantom: self.phantom,
            world: self.world,
            ec: self.ec,
        }
    }

    /// Returns true if this is a creation or undeletion, or if the context or args have changed
    fn is_hot(&self) -> bool {
        match self.event {
            SetEvent::Created | SetEvent::Undeleted => true,
            SetEvent::Updated => {
                C::has_changed(self.context)
                    || self.previous.map(|p| !p.eq(self.args)).unwrap_or(true)
            }
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
        N: ChildTuple<Context = C>,
        C: NodeContext,
        R: MavericRoot,
    > SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C, R>
{
    pub fn add_children(self) {
        self.unordered_children_with_args_and_context(|args, context, commands| {
            args.add_children(context, commands)
        })
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, C: NodeContext, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C, R>
{
    pub fn no_children(self) {}

    pub fn ordered_children_with_args_and_context(
        self,

        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>, &mut OrderedChildCommands<R>),
    ) {
        self.ordered_children_advanced(|n, _, c, _, cc| f(n, c, cc))
    }

    pub fn unordered_children_with_args_and_context(
        self,

        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_advanced(|n, _, c, _, cc| f(n, c, cc))
    }

    pub fn ordered_children_advanced(
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

        let mut occ = OrderedChildCommands::<R>::new(self.ec, self.world);
        f(self.args, self.previous, self.context, self.event, &mut occ);
        occ.finish();
    }

    pub fn unordered_children_advanced(
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

        let mut occ = UnorderedChildCommands::<R>::new(self.ec, self.world);
        f(self.args, self.previous, self.context, self.event, &mut occ);
        occ.finish();
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), NoContext, R>
{
    pub fn ordered_children(self, f: impl FnOnce(&mut OrderedChildCommands<R>)) {
        self.ordered_children_with_args_and_context(|_, _, cc| f(cc))
    }

    pub fn unordered_children(self, f: impl FnOnce(&mut UnorderedChildCommands<R>)) {
        self.unordered_children_with_args_and_context(|_, _, cc| f(cc))
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext, R>
{
    pub fn ordered_children_with_args(self, f: impl FnOnce(&'n N, &mut OrderedChildCommands<R>)) {
        self.ordered_children_with_args_and_context(|n, _, cc| f(n, cc))
    }

    pub fn unordered_children_with_args(
        self,

        f: impl FnOnce(&'n N, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_with_args_and_context(|n, _, cc| f(n, cc))
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, C: NodeContext, R: MavericRoot>
    SetChildrenCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), C, R>
{
    pub fn ordered_children_with_context(
        self,

        f: impl FnOnce(&'c1 C::Wrapper<'c2>, &mut OrderedChildCommands<R>),
    ) {
        self.ordered_children_with_args_and_context(|_n, c, cc| f(c, cc))
    }

    pub fn unordered_children_with_context(
        self,

        f: impl FnOnce(&'c1 C::Wrapper<'c2>, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_with_args_and_context(|_n, c, cc| f(c, cc))
    }
}
