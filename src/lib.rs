use std::{rc::Rc, time::Duration};

use bevy::{
    ecs::system::{EntityCommands, StaticSystemParam},
    prelude::*,
    utils::hashbrown::HashMap,
};
use prelude::*;

pub mod child_commands;
pub mod component_commands;
pub mod components;
pub mod state_tree_context;
pub mod state_tree_node;

pub mod prelude {
    pub use crate::child_commands::*;
    pub use crate::component_commands::*;
    pub(crate) use crate::components::*;
    pub use crate::state_tree_context::*;
    pub use crate::state_tree_node::*;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildDeletionPolicy {
    DeleteImmediately,
    Linger(Duration),
}

#[derive(Debug, Default)]
pub struct StateTreePlugin;

impl Plugin for StateTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, handle_scheduled_for_removal);
    }
}

pub fn register_state_tree<R: StateTreeRoot>(app: &mut App) {
    app.add_plugins(StateTreePlugin);
    app.add_systems(Update, sync_state::<R>);
}

fn handle_scheduled_for_removal(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ScheduledForDeletion)>,
) {
    for (entity, mut schedule) in query.iter_mut() {
        schedule.timer.tick(time.delta());
        if schedule.timer.finished() {
            commands.entity(entity).despawn_recursive()
        }
    }
}

fn sync_state<R: StateTreeRoot>(
    mut commands: Commands,
    param: StaticSystemParam<R::ContextParam>,
    root_query: Query<EntityRef, With<HierarchyRoot<R>>>,
    tree: Query<(EntityRef, &HierarchyChild<R>)>,
) {
    let context = R::get_context(param);

    if !context.is_changed() {
        return;
    }

    let root_node = R::default();

    let all_child_nodes: HashMap<Entity, (EntityRef, ChildKey)> =
        tree.iter().map(|(e, c)| (e.id(), (e, c.key))).collect();

    let all_child_nodes = Rc::new(all_child_nodes);

    match root_query.get_single().ok() {
        Some(entity_ref) => {
            update_recursive::<R, R>(
                &mut commands,
                entity_ref,
                root_node,
                &context,
                all_child_nodes,
            );
        }
        None => {
            let mut ec = commands.spawn(HierarchyRoot::<R>::default());
            create_recursive::<R, R>(&mut ec, root_node, &context);
        }
    }
}

fn create_recursive<R: StateTreeRoot, N: StateTreeNode>(
    mut cec: &mut EntityCommands,
    node: N,
    context: &N::Context,
) {
    let mut creation_commands = ComponentCreationCommands::new(&mut cec);
    node.get_components(&context, &mut creation_commands);
    let mut child_commands = ChildCreationCommands::<R>::new(&mut cec);

    node.get_children(&context, &mut child_commands);

    cec.insert(HierarchyNode::new(node));
}

fn update_recursive<R: StateTreeRoot, N: StateTreeNode>(
    commands: &mut Commands,
    entity_ref: EntityRef,
    node: N,
    context: &N::Context,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef<'_>, ChildKey)>>,
) {
    let mut ec = commands.entity(entity_ref.id());
    let mut component_commands = ComponentUpdateCommands::new(entity_ref, &mut ec);
    node.get_components(&context, &mut component_commands);
    let children = entity_ref.get::<Children>();

    let mut child_commands =
        UnorderedChildCommands::<R>::new(&mut ec, children, all_child_nodes.clone());

    node.get_children(&context, &mut child_commands);
    let child_deletion_policy = node.get_child_deletion_policy(&context);
    child_commands.finish(children, child_deletion_policy);

    ec.insert(HierarchyNode::new(node));

    if entity_ref.contains::<ScheduledForDeletion>() {
        ec.remove::<ScheduledForDeletion>();
    }
}
