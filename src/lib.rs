use std::{rc::Rc, any::type_name};

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
pub mod has_detect_changes;
pub mod child_deletion_policy;
pub mod child_key;
pub mod desired_transform;
pub mod widgets;
pub mod transition;

pub mod prelude {
    pub use crate::child_commands::*;
    pub use crate::component_commands::*;
    pub(crate) use crate::components::*;
    pub use crate::state_tree_context::*;
    pub use crate::state_tree_node::*;
    pub use crate::has_detect_changes::*;
    pub use crate::child_deletion_policy::*;
    pub use crate::child_key::*;
    pub use crate::desired_transform::*;
    pub use crate::widgets::*;
    pub use crate::transition::*;
}



#[derive(Debug, Default)]
pub struct StateTreePlugin;

impl Plugin for StateTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, handle_scheduled_for_removal);
    }
}

pub fn register_state_tree<R: StateTreeRoot>(app: &mut App) {
    app.add_plugins(StateTreePlugin);
    app.add_systems(First, sync_state::<R>);
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

fn sync_state<'a, R: StateTreeRoot>(
    mut commands: Commands,
    param: StaticSystemParam<R::ContextParam<'a>>,
    root_query: Query<EntityRef, With<HierarchyRoot<R>>>,
    tree: Query<(EntityRef, &HierarchyChild<R>)>,
) {
    let context = R::get_context(param);

    if !context.has_changed() {
        return;
    }

    let root_node = R::default();

    let all_child_nodes: HashMap<Entity, (EntityRef, HierarchyChild<R>)> =
        tree.iter().map(|(e, c)| (e.id(), (e, c.clone()))).collect();

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

fn create_recursive<'c, R: StateTreeRoot, N: StateTreeNode>(
    mut cec: &mut EntityCommands,
    node: N,
    context: &N::Context<'c>,
) {

    //info!("Creating Node {}", type_name::<N>());
    let mut creation_commands = ComponentCreationCommands::new(&mut cec);
    node.get_components(&context, &mut creation_commands);
    let mut child_commands = ChildCreationCommands::<R>::new(&mut cec);

    node.get_children(&context, &mut child_commands);

    cec.insert(HierarchyNode::new(node));
}

fn update_recursive<'c,R: StateTreeRoot, N: StateTreeNode>(
    commands: &mut Commands,
    entity_ref: EntityRef,
    node: N,
    context: &N::Context<'c>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef, HierarchyChild<R>)>>,
) {
    let mut ec = commands.entity(entity_ref.id());
    let mut component_commands = ComponentUpdateCommands::new(entity_ref, &mut ec);
    node.get_components(&context, &mut component_commands);
    let children = entity_ref.get::<Children>();

    let mut child_commands: UnorderedChildCommands<'_, '_, '_, '_, '_, R> =
        UnorderedChildCommands::<R>::new(&mut ec, children, all_child_nodes.clone());

    node.get_children(&context, &mut child_commands);
    child_commands.finish();

    ec.insert(HierarchyNode::new(node));

    if entity_ref.contains::<ScheduledForDeletion>() {
        ec.remove::<ScheduledForDeletion>();
    }
}
