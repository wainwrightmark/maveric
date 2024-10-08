use std::borrow::BorrowMut;

use crate::prelude::*;
use bevy::{ecs::system::StaticSystemParam, prelude::*};

pub trait CanRegisterMaveric {
    fn register_maveric<R: MavericRoot>(&mut self) -> &mut Self;
}

impl CanRegisterMaveric for App {
    fn register_maveric<R: MavericRoot>(&mut self) -> &mut Self {
        if !self.is_plugin_added::<ScheduleForDeletionPlugin>() {
            self.add_plugins(ScheduleForDeletionPlugin);
        }

        if !self.is_plugin_added::<ScheduledChangePlugin>() {
            self.add_plugins(ScheduledChangePlugin);
        }

        #[cfg(feature = "tracing")]
        {
            if !self.is_plugin_added::<crate::tracing::TracingPlugin>() {
                self.add_plugins(crate::tracing::TracingPlugin::default());
            }
        }

        #[cfg(debug_assertions)]
        {
            if !self.is_plugin_added::<CheckTransitionsPlugin>() {
                self.add_plugins(CheckTransitionsPlugin);
            }
        }

        self.add_systems(First, sync_state::<R>.run_if(should_run::<R>));
        self
    }
}

fn should_run<'w, 's, R: MavericRoot>(param: StaticSystemParam<R::Context<'w, 's>>) -> bool {
    let inner = param.into_inner();

    <R::Context<'w, 's>>::has_item_changed(&inner)
}

#[allow(clippy::needless_pass_by_value)]
fn sync_state<'w, 's, R: MavericRoot>(
    mut commands: Commands,
    param: StaticSystemParam<R::Context<'w, 's>>,
    root_query: Query<(Entity, &MavericChildComponent<R>), Without<Parent>>,
    world: &World,
    mut allocator: Local<Allocator>,
) {
    let inner = param.into_inner();

    let changed = <R::Context<'w, 's>>::has_item_changed(&inner);
    if !changed {
        return;
    }

    let allocator = allocator.borrow_mut();

    let mut root_commands = RootCommands::new(&mut commands, world, &root_query, allocator);

    R::set_children(&inner, &mut root_commands);
    root_commands.finish();

    #[cfg(feature = "tracing")]
    {
        crate::tracing::GRAPH_UPDATES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    reset_allocator(allocator);
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
        let mut state = app.world_mut().resource_mut::<TreeState>();
        *state = new_state;
    }

    fn check_leaves(app: &mut App, expected_blues: usize, expected_reds: usize) {
        let world = app.world_mut();
        let mut query = world.query::<&MavericNodeComponent<Leaf>>();

        let leaves: Vec<Leaf> = query.iter(world).map(|x| x.node.clone()).collect();
        let reds = leaves.iter().filter(|x| *x == &Leaf::Red).count();
        let blues = leaves.iter().filter(|x| *x == &Leaf::Blue).count();

        assert_eq!(reds, expected_reds);
        assert_eq!(blues, expected_blues);
    }

    #[derive(Debug, Clone, PartialEq, Eq, Resource, Default)]
    pub struct TreeState {
        branch_count: u32,
        blue_leaf_count: u32,
        red_leaf_count: u32,
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    struct Root;

    impl MavericRoot for Root {
        type Context<'w, 's> = Res<'w, TreeState>;

        fn set_children(
            context: &<Self::Context<'_, '_> as bevy::ecs::system::SystemParam>::Item<'_, '_>,
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
        type Context<'w, 's> = Res<'w, TreeState>;

        fn set_components(_commands: SetComponentCommands<Self, Self::Context<'_, '_>>) {}

        fn set_children<R: MavericRoot>(
            commands: SetChildrenCommands<Self, Self::Context<'_, '_>, R>,
        ) {
            let Some((context,mut commands)) = commands.ignore_node().ordered_children_with_context()
            else {
                return;
            };

            for x in 0..(context.blue_leaf_count) {
                commands.add_child(x, Leaf::Blue, &());
            }

            for x in (context.blue_leaf_count)..(context.blue_leaf_count + context.red_leaf_count) {
                commands.add_child(x, Leaf::Red, &());
            }
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    enum Leaf {
        Blue,
        Red,
    }

    impl MavericNode for Leaf {
        type Context<'w, 's> = ();

        fn set_components(_commands: SetComponentCommands<Self, Self::Context<'_, '_>>) {}

        fn set_children<R: MavericRoot>(
            _commands: SetChildrenCommands<Self, Self::Context<'_, '_>, R>,
        ) {
        }
    }
}
