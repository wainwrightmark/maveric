use crate::prelude::*;

pub trait HierarchyNode: Send + Sync + Sized + PartialEq + 'static {
    type Context: NodeContext;
    const DELETER: &'static dyn Deleter = &NodeDeleter::<Self>::new();

    fn set<R: HierarchyRoot>(data: NodeData<Self, Self::Context, R, true>, commands: &mut NodeCommands);

    fn on_deleted<'r>(&self, _commands: &mut ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }
}
