use crate::prelude::*;
use std::{any::type_name, hash::BuildHasherDefault, marker::PhantomData};

use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    utils::{
        hashbrown::{hash_map::DefaultHashBuilder, HashMap},
        AHasher,
    },
};

pub trait ChildCommands {
    fn add_child<NChild: MavericNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &NChild::Context<'_, '_>,
    );

    /// Remove a child immediately if it was previously present
    fn remove_child(&mut self, key: impl Into<ChildKey>);
}

#[derive(Debug, Default)]
struct DuplicateChecker {
    #[cfg(debug_assertions)]
    set: bevy::utils::HashSet<ChildKey>,
}

impl DuplicateChecker {
    #[inline]
    #[allow(clippy::used_underscore_binding)]
    pub(crate) fn test(&mut self, _key: ChildKey) {
        #[cfg(debug_assertions)]
        {
            assert!(self.set.insert(_key), "Duplicate Child Key {_key}");
        }
    }
}

pub struct UnorderedChildCommands<'c, 'a, 'world, 'alloc, R: MavericRoot> {
    ec: &'c mut EntityCommands<'a>,
    world: &'world World,
    remaining_old_entities: HashMap<ChildKey, Entity, DefaultHashBuilder, &'alloc Allocator>,
    phantom: PhantomData<R>,
    duplicate_checker: DuplicateChecker,
}

impl<'c, 'a, 'world, 'alloc, R: MavericRoot> ChildCommands
    for UnorderedChildCommands<'c, 'a, 'world, 'alloc, R>
{
    fn remove_child(&mut self, key: impl Into<ChildKey>) {
        let key: ChildKey = key.into();

        self.duplicate_checker.test(key);

        if let Some(entity) = self.remaining_old_entities.remove(&key) {
            self.ec.commands().entity(entity).despawn_recursive();
        }
    }

    fn add_child<NChild: MavericNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &NChild::Context<'_, '_>,
    ) {
        let key = key.into();

        self.duplicate_checker.test(key);

        if let Some(entity) = self.remaining_old_entities.remove(&key) {
            //check if this node has changed

            if let Some(previous) = self.world.get::<MavericNodeComponent<NChild>>(entity) {
                if !child.should_recreate(&previous.node, context) {
                    update_recursive::<R, NChild>(
                        &mut self.ec.commands(),
                        entity,
                        child,
                        context,
                        self.world,
                        self.remaining_old_entities.allocator(),
                    );
                    return; // do not spawn a new child;
                }
            } else {
                warn!(
                    "Child with key '{key}' has had node type changed to {}",
                    type_name::<NChild>()
                );
                // The node type has changed - delete this entity and readd
            }

            self.ec.commands().entity(entity).despawn_recursive();
        }

        self.ec.with_children(|cb| {
            let cec = cb.spawn_empty();
            create_recursive::<R, NChild>(
                cec,
                child,
                context,
                key,
                self.world,
                self.remaining_old_entities.allocator(),
            );
        });
    }
}

impl<'c, 'a, 'world, 'alloc, R: MavericRoot> UnorderedChildCommands<'c, 'a, 'world, 'alloc, R> {
    pub(crate) fn new(
        ec: &'c mut EntityCommands<'a>,
        world: &'world World,
        allocator: &'alloc Allocator,
    ) -> Self {
        let children = world.get::<Children>(ec.id());
        let child_count = children.map(|x| x.len()).unwrap_or_default();
        let mut remaining_old_entities: HashMap<ChildKey, Entity,BuildHasherDefault<AHasher>, &'alloc Allocator> =
        HashMap::<ChildKey, Entity,BuildHasherDefault<AHasher>,&'alloc Allocator>::with_capacity_in(child_count, allocator);
        if let Some(children) = children {
            remaining_old_entities.extend(children.iter().filter_map(|entity| {
                world
                    .get::<MavericChildComponent<R>>(*entity)
                    .map(|hcc| (hcc.key, *entity))
            }));
        }

        Self {
            ec,
            world,
            remaining_old_entities,
            phantom: PhantomData,
            duplicate_checker: DuplicateChecker::default(),
        }
    }

