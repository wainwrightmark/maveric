use bevy::ecs::system::{StaticSystemParam, SystemParam};

use crate::prelude::*;

pub trait HierarchyRoot: HierarchyRootChildren {
    type ContextParam<'c> : SystemParam;
    fn get_context<'a, 'c, 'w: 'c, 's>(
        param: StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
    ) -> <Self::Context as NodeContext>::Wrapper<'c>;
}

pub trait HierarchyRootChildren: Send + Sync + 'static {
    type Context: NodeContext;

    fn set_children<'r>(
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    );
}



/// Implement HasContextParam for a node. The node must implement `HierarchyRoot`
#[macro_export]
macro_rules! impl_hierarchy_root {
    ($node: ident) => {
        impl HierarchyRoot for $node {
            type ContextParam<'c> = <<Self as HierarchyRootChildren>::Context as NodeContext>::Wrapper<'c>;

            fn get_context<'a, 'c, 'w: 'c, 's>(
                param: bevy::ecs::system::StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
            ) -> <<Self as HierarchyRootChildren>::Context as NodeContext>::Wrapper<'c> {
                param.into_inner()
            }
        }
    };
}
