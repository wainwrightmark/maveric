use crate::prelude::*;

pub trait MavericNode: Send + Sync + Sized + PartialEq + 'static {
    type Context: NodeContext;
    const DELETER: &'static dyn Deleter = &NodeDeleter::<Self>::new();

    fn set_components<R: MavericRoot>(commands: NodeCommands<Self, Self::Context, R, false>);
    fn set_children<R: MavericRoot>(commands: NodeCommands<Self, Self::Context, R, true>);

    fn on_deleted<'r>(&self, _commands: &mut ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }
}
