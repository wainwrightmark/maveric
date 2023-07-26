use crate::{prelude::*, ChildDeletionPolicy};

use bevy::{
    ecs::system::{StaticSystemParam, SystemParam},
    prelude::*,
};

pub type ChildKey = u32; //TODO either

pub trait StateTreeRoot: StateTreeNode + 'static + Send + Sync + Default {
    type ContextParam: SystemParam;

    fn get_context(param: StaticSystemParam<Self::ContextParam>) -> Self::Context;
}

pub trait StateTreeNode: Eq {
    type Context: DetectChanges;

    fn get_components(
        &self,
        context: &Self::Context,
        component_commands: &mut impl ComponentCommands,
    );

    fn get_children(&self, context: &Self::Context, child_commands: &mut impl ChildCommands);

    fn get_child_deletion_policy(&self, context: &Self::Context)-> ChildDeletionPolicy;
    //fn are_children_ordered(&self) -> bool;
}

// #[derive(Debug, Component)]
// pub struct StateTreeNodeComponent<S: StateTreeSystem>{
//     phantom: PhantomData<S>,
//     pub node: S::Node
// }

// impl<S: StateTreeSystem> StateTreeNodeComponent<S> {
//     pub fn new(node: S::Node)-> Self{
//         Self{
//             phantom: Default::default(),
//             node
//         }
//     }
// }

// impl<S: StateTreeSystem> std::ops::Deref for StateTreeNodeComponent<S> {
//     type Target = S::Node;

//     fn deref(&self) -> &Self::Target {
//         &self.node
//     }
// }

// pub trait StateTreeSystem : Send + Sync + 'static {
//     type Node: StateTreeNode;
//     type Roots: Iterator<Item = Self::Node>;
//     type Children: Iterator<Item = Self::Node>;

//     fn get_roots(args: &<Self::Node as StateTreeNode>::Context) -> Self::Roots;

//     fn get_children(node: &Self::Node, args: &<Self::Node as StateTreeNode>::Context) -> Self::Children;
// }

// pub trait StateTreeNode: Eq + Debug + Sized + Hash + Send + Sync + 'static {
//     type Context: StateTreeContext;

//     fn create(&self, commands: &mut EntityCommands, context: &Self::Context);
//     fn update(
//         &self,
//         commands: &mut EntityCommands,
//         context: &Self::Context,
//         previous: &Self::Context,
//         entity_ref: EntityRef,
//     );

//     fn delete(
//         &self,
//         commands: &mut EntityCommands,
//         context: &Self::Context,
//         previous: &Self::Context,
//         entity_ref: EntityRef,
//     ) -> DeleteResult;

//     fn cancel_delete(
//         &self,
//         commands: &mut EntityCommands,
//         context: &Self::Context,
//         previous: &Self::Context,
//         entity_ref: EntityRef,
//     );

//     fn should_update(
//         &self,
//         context: &Self::Context,
//         previous: &Self::Context,
//     ) -> StateTreeShouldUpdate;
// }