    pub(crate) fn finish(self) {
        //remove all remaining old entities
        for (_key, entity) in &self.remaining_old_entities {
            let _ = delete_recursive::<R>(&mut self.ec.commands(), *entity, self.world);
        }
    }
}

pub struct OrderedChildCommands<'c, 'a, 'world, 'alloc, R: MavericRoot> {
    ec: &'c mut EntityCommands<'a>,
    world: &'world World,
    phantom: PhantomData<R>,
    remaining_old_entities:
        HashMap<ChildKey, (usize, Entity), DefaultHashBuilder, &'alloc Allocator>,
    new_children: allocator_api2::vec::Vec<Entity, &'alloc Allocator>,
    new_indices: allocator_api2::vec::Vec<Option<usize>, &'alloc Allocator>,
    duplicate_checker: DuplicateChecker,
}

impl<'c, 'a, 'world, 'alloc, R: MavericRoot> ChildCommands
    for OrderedChildCommands<'c, 'a, 'world, 'alloc, R>
{
    fn remove_child(&mut self, key: impl Into<ChildKey>) {
        let key: ChildKey = key.into();

        self.duplicate_checker.test(key);

        if let Some((_index, entity)) = self.remaining_old_entities.remove(&key) {
            self.ec.commands().entity(entity).despawn_recursive();
        }
    }

    fn add_child<NChild: MavericNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &NChild::Context<'_, '_>,
    ) {
        let key = key.into();

        self.duplicate_checker.test(key);

        if let Some((old_index, entity)) = self.remaining_old_entities.remove(&key) {
            //check if this node has changed

            if let Some(previous) = self.world.get::<MavericNodeComponent<NChild>>(entity) {
                if !child.should_recreate(&previous.node, context) {
                    update_recursive::<R, NChild>(
                        &mut self.ec.commands(),
                        entity,
                        child,
                        context,
                        self.world,
                        self.remaining_old_entities.allocator(),
                    );
                    self.new_children.push(entity);
                    self.new_indices.push(Some(old_index));
                    return; //do not spawn a new child
                }
            } else {
                warn!(
                    "Child with key '{key}' has had node type changed to {}",
                    type_name::<NChild>()
                );
                // The node type has changed - delete this entity and readd
            }

            // Delete and readd
            self.ec.commands().entity(entity).despawn_recursive();
        };
        let mut commands = self.ec.commands();

        let new_commands = commands.spawn_empty();
        let id = create_recursive::<R, NChild>(
            new_commands,
            child,
            context,
            key,
            self.world,
            self.remaining_old_entities.allocator(),
        );
        self.new_children.push(id);
        self.new_indices.push(None);
    }
}

impl<'c, 'a, 'world, 'alloc, R: MavericRoot> OrderedChildCommands<'c, 'a, 'world, 'alloc, R> {
    pub(crate) fn new(
        ec: &'c mut EntityCommands<'a>,
        world: &'world World,
        allocator: &'alloc Allocator,
    ) -> Self {
        let children = world.get::<Children>(ec.id());
        let child_count = children.map(|x| x.len()).unwrap_or_default();
        let mut remaining_old_entities: HashMap<ChildKey, (usize, Entity),BuildHasherDefault<AHasher>, &'alloc Allocator> =
        HashMap::<ChildKey, (usize, Entity),BuildHasherDefault<AHasher>,&'alloc Allocator>::with_capacity_in(child_count, allocator);

        if let Some(children) = children {
            remaining_old_entities.extend(children.iter().enumerate().filter_map(
                |(index, entity)| {
                    world
                        .get::<MavericChildComponent<R>>(*entity)
                        .map(|hcc| (hcc.key, (index, *entity)))
                },
            ));
        }

        Self {
            ec,
            world,
            remaining_old_entities,

            phantom: PhantomData,
            new_children: allocator_api2::vec::Vec::new_in(allocator),
            new_indices: allocator_api2::vec::Vec::new_in(allocator),
            duplicate_checker: DuplicateChecker::default(),
        }
    }

