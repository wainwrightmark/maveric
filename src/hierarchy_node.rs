use std::marker::PhantomData;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetComponentsEvent {
    Created,
    Updated,
    Undeleted,
}

pub trait HasChild<NChild: HierarchyNode>: AncestorAspect {
    fn convert_context<'a, 'r>(
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<NChild as NodeBase>::Context as NodeContext>::Wrapper<'r>;

    const DELETER: &'static dyn ChildDeleter<Self> = &NodeDeleter::<Self, NChild>::new();
}

pub trait NodeBase: Sized + Send + Sync + 'static {
    type Context: NodeContext;
    type Args: PartialEq + Send + Sync + 'static;
}

pub trait AncestorAspect: NodeBase {
    fn set_children<'r>(
        args: &Self::Args,
        context: &<Self::Context as NodeContext>::Wrapper<'r>, //TODO should not be wrapper
        commands: &mut impl ChildCommands<Self>,
    );
}

pub trait ComponentsAspect: NodeBase {
    fn set_components<'r>(
        args: &Self::Args,
        context: &<Self::Context as NodeContext>::Wrapper<'r>, //TODO should not be wrapper
        commands: &mut impl ComponentCommands,
        event: SetComponentsEvent,
    );

    #[allow(clippy::unused_variables)]
    fn on_deleted<'r>(
        _context: &<Self::Context as NodeContext>::Wrapper<'r>, //TODO should not be wrapper
        _commands: &mut impl ComponentCommands,
    ) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }
}

pub trait HierarchyNode: NodeBase {
    type ComponentsAspect: ComponentsAspect;
    type AncestorAspect: AncestorAspect;

    fn components_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ComponentsAspect as NodeBase>::Context as NodeContext>::Wrapper<'r>;
    fn ancestor_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::AncestorAspect as NodeBase>::Context as NodeContext>::Wrapper<'r>;

    fn component_args<'a>(
        args: &'a <Self as NodeBase>::Args,
    ) -> &'a <Self::ComponentsAspect as NodeBase>::Args;
    fn ancestor_args<'a>(
        args: &'a <Self as NodeBase>::Args,
    ) -> &'a <Self::AncestorAspect as NodeBase>::Args;
}

impl<N: NodeBase + AncestorAspect + ComponentsAspect> HierarchyNode for N {
    type ComponentsAspect = Self;

    type AncestorAspect = Self;

    fn components_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ComponentsAspect as NodeBase>::Context as NodeContext>::Wrapper<'r> {
        context
    }

    fn ancestor_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::AncestorAspect as NodeBase>::Context as NodeContext>::Wrapper<'r> {
        context
    }

    fn component_args<'a>(
        args: &'a <Self as NodeBase>::Args,
    ) -> &'a <Self::ComponentsAspect as NodeBase>::Args {
        args
    }

    fn ancestor_args<'a>(
        args: &'a <Self as NodeBase>::Args,
    ) -> &'a <Self::AncestorAspect as NodeBase>::Args {
        args
    }
}

// pub(crate) trait CanDelete {
//     const DELETER: &'static dyn Deleter;
// }

// impl<N: HierarchyNode> CanDelete for N {
//     const DELETER: &'static dyn Deleter = &NodeDeleter::<Self>::new();
// }

#[derive(Debug)]
struct NodeDeleter<NParent: AncestorAspect + HasChild<NChild>, NChild: HierarchyNode> {
    phantom: PhantomData<(NParent, NChild)>,
}

impl<NParent: AncestorAspect + HasChild<NChild>, NChild: HierarchyNode> NodeDeleter<NParent, NChild> {
    const fn new() -> Self { Self { phantom: PhantomData } }
}

impl<NParent: AncestorAspect + HasChild<NChild>, NChild: HierarchyNode> ChildDeleter<NParent>
    for NodeDeleter<NParent, NChild>
{
    fn on_deleted<'r>(
        &self,
        commands: &mut ConcreteComponentCommands,
        parent_context: &<<NParent>::Context as NodeContext>::Wrapper<'r>,
    ) -> DeletionPolicy {
        let child_context = NParent::convert_context(parent_context);

        let component_context = NChild::components_context(child_context);

        <NChild::ComponentsAspect>::on_deleted(component_context, commands)
    }
}

pub  trait ChildDeleter<NParent: AncestorAspect>: Send + Sync + 'static {
    fn on_deleted<'r>(
        &self,
        commands: &mut ConcreteComponentCommands,
        parent_context: &<NParent::Context as NodeContext>::Wrapper<'r>,
    ) -> DeletionPolicy;
}
