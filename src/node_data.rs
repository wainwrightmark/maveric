use std::marker::PhantomData;

use crate::prelude::*;

pub struct NodeData<
    'n,
    'p,
    'c1,
    'c2,
    N: PartialEq,
    C: NodeContext,
    R: HierarchyRoot,
    const CHILDREN: bool,
> {
    args: &'n N,
    previous: Option<&'p N>,
    context: &'c1 C::Wrapper<'c2>,
    event: SetEvent,
    phantom: PhantomData<R>,
}

impl<'n, 'p, 'c1, 'c2, N: PartialEq, C: NodeContext, R: HierarchyRoot, const CHILDREN: bool>
    NodeData<'n, 'p, 'c1, 'c2, N, C, R, CHILDREN>
{
    pub(crate) fn new(
        args: &'n N,
        previous: Option<&'p N>,
        context: &'c1 C::Wrapper<'c2>,
        event: SetEvent,
    ) -> Self {
        Self {
            args,
            previous,
            context,
            event,
            phantom: Default::default(),
        }
    }

    /// Creates a clone that cannot be used to add children
    pub fn clone(&self) -> NodeData<'n, 'p, 'c1, 'c2, N, C, R, false> {
        NodeData {
            args: self.args.clone(),
            previous: self.previous.clone(),
            context: self.context.clone(),
            event: self.event.clone(),
            phantom: Default::default(),
        }
    }

    pub fn ignore_args(self) -> NodeData<'n, 'p, 'c1, 'c2, (), C, R, CHILDREN> {
        self.map_args(|_| &())
    }

    pub fn ignore_context(self) -> NodeData<'n, 'p, 'c1, 'c2, N, NoContext, R, CHILDREN> {
        self.map_context(|_| &())
    }

    pub fn map_args<N2: PartialEq>(
        self,
        map: impl Fn(&N) -> &N2,
    ) -> NodeData<'n, 'p, 'c1, 'c2, N2, C, R, CHILDREN> {
        let new_args = map(self.args);

        NodeData {
            args: new_args,
            previous: self.previous.map(map),
            context: self.context,
            event: self.event,
            phantom: self.phantom,
        }
    }

    pub fn map_context<C2: NodeContext>(
        self,
        map: impl FnOnce(&'c1 C::Wrapper<'c2>) -> &'c1 C2::Wrapper<'c2>,
    ) -> NodeData<'n, 'p, 'c1, 'c2, N, C2, R, CHILDREN> {
        let new_context = map(self.context);
        NodeData {
            args: self.args,
            previous: self.previous,
            context: new_context,
            event: self.event,
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

impl<'n, 'p, 'c1, 'c2, N: PartialEq, C: NodeContext, R: HierarchyRoot, const CHILDREN: bool>
    NodeData<'n, 'p, 'c1, 'c2, N, C, R, CHILDREN>
{
    pub fn insert_with_args_and_context<B: Bundle>(
        &mut self,
        commands: &mut NodeCommands,
        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>) -> B,
    ) {
        self.components_advanced(commands, |n, _, c, _, cc| cc.insert(f(n, c)))
    }

    pub fn components_advanced(
        &mut self,
        commands: &mut NodeCommands,

        f: impl FnOnce(&'n N, Option<&'p N>, &'c1 C::Wrapper<'c2>, SetEvent, &mut ComponentCommands),
    ) {
        if !self.is_hot() {
            return;
        }
        let mut occ = ComponentCommands::new(commands, self.event);
        f(self.args, self.previous, self.context, self.event, &mut occ);
    }
}

impl<'n, 'p, 'c1, 'c2, R: HierarchyRoot, const CHILDREN: bool>
    NodeData<'n, 'p, 'c1, 'c2, (), NoContext, R, CHILDREN>
{
    pub fn insert<B: Bundle>(&mut self, commands: &mut NodeCommands, b: B) {
        self.insert_with_args_and_context(commands, |_, _| b)
    }
}

impl<'n, 'p, 'c1, 'c2, N: PartialEq, R: HierarchyRoot, const CHILDREN: bool>
    NodeData<'n, 'p, 'c1, 'c2, N, NoContext, R, CHILDREN>
{
    pub fn insert_with_args<B: Bundle>(
        &mut self,
        commands: &mut NodeCommands,
        f: impl FnOnce(&'n N) -> B,
    ) {
        self.insert_with_args_and_context(commands, |n, _| f(n))
    }
}

impl<'n, 'p, 'c1, 'c2, C: NodeContext, R: HierarchyRoot, const CHILDREN: bool>
    NodeData<'n, 'p, 'c1, 'c2, (), C, R, CHILDREN>
{
    pub fn insert_with_context<B: Bundle>(
        &mut self,
        commands: &mut NodeCommands,
        f: impl FnOnce(&'c1 C::Wrapper<'c2>) -> B,
    ) {
        self.insert_with_args_and_context(commands, |_, c| f(c))
    }
}

impl<'n, 'p, 'c1, 'c2, N: PartialEq, C: NodeContext, R: HierarchyRoot>
    NodeData<'n, 'p, 'c1, 'c2, N, C, R, true>
{
    pub fn ordered_children_with_args_and_context(
        self,
        commands: &mut NodeCommands,
        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>, &mut OrderedChildCommands<R>),
    ) {
        self.ordered_children_with_args_and_context_advanced(commands, |n, _, c, _, cc| f(n, c, cc))
    }

    pub fn unordered_children_with_args_and_context(
        self,
        commands: &mut NodeCommands,
        f: impl FnOnce(&'n N, &'c1 C::Wrapper<'c2>, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_with_args_and_context_advanced(commands, |n, _, c, _, cc| {
            f(n, c, cc)
        })
    }

    pub fn ordered_children_with_args_and_context_advanced(
        self,
        commands: &mut NodeCommands,
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

        let mut occ = OrderedChildCommands::<R>::new(commands);
        f(self.args, self.previous, self.context, self.event, &mut occ);
        occ.finish();
    }

    pub fn unordered_children_with_args_and_context_advanced(
        self,
        commands: &mut NodeCommands,
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

        let mut occ = UnorderedChildCommands::<R>::new(commands);
        f(self.args, self.previous, self.context, self.event, &mut occ);
        occ.finish();
    }
}

impl<'n, 'p, 'c1, 'c2, R: HierarchyRoot> NodeData<'n, 'p, 'c1, 'c2, (), NoContext, R, true> {
    pub fn ordered_children(
        self,
        commands: &mut NodeCommands,
        f: impl FnOnce(&mut OrderedChildCommands<R>),
    ) {
        self.ordered_children_with_args_and_context(commands, |_, _, cc| f(cc))
    }

    pub fn unordered_children(
        self,
        commands: &mut NodeCommands,
        f: impl FnOnce(&mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_with_args_and_context(commands, |_, _, cc| f(cc))
    }
}

impl<'n, 'p, 'c1, 'c2, N: PartialEq, R: HierarchyRoot>
    NodeData<'n, 'p, 'c1, 'c2, N, NoContext, R, true>
{
    pub fn ordered_children_with_args(
        self,
        commands: &mut NodeCommands,
        f: impl FnOnce(&'n N, &mut OrderedChildCommands<R>),
    ) {
        self.ordered_children_with_args_and_context(commands, |n, _, cc| f(n, cc))
    }

    pub fn unordered_children_with_args(
        self,
        commands: &mut NodeCommands,
        f: impl FnOnce(&'n N, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_with_args_and_context(commands, |n, _, cc| f(n, cc))
    }
}

impl<'n, 'p, 'c1, 'c2, C: NodeContext, R: HierarchyRoot>
    NodeData<'n, 'p, 'c1, 'c2, (), C, R, true>
{
    pub fn ordered_children_with_context(
        self,
        commands: &mut NodeCommands,
        f: impl FnOnce(&'c1 C::Wrapper<'c2>, &mut OrderedChildCommands<R>),
    ) {
        self.ordered_children_with_args_and_context(commands, |_n, c, cc| f(c, cc))
    }

    pub fn unordered_children_with_context(
        self,
        commands: &mut NodeCommands,
        f: impl FnOnce(&'c1 C::Wrapper<'c2>, &mut UnorderedChildCommands<R>),
    ) {
        self.unordered_children_with_args_and_context(commands, |_n, c, cc| f(c, cc))
    }
}
