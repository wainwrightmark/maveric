use std::marker::PhantomData;

use bevy::ecs::system::EntityCommands;

use crate::prelude::*;
pub struct NodeCommands<
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
    const CHILDREN: bool,
> {
    args: &'n N,
    previous: Option<&'p N>,
    context: &'c1 C::Wrapper<'c2>,
    event: SetEvent,
    world: &'world World,
    ec: &'ec mut EntityCommands<'w, 's, 'a>,
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
        N: PartialEq,
        C: NodeContext,
        R: MavericRoot,
        const CHILDREN: bool,
    > NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C, R, CHILDREN>
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

    pub fn scope<'ec2, 'selfie>(
        &'selfie mut self,
        f: impl FnOnce(NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec2, 'w, 's, 'a, N, C, R, false>),
    ) where
        'n: 'ec2,
        'p: 'ec2,
        'c1: 'ec2,
        'c2: 'ec2,
        'world: 'ec2,
        'ec: 'ec2,
        'w: 'ec2,
        's: 'ec2,
        'a: 'ec2,
        'selfie: 'ec2,
    {
        let clone = NodeCommands {
            args: self.args,
            previous: self.previous,
            context: self.context,
            event: self.event,
            world: self.world,
            phantom: self.phantom,
            ec: &mut self.ec,
        };
        f(clone)
    }

    pub fn ignore_args(
        self,
    ) -> NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), C, R, CHILDREN> {
        self.map_args(|_| &())
    }

    pub fn ignore_context(
        self,
    ) -> NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext, R, CHILDREN> {
        self.map_context(|_| &())
    }

    pub fn map_args<N2: PartialEq>(
        self,
        map: impl Fn(&N) -> &N2,
    ) -> NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N2, C, R, CHILDREN> {
        let new_args = map(self.args);

        NodeCommands {
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
    ) -> NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C2, R, CHILDREN> {
        let new_context = map(self.context);
        NodeCommands {
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
            SetEvent::Created | SetEvent::Undeleted => return true,
            SetEvent::Updated => {
                C::has_changed(self.context)
                    || self.previous.map(|p| !p.eq(self.args)).unwrap_or(true)
            }
        }
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, C: NodeContext, R: MavericRoot>
    NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C, R, false>
{
    pub fn insert_with_args_and_context<B: Bundle>(
        &mut self,

        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>) -> B,
    ) {
        // if self.is_hot(){
        //     info!("insert {}", std::any::type_name::<B>());
        // }

        self.components_advanced(|n, _, c, _, cc| cc.insert(f(n, c)))
    }

    pub fn components_advanced(
        &mut self,

        f: impl FnOnce(&'n N, Option<&'p N>, &'c1 C::Wrapper<'c2>, SetEvent, &mut ComponentCommands),
    ) {
        if self.is_hot() {
            let mut occ = ComponentCommands::new(self.ec, self.world, self.event);
            f(self.args, self.previous, self.context, self.event, &mut occ);
        }
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq + IntoBundle, R: MavericRoot>
    NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext, R, false>
{
    pub fn insert_bundle(&mut self) {
        if self.is_hot() {
            //info!("insert {}", std::any::type_name::<N>());
            self.ec.insert(self.args.clone().into_bundle());
        }
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, R: MavericRoot>
    NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), NoContext, R, false>
{
    pub fn insert<B: Bundle>(&mut self, b: B) {
        self.insert_with_args_and_context(|_, _| b)
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, R: MavericRoot>
    NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext, R, false>
{
    pub fn insert_with_args<B: Bundle>(&mut self, f: impl FnOnce(&'n N) -> B) {
        self.insert_with_args_and_context(|n, _| f(n))
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, C: NodeContext, R: MavericRoot>
    NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), C, R, false>
{
    pub fn insert_with_context<B: Bundle>(&mut self, f: impl FnOnce(&'c1 C::Wrapper<'c2>) -> B) {
        self.insert_with_args_and_context(|_, c| f(c))
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
    > NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C, R, true>
{
    pub fn add_children(self) {
        self.unordered_children_with_args_and_context(|args, context, commands| {
            args.add_children(context, commands)
        })
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, C: NodeContext, R: MavericRoot>
    NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C, R, true>
{
    pub fn no_children(
        self,
    ) -> NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, C, R, false> {
        NodeCommands {
            args: self.args.clone(),
            previous: self.previous.clone(),
            context: self.context.clone(),
            event: self.event.clone(),
            phantom: Default::default(),
            world: self.world,
            ec: self.ec,
        }
    }

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
    NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), NoContext, R, true>
{
    pub fn ordered_children(self, f: impl FnOnce(&mut OrderedChildCommands<R>)) {
        self.ordered_children_with_args_and_context(|_, _, cc| f(cc))
    }

    pub fn unordered_children(self, f: impl FnOnce(&mut UnorderedChildCommands<R>)) {
        self.unordered_children_with_args_and_context(|_, _, cc| f(cc))
    }
}

impl<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N: PartialEq, R: MavericRoot>
    NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, N, NoContext, R, true>
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
    NodeCommands<'n, 'p, 'c1, 'c2, 'world, 'ec, 'w, 's, 'a, (), C, R, true>
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
