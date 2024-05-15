use bevy::ecs::system::{ReadOnlySystemParam, StaticSystemParam};

use crate::prelude::*;

pub trait MavericRoot: MavericRootChildren {
    type ContextParam<'w, 's>: ReadOnlySystemParam;
    fn get_context<'w, 's>(
        param: StaticSystemParam<'w, 's, Self::ContextParam<'_, '_>>,
    ) -> <Self::Context as NodeContext>::Wrapper<'w,'s>;
}

pub trait MavericRootChildren: Send + Sync + 'static {
    type Context: NodeContext;

    fn set_children(
        context: &<Self::Context as NodeContext>::Wrapper<'_,'_>,
        commands: &mut impl ChildCommands,
    );
}
