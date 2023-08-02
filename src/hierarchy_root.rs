use std::marker::PhantomData;

use crate::{prelude::*, DeletionPolicy};
use bevy::{
    ecs::system::{StaticSystemParam, SystemParam},
    utils::HashSet,
};

pub trait HierarchyRoot: ChildrenAspect + Default + Send + Sync + 'static {
    type ContextParam<'c>: SystemParam;

    fn get_context<'a, 'c, 'w: 'c, 's>(
        param: StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
    ) -> <<Self as NodeBase>::Context as NodeContext>::Wrapper<'c>;
}

#[macro_export]
macro_rules! impl_hierarchy_root {
    ($node: ident) => {
        impl HierarchyRoot for $node {
            type ContextParam<'c> = <<Self as NodeBase>::Context as NodeContext>::Wrapper<'c>;

            fn get_context<'a, 'c, 'w: 'c, 's>(
                param: bevy::ecs::system::StaticSystemParam<'w, 's, Self::ContextParam<'a>>,
            ) -> <<Self as NodeBase>::Context as NodeContext>::Wrapper<'c> {
                param.into_inner()
            }
        }
    };
}