    pub(crate) fn finish(mut self) {
        let order_changed = {
            let mut changed = false;
            let mut last = 0;
            'oc: for old_index in &self.new_indices {
                let Some(old_index) = *old_index else {
                    changed = true;
                    break 'oc;
                };
                if old_index < last {
                    changed = true;
                    break 'oc;
                }
                last = old_index;
            }
            changed
        };

        //remove all remaining old entities
        for (_key, (old_deleted_index, entity)) in &self.remaining_old_entities {
            let Some(lingering_entity) =
                delete_recursive::<R>(&mut self.ec.commands(), *entity, self.world)
            else {
                continue;
            };

            if !order_changed {
                continue;
            }

            let mut closest_to_next: Option<(usize, usize)> = None;
            'inner: for (new_index, old_index) in self.new_indices.iter().enumerate() {
                let Some(old_index) = old_index else {
                    continue;
                };
                let Some(distance) = old_index.checked_sub(*old_deleted_index) else {
                    continue;
                };

                let replace = if let Some((prev_distance, _)) = closest_to_next {
                    distance < prev_distance
                } else {
                    true
                };

                if !replace {
                    continue;
                };
                closest_to_next = Some((distance, new_index));
                if distance == 1 {
                    break 'inner;
                };
            }

            if let Some((_, new_index)) = closest_to_next {
                self.new_children.insert(new_index, lingering_entity);
                self.new_indices.insert(new_index, Some(*old_deleted_index));
            } else {
                self.new_children.push(lingering_entity);
                self.new_indices.push(Some(*old_deleted_index));
            }
        }
        if order_changed {
            self.ec.replace_children(&self.new_children);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use bevy::{time::TimePlugin, utils::HashSet};

    #[test]
    pub fn test_ordering() {
        let mut app = App::new();
        app.add_plugins(TimePlugin);

        app.init_resource::<TreeState>()
            .init_resource::<LingerState>()
            .register_maveric::<Root>();

        let test_states: Vec<TreeState> = vec![
            TreeState(vec![]),
            TreeState(vec![1, 2]),
            TreeState(vec![1, 2, 3, 4, 5]),
            TreeState(vec![5, 4, 3, 2, 1]),
            TreeState(vec![1, 2, 3, 4, 5]),
            TreeState(vec![1, 3, 5]),
            TreeState(vec![1, 2, 3, 4, 5]),
            TreeState(vec![5, 3, 1]),
            TreeState(vec![1, 2, 3, 4, 5]),
        ];

        for (index, tree_state) in test_states.into_iter().enumerate() {
            update_state(&mut app, tree_state.clone());
            app.update();
            check_leaves(&mut app, &tree_state, index);
        }
    }

    #[test]
    pub fn test_linger_same_order() {
        test_linger(
            TreeState(vec![1, 2, 3, 4, 5]),
            LingerState(HashSet::from_iter([1, 2, 3, 4, 5])),
            TreeState(vec![2, 4]),
            vec![(1, true), (2, false), (3, true), (4, false), (5, true)],
        );
    }

    #[test]
    pub fn test_linger_new_order() {
        test_linger(
            TreeState(vec![1, 2, 3, 4, 5]),
            LingerState(HashSet::from_iter([1, 2, 3, 4, 5])),
            TreeState(vec![4, 2]),
            vec![(3, true), (4, false), (1, true), (2, false), (5, true)],
        );
    }

    #[test]
    pub fn test_partial_linger_new_order() {
        test_linger(
            TreeState(vec![1, 2, 3, 4, 5]),
            LingerState(HashSet::from_iter([2, 3, 4, 5])),
            TreeState(vec![4, 2]),
            vec![(3, true), (4, false), (2, false), (5, true)],
        );
    }

    #[test]
    #[should_panic]
    pub fn test_duplicate_key() {
        let mut app = App::new();
        app.add_plugins(TimePlugin);

        app.init_resource::<TreeState>()
            .init_resource::<LingerState>()
            .register_maveric::<Root>();

        update_state(&mut app, TreeState(vec![123, 123])); //Duplicate key should fail in debug mode
        app.update();
    }

    fn test_linger(
        initial_tree_state: TreeState,
        linger_state: LingerState,
        second_tree_state: TreeState,
        expected: Vec<(u32, bool)>,
    ) {
        let mut app = App::new();
        app.add_plugins(TimePlugin);

        app.insert_resource::<TreeState>(initial_tree_state)
            .insert_resource::<LingerState>(linger_state)
            .register_maveric::<Root>();

        app.update();

        update_state(&mut app, second_tree_state);
        app.update();

        let leaves: Vec<(u32, bool)> = get_leaves(&mut app);
        assert_eq!(leaves, expected);
    }

    fn update_state(app: &mut App, new_state: TreeState) {
        let mut state = app.world_mut().resource_mut::<TreeState>();
        *state = new_state;
    }

    fn check_leaves(app: &mut App, tree_state: &TreeState, test_index: usize) {
        let leaves = get_leaves(app);
        let expected: Vec<(u32, bool)> = tree_state.0.iter().map(|x| (*x, false)).collect();

        assert_eq!(leaves, expected, "test case {test_index}");
    }

    fn get_leaves(app: &mut App) -> Vec<(u32, bool)> {
        let children = app
            .world_mut()
            .query_filtered::<&Children, With<MavericNodeComponent<Branch>>>()
            .get_single(app.world());

        let children = match children {
            Ok(children) => children,
            Err(_) => {
                return vec![];
            }
        };

        let leaves: Vec<(u32, bool)> = children
            .iter()
            .map(|entity| {
                let number = app
                    .world()
                    .get::<MavericNodeComponent<Leaf>>(*entity)
                    .expect("Child should be a hnc Leaf")
                    .node
                    .number;
                let scheduled = app.world().get::<ScheduledForDeletion>(*entity).is_some();
                (number, scheduled)
            })
            .collect();

        leaves
    }

    #[derive(Debug, Clone, PartialEq, Eq, Resource, Default)]
    pub struct TreeState(Vec<u32>);

    #[derive(Debug, Clone, PartialEq, Eq, Resource, Default)]
    pub struct LingerState(HashSet<u32>);

    #[derive(Debug, Clone, PartialEq, Default)]
    struct Root;

    impl MavericRoot for Root {
        type Context<'w, 's> = (Res<'w, TreeState>, Res<'w, LingerState>);

        fn set_children(context: &Self::Context<'_, '_>, commands: &mut impl ChildCommands) {
            commands.add_child("branch", Branch, context);
        }
    }

    #[derive(Debug, Clone, PartialEq, Default)]
    struct Branch;

    impl MavericNode for Branch {
        type Context<'w, 's> = (Res<'w, TreeState>, Res<'w, LingerState>);

        fn set_components(_commands: SetComponentCommands<Self, Self::Context<'_, '_>>) {}

        fn set_children<R: MavericRoot>(
            commands: SetChildrenCommands<Self, Self::Context<'_, '_>, R>,
        ) {
            commands
                .ignore_node()
                .ordered_children_with_context(|context, commands| {
                    for &number in &context.0 .0 {
                        let linger = context.1 .0.contains(&number);
                        commands.add_child(number, Leaf { number, linger }, &());
                    }
                });
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Leaf {
        number: u32,
        linger: bool,
    }

    impl MavericNode for Leaf {
        type Context<'w, 's> = ();

        fn on_deleted<'r>(&self, _commands: &mut ComponentCommands) -> DeletionPolicy {
            if self.linger {
                DeletionPolicy::linger(1.0)
            } else {
                DeletionPolicy::DeleteImmediately
            }
        }

        fn set_components(_commands: SetComponentCommands<Self, Self::Context<'_, '_>>) {}

        fn set_children<R: MavericRoot>(
            _commands: SetChildrenCommands<Self, Self::Context<'_, '_>, R>,
        ) {
        }
    }
}
