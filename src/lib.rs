use std::{rc::Rc, time::Duration};

use bevy::{
    ecs::{query::Has, system::StaticSystemParam},
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
    tree: Query<(EntityRef, &HierarchyChild1<R>)>,
) {
    let context = R::get_context(param);

    if !context.is_changed() {
        return;
    }

    let root_node = R::default();
    let child_deletion_policy = root_node.get_child_deletion_policy(&context);

    let mut all_child_nodes: HashMap<Entity, (EntityRef, ChildKey)> = Default::default();

    for (er, child) in tree.iter() {
        all_child_nodes.insert(er.id(), (er, child.key));
    }

    let all_child_nodes = Rc::new(all_child_nodes);

    match root_query.get_single().ok() {
        Some(entity_ref) => {
            let mut ec = commands.entity(entity_ref.id());

            let mut component_commands = ComponentUpdateCommands {
                ec: &mut ec,
                entity_ref,
            };
            root_node.get_components(&context, &mut component_commands);
            let children = entity_ref.get::<Children>();

            let mut child_commands =
                UnorderedChildCommands::new(&mut ec, children, all_child_nodes);

            root_node.get_children(&context, &mut child_commands);

            child_commands.finish(children, child_deletion_policy);
        }
        None => {
            let mut ec = commands.spawn_empty();

            ec.insert((
                HierarchyRoot::<R>::default(),
                HierarchyNode1::<R>::default(),
            ));

            let mut component_commands = ComponentCreationCommands { ec: &mut ec };
            root_node.get_components(&context, &mut component_commands);

            let mut child_commands = UnorderedChildCommands::new(&mut ec, None, all_child_nodes);

            root_node.get_children(&context, &mut child_commands);

            child_commands.finish(None, child_deletion_policy);
        }
    }

    drop(tree);
}
