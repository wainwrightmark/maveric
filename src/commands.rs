use crate::prelude::*;
use bevy::{
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};

pub trait ChildCommands<NParent: AncestorAspect> {
    fn add_child<'c, NChild: HierarchyNode>(&mut self, key: impl Into<ChildKey>, child: NChild)
    where
        NParent: HasChild<NChild>;
}


pub trait ComponentCommands {
    fn get<T: Component>(&self) -> Option<&T>;
    fn insert<T: Bundle>(&mut self, bundle: T);
    fn remove<T: Bundle>(&mut self);
}
