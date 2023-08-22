use crate::prelude::*;
use bevy::prelude::*;

pub trait IntoComponents: PartialEq + Send + Sync + Sized + 'static {
    type B: Bundle;
    type Context: NodeContext;

    fn set<R: HierarchyRoot>(
        data: NodeData<Self, Self::Context, R, false>,
        commands: &mut NodeCommands,
    );

    fn with_children_same_context<C: ChildTuple<Context = Self::Context>>(
        self,
        children: C,
    ) -> NodeWithChildren<Self, C, SameContext> {
        NodeWithChildren {
            bundle: self,
            children,
            context_type: SameContext,
        }
    }

    fn with_children_different_contexts<C: ChildTuple>(
        self,
        children: C,
    ) -> NodeWithChildren<Self, C, DifferentContexts> {
        NodeWithChildren {
            bundle: self,
            children,
            context_type: DifferentContexts,
        }
    }

    fn with_no_context_children<C: ChildTuple<Context = NoContext>>(
        self,
        children: C,
    ) -> NodeWithChildren<Self, C, NoChildrenContext> {
        NodeWithChildren {
            bundle: self,
            children,
            context_type: NoChildrenContext,
        }
    }
}

pub trait NoContextIntoComponents: IntoComponents<Context = NoContext> {
    fn with_children<C: ChildTuple>(
        self,
        children: C,
    ) -> NodeWithChildren<Self, C, NoBundleContext> {
        NodeWithChildren {
            bundle: self,
            children,
            context_type: NoBundleContext,
        }
    }
}

impl<T : IntoComponents<Context = NoContext>> NoContextIntoComponents for T{}

impl<T: Bundle + PartialEq + Clone> IntoComponents for T {
    type B = Self;
    type Context = NoContext;

    fn set<R: HierarchyRoot>(
        data: NodeData<Self, Self::Context, R, false>,
        commands: &mut NodeCommands,
    ) {
        data.ignore_context()
            .insert_with_args(commands, |a| a.clone())
    }
}

impl<T: IntoComponents> HierarchyNode for T {
    type Context = T::Context;

    fn set<R: HierarchyRoot>(
        data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        Self::set(data.no_children(), commands);
    }
}
