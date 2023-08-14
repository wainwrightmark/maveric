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
    fn register_state_hierarchy<R: HierarchyRoot>(&mut self) -> &mut Self {
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

    R::set_children(
        &R::default(),
        Some(&R::default()),
        &context,
        &mut root_commands,
    );
    root_commands.finish();
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use bevy::time::TimePlugin;
    #[test]
    pub fn test_plugin() {
        let mut app = App::new();

        app.add_plugins(TimePlugin::default());

        app.init_resource::<CounterState>()
            .register_state_hierarchy::<Root>();
        app.update();


        check_marker(&mut app, 0);

        let mut counter_res = app.world.resource_mut::<CounterState>();
        counter_res.number = 2;

        check_marker(&mut app, 0);

        app.update();

        check_marker(&mut app, 2);
    }

    fn check_marker(app: &mut App, expected: usize){
        let marker = app.world.query::<&Marker>().get_single(&app.world).unwrap();
        assert_eq!(marker.number, expected);
    }

    #[derive(Debug, Clone, PartialEq, Default, Component)]
    struct Marker{number: usize}

    #[derive(Debug, Clone, PartialEq, Resource, Default)]
    pub struct CounterState {
        number: usize,
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    struct Root;

    impl HasContext for Root {
        type Context = CounterState;
    }

    impl ChildrenAspect for Root {
        fn set_children(
            &self,
            _previous: Option<&Self>,
            context: &<Self::Context as NodeContext>::Wrapper<'_>,
            commands: &mut impl ChildCommands,
        ) {
            commands.add_child(0, Child, context);
        }
    }

    impl_hierarchy_root!(Root);

    #[derive(Debug, Clone, PartialEq, Default)]
    struct Child;

    impl HasContext for Child {
        type Context = CounterState;
    }

    impl NoChildrenAspect for Child {}

    impl ComponentsAspect for Child {
        fn set_components<'r>(
            &self,
            _previous: Option<&Self>,
            context: &<Self::Context as NodeContext>::Wrapper<'r>,
            commands: &mut impl ComponentCommands,
            _event: SetComponentsEvent,
        ) {
            let number = context.number;
            commands.insert(Marker{number});
        }
    }
}
