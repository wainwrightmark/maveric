use bevy::ecs::system::{ReadOnlySystemParam, StaticSystemParam};

use crate::prelude::*;

pub trait MavericRoot: MavericRootChildren {
    type ContextParam<'c>: ReadOnlySystemParam;
    fn get_context<'a, 'w, 's>(
        param: StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
    ) -> <Self::Context as NodeContext>::Wrapper<'w>;
}

pub trait MavericRootChildren: Send + Sync + 'static {
    type Context: NodeContext;

    fn set_children(
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    );
}
