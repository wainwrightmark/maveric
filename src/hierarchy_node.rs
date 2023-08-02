use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetComponentsEvent {
    Created,
    Updated,
    Undeleted,
}

pub trait NodeBase: Send + Sync + 'static {
    type Context: NodeContext;
    type Args: PartialEq + Send + Sync + 'static;
}

pub trait AncestorAspect: NodeBase {
    fn set_children<'r>(
        args: &Self::Args,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    );
}

pub trait ComponentsAspect: NodeBase {
    fn set_components<'r>(
        args: &Self::Args,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        event: SetComponentsEvent,
    );
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
    fn ancestor_aspect<'a>(
        args: &'a <Self as NodeBase>::Args,
    ) -> &'a <Self::AncestorAspect as NodeBase>::Args;

    //TODO components_changed and children_changed

    // fn on_undeleted<'r>(
    //     &self,
    //     _context: &<Self::Context as NodeContext>::Wrapper<'r>,
    //     _commands: &mut impl ComponentCommands,
    // ) {
    //     // do nothing by default
    // }

    // #[allow(clippy::unused_variables)]
    // fn on_deleted(
    //     &self,
    //     _commands: &mut impl ComponentCommands,
    //     _new_sibling_keys: &HashSet<ChildKey>,
    // ) -> DeletionPolicy {
    //     DeletionPolicy::DeleteImmediately
    // }
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

    fn ancestor_aspect<'a>(
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

// struct NodeDeleter<N: HierarchyNode> {
//     phantom: PhantomData<N>,
// }

// impl<N: HierarchyNode> NodeDeleter<N> {
//     pub const fn new() -> Self {
//         Self {
//             phantom: PhantomData,
//         }
//     }
// }

// impl<N: HierarchyNode> Deleter for NodeDeleter<N> {
//     fn on_deleted(
//         &self,
//         component_commands: &mut ConcreteComponentCommands,
//         new_sibling_keys: &HashSet<ChildKey>,
//     ) -> DeletionPolicy {
//         if let Some(node) = component_commands
//             .entity_ref
//             .get::<HierarchyNodeComponent<N>>()
//         {
//             node.node.on_deleted(component_commands, new_sibling_keys)
//         } else {
//             DeletionPolicy::DeleteImmediately
//         }
//     }
// }

// pub(crate) trait Deleter: Send + Sync + 'static {
//     fn on_deleted(
//         &self,
//         component_commands: &mut ConcreteComponentCommands,
//         new_sibling_keys: &HashSet<ChildKey>,
//     ) -> DeletionPolicy;
// }
