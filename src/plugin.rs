use std::borrow::BorrowMut;

use crate::prelude::*;
use bevy::{ecs::system::StaticSystemParam, prelude::*};

#[derive(Debug, Default)]
struct ScheduleForRemovalPlugin;

impl Plugin for ScheduleForRemovalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, handle_scheduled_for_removal);
    }
}

pub trait CanRegisterMaveric {
    fn register_maveric<R: MavericRoot>(&mut self) -> &mut Self;
}

impl CanRegisterMaveric for App {
    fn register_maveric<R: MavericRoot>(&mut self) -> &mut Self {
        if !self.is_plugin_added::<ScheduleForRemovalPlugin>() {
            self.add_plugins(ScheduleForRemovalPlugin);
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

fn sync_state<R: MavericRoot>(
    mut commands: Commands,
    param: StaticSystemParam<R::ContextParam<'_>>,
    root_query: Query<(Entity, &MavericChildComponent<R>), Without<Parent>>,
    world: &World,
    mut allocator: Local<Allocator>,
) {
    let context = R::get_context(param);

    let changed = <R::Context as NodeContext>::has_changed(&context);
    if !changed {
        return;
    }

    let allocator = allocator.borrow_mut();

    let mut root_commands = RootCommands::new(&mut commands, world, root_query, allocator);

    R::set_children(&context, &mut root_commands);
    root_commands.finish();

    //allocator.print_info();
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use bevy::time::TimePlugin;
    #[test]
    pub fn test_plugin() {
        let mut app = App::new();

        app.add_plugins(TimePlugin);

        app.init_resource::<TreeState>().register_maveric::<Root>();
        app.update();

        check_leaves(&mut app, 0, 0);

        update_state(
            &mut app,
            TreeState {
                branch_count: 5,
                blue_leaf_count: 5,
                red_leaf_count: 0,
            },
        );
        check_leaves(&mut app, 0, 0);

        app.update();

        check_leaves(&mut app, 25, 0);

        update_state(
            &mut app,
            TreeState {
                branch_count: 5,
                blue_leaf_count: 5,
                red_leaf_count: 5,
            },
        );
        app.update();
        check_leaves(&mut app, 25, 25);

        update_state(
            &mut app,
            TreeState {
                branch_count: 4,
                blue_leaf_count: 6,
                red_leaf_count: 5,
            },
        );
        app.update();
        check_leaves(&mut app, 24, 20);
    }

    fn update_state(app: &mut App, new_state: TreeState) {
        let mut state = app.world.resource_mut::<TreeState>();
        *state = new_state;
    }

    fn check_leaves(app: &mut App, expected_blues: usize, expected_reds: usize) {
        let leaves: Vec<Leaf> = app
            .world
            .query::<&MavericNodeComponent<Leaf>>()
            .iter(&app.world)
            .map(|x| x.node.clone())
            .collect();
        let reds = leaves.iter().filter(|x| *x == &Leaf::Red).count();
        let blues = leaves.iter().filter(|x| *x == &Leaf::Blue).count();

        assert_eq!(reds, expected_reds);
        assert_eq!(blues, expected_blues);
    }

    #[derive(Debug, Clone, PartialEq, Resource, Default)]
    pub struct TreeState {
        branch_count: u32,
        blue_leaf_count: u32,
        red_leaf_count: u32,
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    struct Root;

    impl_maveric_root!(Root);

    impl MavericRootChildren for Root {
        type Context = TreeState;

        fn set_children(
            context: &<Self::Context as NodeContext>::Wrapper<'_>,
            commands: &mut impl ChildCommands,
        ) {
            for x in 0..(context.branch_count) {
                commands.add_child(x, Branch, context);
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    struct Branch;

    impl MavericNode for Branch {
        type Context = TreeState;

        fn set_components(_commands: SetComponentCommands<Self, Self::Context>) {}

        fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context, R>) {
            commands
                .ignore_args()
                .ordered_children_with_context(|context, commands| {
                    for x in 0..(context.blue_leaf_count) {
                        commands.add_child(x, Leaf::Blue, &());
                    }

                    for x in (context.blue_leaf_count)
                        ..(context.blue_leaf_count + context.red_leaf_count)
                    {
                        commands.add_child(x, Leaf::Red, &());
                    }
                });
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    enum Leaf {
        Blue,
        Red,
    }

    impl MavericNode for Leaf {
        type Context = NoContext;

        fn set_components(_commands: SetComponentCommands<Self, Self::Context>) {}

        fn set_children<R: MavericRoot>(_commands: SetChildrenCommands<Self, Self::Context, R>) {}
    }
}
