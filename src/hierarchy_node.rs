use crate::prelude::*;

pub trait HierarchyNode: Send + Sync + Sized + PartialEq + 'static {
    type Context: NodeContext;
    const DELETER: &'static dyn Deleter = &NodeDeleter::<Self>::new();

    fn set_components<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world,>(commands: SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world, Self, Self::Context>)-> SetComponentsFinishToken<'w,'s,'a,'world>;

    fn set_children< R: HierarchyRoot>(
        commands: SetChildrenCommands<Self, Self::Context, R>
    );

    fn on_deleted<'r>(&self, _commands: &mut impl ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetEvent {
    Created,
    Undeleted,
    Updated,
}
