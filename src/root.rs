use bevy::ecs::system::{ReadOnlySystemParam, StaticSystemParam};

use crate::prelude::*;

pub trait MavericRoot: MavericRootChildren {
    type ContextParam<'c>: ReadOnlySystemParam;
    fn get_context<'w>(
        param: StaticSystemParam<'w, '_, Self::ContextParam<'_>>,
    ) -> <Self::Context as NodeContext>::Wrapper<'w>;
}

pub trait MavericRootChildren: Send + Sync + 'static {
    type Context: NodeContext;

    fn set_children(
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    );
}
