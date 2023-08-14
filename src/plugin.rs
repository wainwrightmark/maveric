use std::rc::Rc;

use crate::prelude::*;
use bevy::{ecs::system::StaticSystemParam, prelude::*, utils::hashbrown::HashMap};

#[derive(Debug, Default)]
struct ScheduleForRemovalPlugin;

impl Plugin for ScheduleForRemovalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, handle_scheduled_for_removal);
    }
}

pub trait CanRegisterStateHierarchy {
    fn register_state_hierarchy<R: HierarchyRoot>(&mut self) -> &mut Self;
}

impl CanRegisterStateHierarchy for App {
    fn register_state_hierarchy<R: HierarchyRoot>(&mut self)  -> &mut Self{
        if !self.is_plugin_added::<ScheduleForRemovalPlugin>() {
            self.add_plugins(ScheduleForRemovalPlugin::default());
        }

        self.add_systems(First, sync_state::<R>);

        self
    }
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
    tree: Query<(EntityRef, &HierarchyChildComponent<R>)>, //TODO just pass in all entities
) {
    let context = R::get_context(param);

    let changed = <R::Context as NodeContext>::has_changed(&context);
    if !changed {
        return;
    }

    let all_child_nodes: HashMap<Entity, (EntityRef, HierarchyChildComponent<R>)> =
        tree.iter().map(|(e, c)| (e.id(), (e, c.clone()))).collect(); //TODO pass the query directly somehow

    let all_child_nodes = Rc::new(all_child_nodes);

    let mut root_commands = RootCommands::new(&mut commands, all_child_nodes, root_query);

    R::set_children(&R::default(), &context, &mut root_commands);
    root_commands.finish();
}
