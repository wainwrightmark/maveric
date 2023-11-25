use bevy::ecs::system::{StaticSystemParam, ReadOnlySystemParam};

use crate::prelude::*;

pub trait MavericRoot: MavericRootChildren {
    type ContextParam<'c>: ReadOnlySystemParam;
    fn get_context<'a, 'c, 'w: 'c, 's>(
        param: StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
    ) -> <Self::Context as NodeContext>::Wrapper<'c>;
}

pub trait MavericRootChildren: Send + Sync + 'static {
    type Context: NodeContext;

    fn set_children(
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        commands: &mut impl ChildCommands,
    );
}

// /// Implement `Root` for a node. The node must implement `RootChildren`
// #[macro_export]
// macro_rules! impl_maveric_root {
//     ($node: ident) => {
//         impl $crate::prelude::MavericRoot for $node {
//             type ContextParam<'c> = <<Self as MavericRootChildren>::Context as $crate::prelude::NodeContext>::Wrapper<'c>;

//             fn get_context<'a, 'c, 'w: 'c, 's>(
//                 param: bevy::ecs::system::StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
//             ) -> <<Self as MavericRootChildren>::Context as $crate::prelude::NodeContext>::Wrapper<'c> {
//                 param.into_inner()
//             }
//         }
//     };
// }
