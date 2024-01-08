use crate::prelude::*;

#[derive(Debug)]
pub struct NodeArgs<'n, 'p, 'c1, 'c2, N: PartialEq, C: NodeContext> {
    pub context: &'c1 C::Wrapper<'c2>,
    pub event: SetEvent,
    pub node: &'n N,
    pub previous: Option<&'p N>,
}

impl<'n, 'p, 'c1, 'c2, N: PartialEq, C: NodeContext> Clone for NodeArgs<'n, 'p, 'c1, 'c2, N, C> {
    fn clone(&self) -> Self {
        Self {
            context: self.context,
            event: self.event,
            node: self.node,
            previous: self.previous,
        }
    }
}

impl<'n, 'p, 'c1, 'c2, N: PartialEq, C: NodeContext> NodeArgs<'n, 'p, 'c1, 'c2, N, C> {
    pub(crate) const fn new(
        context: &'c1 C::Wrapper<'c2>,
        event: SetEvent,
        node: &'n N,
        previous: Option<&'p N>,
    ) -> Self {
        Self {
            context,
            event,
            node,
            previous,
        }
    }

    /// Returns true if this is a creation or undeletion, or if the context or args have changed
    #[must_use]
    pub fn is_hot(&self) -> bool {
        match self.event {
            SetEvent::Created | SetEvent::Undeleted => true,
            SetEvent::Updated => {
                C::has_changed(self.context) || self.previous.map_or(true, |p| !p.eq(self.node))
            }
        }
    }

    pub fn map_node<N2: PartialEq>(
        self,
        map: impl Fn(&N) -> &N2,
    ) -> NodeArgs<'n, 'p, 'c1, 'c2, N2, C> {
        NodeArgs {
            node: map(self.node),
            previous: self.previous.map(map),
            context: self.context,
            event: self.event,
        }
    }

    pub fn map_context<C2: NodeContext>(
        self,
        map: impl FnOnce(&'c1 C::Wrapper<'c2>) -> &'c1 C2::Wrapper<'c2>,
    ) -> NodeArgs<'n, 'p, 'c1, 'c2, N, C2> {
        NodeArgs {
            node: self.node,
            previous: self.previous,
            context: map(self.context),
            event: self.event,
        }
    }
}
