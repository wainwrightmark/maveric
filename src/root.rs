use bevy::ecs::system::{StaticSystemParam, SystemParam};

use crate::prelude::*;

pub trait MavericRoot: RootChildren {
    type ContextParam<'c> : SystemParam;
    fn get_context<'a, 'c, 'w: 'c, 's>(
        param: StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
    ) -> <Self::Context as NodeContext>::Wrapper<'c>;
}

pub trait RootChildren: Send + Sync + 'static {
    type Context: NodeContext;

    fn set_children<'r>(
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    );
}



/// Implement `Root` for a node. The node must implement `RootChildren`
#[macro_export]
macro_rules! impl_maveric_root {
    ($node: ident) => {
        impl MavericRoot for $node {
            type ContextParam<'c> = <<Self as RootChildren>::Context as NodeContext>::Wrapper<'c>;

            fn get_context<'a, 'c, 'w: 'c, 's>(
                param: bevy::ecs::system::StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
            ) -> <<Self as RootChildren>::Context as NodeContext>::Wrapper<'c> {
                param.into_inner()
            }
        }
    };
}
