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
pub mod hierarchy_node;
pub mod hierarchy_root;
pub mod node_context;
pub mod transition;
pub mod widgets;

pub mod prelude {
    pub use crate::child_commands::*;
    pub use crate::component_commands::*;
    pub(crate) use crate::components::*;

    pub use crate::child_deletion_policy::*;
    pub use crate::child_key::*;
    pub use crate::desired_transform::*;
    pub use crate::hierarchy_node::*;
    pub use crate::hierarchy_root::*;
    pub use crate::node_context::*;
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
    root_query: Query<Entity, (Without<Parent>, With<HierarchyChildComponent<R>>)>,
    tree: Query<(EntityRef, &HierarchyChildComponent<R>)>,
) {
    let context = R::get_context(param);

    let changed = <R::Context as NodeContext>::has_changed(&context);
    if !changed {
        return;
    }

    let all_child_nodes: HashMap<Entity, (EntityRef, HierarchyChildComponent<R>)> =
        tree.iter().map(|(e, c)| (e.id(), (e, c.clone()))).collect();

    let all_child_nodes = Rc::new(all_child_nodes);

    let mut root_commands =
        RootCommands::new(&mut commands, all_child_nodes, root_query);

    R::set_children(&(),&context, &mut root_commands);
    root_commands.finish();
}

fn create_recursive<'c, R: HierarchyRoot, N: HierarchyNode>(
    mut cec: &mut EntityCommands,
    args: <N as NodeBase>::Args,
    context: &<<N as NodeBase>::Context as NodeContext>::Wrapper<'c>,
) {
    //info!("Creating Node {}", type_name::<N>());
    let mut commands = CreationCommands::<R>::new(&mut cec);

    let ancestor_context = N::ancestor_context(context);
    let component_context = N::components_context(context);

    let ancestor_args = N::ancestor_aspect(&args);
    let component_args = N::component_args(&args);

    <N::ComponentsAspect as ComponentsAspect>::set_components(
        component_args,
        &component_context,
        &mut commands,
        SetComponentsEvent::Created,
    );
    <N::AncestorAspect as AncestorAspect>::set_children(
        ancestor_args,
        &ancestor_context,
        &mut commands,
    );

    cec.insert(HierarchyNodeComponent::<N> { args });
}

fn update_recursive<'c, R: HierarchyRoot, N: HierarchyNode>(
    commands: &mut Commands,
    entity_ref: EntityRef,
    args: <N as NodeBase>::Args,
    context: &<<N as NodeBase>::Context as NodeContext>::Wrapper<'c>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef, HierarchyChildComponent<R>)>>,
    undeleted: bool,
) {
    let mut ec = commands.entity(entity_ref.id());

    let children = entity_ref.get::<Children>();

    let mut commands =
        UnorderedChildCommands::<R>::new(&mut ec, entity_ref, children, all_child_nodes.clone());

    //todo check for changes

    let ancestor_context = N::ancestor_context(context);
    let component_context = N::components_context(context);

    let ancestor_args = N::ancestor_aspect(&args);
    let component_args = N::component_args(&args);

    <N::ComponentsAspect as ComponentsAspect>::set_components(
        component_args,
        &component_context,
        &mut commands,
        if undeleted {
            SetComponentsEvent::Undeleted
        } else {
            SetComponentsEvent::Updated
        },
    );
    <N::AncestorAspect as AncestorAspect>::set_children(
        ancestor_args,
        &ancestor_context,
        &mut commands,
    );

    commands.finish();

    ec.insert(HierarchyNodeComponent::<N> { args });

    if entity_ref.contains::<ScheduledForDeletion>() {
        ec.remove::<ScheduledForDeletion>();
    }
}
