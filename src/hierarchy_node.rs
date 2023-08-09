use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetComponentsEvent {
    Created,
    Updated,
    Undeleted,
}

pub trait NodeBase: PartialEq + Sized + Send + Sync + 'static {
    type Context: NodeContext;
}

pub trait ChildrenAspect: NodeBase {
    fn set_children<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands<Self>,
    );
}

pub trait ComponentsAspect: NodeBase {
    fn set_components<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        event: SetComponentsEvent,
    );

    #[allow(clippy::unused_variables)]
    fn on_deleted<'r>(
        &self,
        _context: &<Self::Context as NodeContext>::Wrapper<'r>,
        _commands: &mut impl ComponentCommands,
    ) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }
}

pub trait HasComponentsAspect: NodeBase {
    type ComponentsAspect: ComponentsAspect;

    fn components_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ComponentsAspect as NodeBase>::Context as NodeContext>::Wrapper<'r>;

    fn as_component_aspect<'a>(&'a self) -> &'a Self::ComponentsAspect;
}

pub trait HasChildrenAspect: NodeBase {
    type ChildrenAspect: ChildrenAspect;

    fn children_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ChildrenAspect as NodeBase>::Context as NodeContext>::Wrapper<'r>;

    fn as_children_aspect<'a>(&'a self) -> &'a Self::ChildrenAspect;
}

pub trait HierarchyNode: HasChildrenAspect + HasComponentsAspect {}

impl<N: ChildrenAspect> HasChildrenAspect for N {
    type ChildrenAspect = Self;
    fn children_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ChildrenAspect as NodeBase>::Context as NodeContext>::Wrapper<'r> {
        context
    }

    fn as_children_aspect<'a>(&'a self) -> &'a Self::ChildrenAspect {
        self
    }
}

impl<N: ComponentsAspect> HasComponentsAspect for N {
    type ComponentsAspect = Self;

    fn components_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ComponentsAspect as NodeBase>::Context as NodeContext>::Wrapper<'r> {
        context
    }

    fn as_component_aspect<'a>(&'a self) -> &'a Self::ComponentsAspect {
        self
    }
}

impl<N: HasChildrenAspect + HasComponentsAspect> HierarchyNode for N {}

pub trait ChildDeleter<NParent: ChildrenAspect>: Send + Sync + 'static {
    fn on_deleted<'r>(
        &self,
        entity_ref: EntityRef,
        commands: &mut ConcreteComponentCommands,
        parent_context: &<NParent::Context as NodeContext>::Wrapper<'r>,
    ) -> DeletionPolicy;
}

impl NodeBase for () {
    type Context = NoContext;
}

impl ChildrenAspect for () {
    fn set_children<'r>(
        &self,
        _context: &<Self::Context as NodeContext>::Wrapper<'r>,
        _commands: &mut impl ChildCommands<Self>,
    ) {
    }
}
