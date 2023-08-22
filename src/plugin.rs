use crate::prelude::*;
use bevy::{ecs::system::StaticSystemParam, prelude::*};

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
    root_query: Query<(Entity, &HierarchyChildComponent<R>), Without<Parent>>,
    world: &World,
) {
    let context = R::get_context(param);

    let changed = <R::Context as NodeContext>::has_changed(&context);
    if !changed {
        return;
    }

    let mut root_commands = RootCommands::new(&mut commands, world, root_query);

    R::set_children(&context, &mut root_commands);
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

        app.init_resource::<TreeState>()
            .register_state_hierarchy::<Root>();
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
            .query::<&HierarchyNodeComponent<Leaf>>()
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

    impl_hierarchy_root!(Root);

    impl HierarchyRootChildren for Root {
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

    impl HierarchyNode for Branch {
        type Context = TreeState;

        fn set<R: HierarchyRoot>(
            args: NodeData<Self, Self::Context, R, true>,
            commands: &mut NodeCommands,
        ) {
            args.ignore_args()
                .ordered_children_with_context(commands, |context, commands| {
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

    impl HierarchyNode for Leaf {
        type Context = NoContext;

        fn set<R: HierarchyRoot>(
            _args: NodeData<Self, Self::Context, R, true>,
            _commands: &mut NodeCommands,
        ) {
        }
    }
}
