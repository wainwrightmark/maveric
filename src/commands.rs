use crate::prelude::*;
use bevy::prelude::*;

pub trait ChildCommands {
    fn add_child<NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &<NChild::Context as NodeContext>::Wrapper<'_>,
    );
}

pub trait ComponentCommands {
    fn get<T: Component>(&self) -> Option<&T>;
    fn insert<T: Bundle>(&mut self, bundle: T);
    fn remove<T: Bundle>(&mut self);
}
