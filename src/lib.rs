use std::rc::Rc;

use bevy::{
    ecs::system::{EntityCommands, StaticSystemParam},
    prelude::*,
    utils::hashbrown::HashMap,
};
use prelude::*;

pub mod child_commands;
pub mod child_deletion_policy;
pub mod child_key;
pub mod component_commands;
pub mod components;
pub mod desired_transform;
pub mod node_context;
pub mod hierarchy_node;
pub mod transition;
pub mod widgets;
pub mod hierarchy_root;

pub mod prelude {
    pub use crate::child_commands::*;
    pub use crate::component_commands::*;
    pub(crate) use crate::components::*;

    pub use crate::child_deletion_policy::*;
    pub use crate::child_key::*;
    pub use crate::desired_transform::*;
    pub use crate::node_context::*;
    pub use crate::hierarchy_node::*;
    pub use crate::hierarchy_root::*;    
    pub use crate::transition::prelude::*;
    pub use crate::widgets::prelude::*;

    pub use crate::register_state_tree;
}

#[derive(Debug, Default)]
pub struct StateTreePlugin;

impl Plugin for StateTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, handle_scheduled_for_removal);
    }
}

pub fn register_state_tree<R: HierarchyRoot>(app: &mut App) {
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

fn sync_state<'a, R: HierarchyRoot>(
    mut commands: Commands,
    param: StaticSystemParam<R::ContextParam<'a>>,
    root_query: Query<EntityRef, With<HierarchyRootComponent<R>>>,
    tree: Query<(EntityRef, &HierarchyChildComponent<R>)>,
) {
    let context = R::get_context(param);

    let changed = <R::Context as NodeContext>::has_changed(&context);
    if !changed {
        return;
    }

    let root_node = R::default();

    let all_child_nodes: HashMap<Entity, (EntityRef, HierarchyChildComponent<R>)> =
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
            let mut ec = commands.spawn(HierarchyRootComponent::<R>::default());
            create_recursive::<R, R>(&mut ec, root_node, &context);
        }
    }
}

fn create_recursive<'c, R: HierarchyRoot, N: HierarchyNode>(
    mut cec: &mut EntityCommands,
    node: N,
    context: &<N::Context as NodeContext>::Wrapper<'c>,
) {
    //info!("Creating Node {}", type_name::<N>());
    let mut commands = CreationHierarchyCommands::<R>::new(&mut cec);
    node.update(&context, &mut commands);

    cec.insert(HierarchyNodeComponent::new(node));
}

fn update_recursive<'c, R: HierarchyRoot, N: HierarchyNode>(
    commands: &mut Commands,
    entity_ref: EntityRef,
    node: N,
    context: &<N::Context as NodeContext>::Wrapper<'c>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef, HierarchyChildComponent<R>)>>,
) {
    let mut ec = commands.entity(entity_ref.id());

    let children = entity_ref.get::<Children>();

    let mut commands =
        UnorderedChildCommands::<R>::new(&mut ec, entity_ref, children, all_child_nodes.clone());

    node.update(&context, &mut commands);
    commands.finish();

    ec.insert(HierarchyNodeComponent::new(node));

    if entity_ref.contains::<ScheduledForDeletion>() {
        ec.remove::<ScheduledForDeletion>();
    }
}
