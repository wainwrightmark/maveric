use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetComponentsEvent {
    Created,
    Updated,
    Undeleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChildrenType {
    #[default]
    Ordered,
    Unordered,
}

pub trait HierarchyNode : Send + Sync + Sized + PartialEq + 'static {
    type Context: NodeContext;
    const DELETER: &'static dyn Deleter = &NodeDeleter::<Self>::new();

    fn set<'r>(
        &self,
        previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl NodeCommands,
        event: SetComponentsEvent,
    );

    fn on_deleted<'r>(&self, _commands: &mut impl ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }

    // fn set_children<'r>(
    //     &self,
    //     previous: Option<&Self>,
    //     context: &<Self::Context as NodeContext>::Wrapper<'r>,
    //     commands: &mut impl ChildCommands,
    // );

    const CHILDREN_TYPE: ChildrenType = ChildrenType::Ordered;
}
