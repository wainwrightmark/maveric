use std::{any::type_name, marker::PhantomData, rc::Rc};

use crate::{prelude::*, DeletionPolicy};
use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};

pub trait ChildCommands<NParent: AncestorAspect>: CommandsBase {
    fn add_child<'c, NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child_args: <NChild as NodeBase>::Args,
    ) where
        NParent: HasChild<NChild>;
}

pub trait CommandsBase {
    fn get<T: Component>(&self) -> Option<&T>;
}

pub trait ComponentCommands: CommandsBase {
    fn insert<T: Bundle>(&mut self, bundle: T);
    fn remove<T: Bundle>(&mut self);
}
