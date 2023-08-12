use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetComponentsEvent {
    Created,
    Updated,
    Undeleted,
}

pub trait HasContext: PartialEq + Sized + Send + Sync + 'static {
    type Context: NodeContext;
}

pub trait ChildrenAspect: HasContext {
    fn set_children<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    );
}

pub trait ComponentsAspect: HasContext {
    fn set_components<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        event: SetComponentsEvent,
    );

    fn on_deleted<'r>(&self, _commands: &mut impl ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }
}

pub trait HasComponentsAspect: HasContext {
    type ComponentsAspect: ComponentsAspect;

    fn components_context<'a, 'r>(
        context: &'a <<Self as HasContext>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ComponentsAspect as HasContext>::Context as NodeContext>::Wrapper<'r>;

    fn as_component_aspect<'a>(&'a self) -> &'a Self::ComponentsAspect;
}

pub trait HasChildrenAspect: HasContext {
    type ChildrenAspect: ChildrenAspect;

    fn children_context<'a, 'r>(
        context: &'a <<Self as HasContext>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ChildrenAspect as HasContext>::Context as NodeContext>::Wrapper<'r>;

    fn as_children_aspect<'a>(&'a self) -> &'a Self::ChildrenAspect;
}

pub trait HierarchyNode: HasChildrenAspect + HasComponentsAspect {
    const DELETER: &'static dyn Deleter = &NodeDeleter::<Self>::new();
}

impl<N: ChildrenAspect> HasChildrenAspect for N {
    type ChildrenAspect = Self;
    fn children_context<'a, 'r>(
        context: &'a <<Self as HasContext>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ChildrenAspect as HasContext>::Context as NodeContext>::Wrapper<'r> {
        context
    }

    fn as_children_aspect<'a>(&'a self) -> &'a Self::ChildrenAspect {
        self
    }
}

impl<N: ComponentsAspect> HasComponentsAspect for N {
    type ComponentsAspect = Self;

    fn components_context<'a, 'r>(
        context: &'a <<Self as HasContext>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ComponentsAspect as HasContext>::Context as NodeContext>::Wrapper<'r> {
        context
    }

    fn as_component_aspect<'a>(&'a self) -> &'a Self::ComponentsAspect {
        self
    }
}

impl<N: HasChildrenAspect + HasComponentsAspect> HierarchyNode for N {}

impl HasContext for () {
    type Context = NoContext;
}

impl<T: NoChildrenAspect + HasContext> ChildrenAspect for T {
    fn set_children<'r>(
        &self,
        _context: &<Self::Context as NodeContext>::Wrapper<'r>,
        _commands: &mut impl ChildCommands,
    ) {
    }
}

pub trait NoChildrenAspect {}

impl NoChildrenAspect for () {}
