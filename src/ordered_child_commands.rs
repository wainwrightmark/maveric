use crate::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*, utils::hashbrown::HashMap};
use std::{any::type_name, marker::PhantomData};


pub(crate) struct OrderedChildCommands<'w, 's, 'a, 'q, R: HierarchyRoot> {
    ec: EntityCommands<'w, 's, 'a>,
    world: &'q World,
    phantom: PhantomData<R>,

    remaining_old_entities: HashMap<ChildKey, (usize, Entity)>,

    new_children: Vec<Entity>,
    new_indices: Vec<Option<usize>>,
}

impl<'w, 's, 'a, 'q, R: HierarchyRoot> ChildCommands for OrderedChildCommands<'w, 's, 'a, 'q, R> {
    fn add_child<NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &<NChild::Context as NodeContext>::Wrapper<'_>,
    ) {
        let key = key.into();

        if let Some((old_index, entity)) = self.remaining_old_entities.remove(&key) {
            //check if this node has changed

            if self
                .world
                .get::<HierarchyNodeComponent<NChild>>(entity)
                .is_some()
            {
                update_recursive::<R, NChild>(
                    self.ec.commands(),
                    entity,
                    child,
                    context,
                    self.world,
                );
                self.new_children.push(entity);
                self.new_indices.push(Some(old_index));
                return; //do not spawn a new child
            }
            warn!(
                "Child with key '{key}' has had node type changed to {}",
                type_name::<NChild>()
            );
            // The node type has changed - delete this entity and readd
            self.ec.commands().entity(entity).despawn_recursive();
        };

        let new_commands = self.ec.commands().spawn_empty();
        let id = create_recursive::<R, NChild>(new_commands, child, context, key, self.world);
        self.new_children.push(id);
        self.new_indices.push(None);
    }
}

impl<'w, 's, 'a, 'q, R: HierarchyRoot> OrderedChildCommands<'w, 's, 'a, 'q, R> {
    pub(crate) fn new(ec: EntityCommands<'w, 's, 'a>, world: &'q World) -> Self {
        let children = world.get::<Children>(ec.id());
        //let tree = tree.clone();
        match children {
            Some(children) => {
                let remaining_old_entities: HashMap<ChildKey, (usize, Entity)> = children
                    .iter()
                    .enumerate()
                    .flat_map(|(index, entity)| {
                        world
                            .get::<HierarchyChildComponent<R>>(*entity)
                            .map(|hcc| (hcc.key, (index, *entity)))
                    })
                    .collect();

                Self {
                    ec,
                    remaining_old_entities,
                    world,
                    phantom: PhantomData,
                    new_children: vec![],
                    new_indices: vec![],
                }
            }
            None => Self {
                ec,
                remaining_old_entities: Default::default(),
                world,
                phantom: PhantomData,
                new_children: vec![],
                new_indices: vec![],
            },
        }
    }

    pub fn finish(mut self) {
        let mut ec = self.ec;

        let order_changed = {
            let mut changed = false;
            let mut last = 0;
            'oc: for old_index in self.new_indices.iter() {
                let Some(old_index) = *old_index else{ changed = true; break 'oc;};
                if old_index < last {
                    changed = true;
                    break 'oc;
                }
                last = old_index;
            }
            changed
        };

        //remove all remaining old entities
        for (_key, (old_deleted_index, entity)) in self.remaining_old_entities {
            let Some(lingering_entity) = delete_recursive::<R>(ec.commands(), entity, self.world) else {continue;};

            if !order_changed {
                continue;
            }

            let mut closest_to_next: Option<(usize, usize)> = None;
            'inner: for (new_index, old_index) in self.new_indices.iter().enumerate() {
                let Some(old_index) = old_index else {continue;};
                let Some(distance) = old_index.checked_sub(old_deleted_index) else{continue;};

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
                self.new_indices.insert(new_index, Some(old_deleted_index));
            } else {
                self.new_children.push(lingering_entity);
                self.new_indices.push(Some(old_deleted_index));
            }
        }
        if order_changed {
            ec.replace_children(&self.new_children);
        }
    }
}

// #[cfg(test)]
// mod tests {

//     use crate::{impl_hierarchy_root, prelude::*};
//     use bevy::{time::TimePlugin, utils::HashSet};
//     #[test]
//     pub fn test_ordering() {
//         let mut app = App::new();
//         app.add_plugins(TimePlugin::default());

//         app.init_resource::<TreeState>()
//             .init_resource::<LingerState>()
//             .register_state_hierarchy::<Root>();

//         let test_states: Vec<TreeState> = vec![
//             TreeState(vec![]),
//             TreeState(vec![1, 2]),
//             TreeState(vec![1, 2, 3, 4, 5]),
//             TreeState(vec![5, 4, 3, 2, 1]),
//             TreeState(vec![1, 2, 3, 4, 5]),
//             TreeState(vec![1, 3, 5]),
//             TreeState(vec![1, 2, 3, 4, 5]),
//             TreeState(vec![5, 3, 1]),
//             TreeState(vec![1, 2, 3, 4, 5]),
//         ];

//         for (index, tree_state) in test_states.into_iter().enumerate() {
//             update_state(&mut app, tree_state.clone());
//             app.update();
//             check_leaves(&mut app, &tree_state, index);
//         }
//     }

//     #[test]
//     pub fn test_linger_same_order() {
//         test_linger(
//             TreeState(vec![1, 2, 3, 4, 5]),
//             LingerState(HashSet::from_iter([1, 2, 3, 4, 5])),
//             TreeState(vec![2, 4]),
//             vec![(1, true), (2, false), (3, true), (4, false), (5, true)],
//         );
//     }

//     #[test]
//     pub fn test_linger_new_order() {
//         test_linger(
//             TreeState(vec![1, 2, 3, 4, 5]),
//             LingerState(HashSet::from_iter([1, 2, 3, 4, 5])),
//             TreeState(vec![4, 2]),
//             vec![(3, true), (4, false), (1, true), (2, false), (5, true)],
//         );
//     }

//     #[test]
//     pub fn test_partial_linger_new_order() {
//         test_linger(
//             TreeState(vec![1, 2, 3, 4, 5]),
//             LingerState(HashSet::from_iter([2, 3, 4, 5])),
//             TreeState(vec![4, 2]),
//             vec![(3, true), (4, false), (2, false), (5, true)],
//         );
//     }

//     fn test_linger(
//         initial_tree_state: TreeState,
//         linger_state: LingerState,
//         second_tree_state: TreeState,
//         expected: Vec<(u32, bool)>,
//     ) {
//         let mut app = App::new();
//         app.add_plugins(TimePlugin::default());

//         app.insert_resource::<TreeState>(initial_tree_state)
//             .insert_resource::<LingerState>(linger_state)
//             .register_state_hierarchy::<Root>();

//         app.update();

//         update_state(&mut app, second_tree_state);
//         app.update();

//         let leaves: Vec<(u32, bool)> = get_leaves(&mut app);
//         assert_eq!(leaves, expected);
//     }

//     fn update_state(app: &mut App, new_state: TreeState) {
//         let mut state = app.world.resource_mut::<TreeState>();
//         *state = new_state;
//     }

//     fn check_leaves(app: &mut App, tree_state: &TreeState, test_index: usize) {
//         let leaves = get_leaves(app);
//         let expected: Vec<(u32, bool)> = tree_state.0.iter().map(|x| (*x, false)).collect();

//         assert_eq!(leaves, expected, "test case {test_index}");
//     }

//     fn get_leaves(app: &mut App) -> Vec<(u32, bool)> {
//         let children = app
//             .world
//             .query_filtered::<&Children, With<HierarchyNodeComponent<Branch>>>()
//             .get_single(&app.world);

//         let children = match children {
//             Ok(children) => children,
//             Err(_) => {
//                 return vec![];
//             }
//         };

//         let leaves: Vec<(u32, bool)> = children
//             .iter()
//             .map(|entity| {
//                 let number = app
//                     .world
//                     .get::<HierarchyNodeComponent<Leaf>>(*entity)
//                     .expect("Child should be a hnc Leaf")
//                     .node
//                     .number;
//                 let scheduled = app.world.get::<ScheduledForDeletion>(*entity).is_some();
//                 (number, scheduled)
//             })
//             .collect();

//         return leaves;
//     }

//     #[derive(Debug, Clone, PartialEq, Resource, Default)]
//     pub struct TreeState(Vec<u32>);

//     #[derive(Debug, Clone, PartialEq, Resource, Default)]
//     pub struct LingerState(HashSet<u32>);

//     #[derive(Debug, Clone, PartialEq, Default)]
//     struct Root;

//     impl_hierarchy_root!(Root);

//     impl HierarchyRootChildren for Root {
//         type Context = NC2<TreeState, LingerState>;

//         fn set_children<'r>(
//             context: &<Self::Context as NodeContext>::Wrapper<'r>,
//             commands: &mut impl ChildCommands,
//         ) {
//             commands.add_child("branch", Branch, context);
//         }
//     }

//     #[derive(Debug, Clone, PartialEq, Default)]
//     struct Branch;

//     impl HierarchyNode for Branch {
//         type Context = NC2<TreeState, LingerState>;

//         fn set_children<'r, R: HierarchyRoot>(
//             set_args: SetChildrenCommands<Self, Self::Context, R>,
//         ) {
//             set_args
//                 .ignore_args()
//                 .ordered_children_with_context(|context, commands| {
//                     for &number in context.0 .0.iter() {
//                         let linger = context.1 .0.contains(&number);
//                         commands.add_child(number, Leaf { number, linger }, &());
//                     }
//                 })
//         }

//         fn set_components<'r, R: HierarchyRoot>(
//             set_args: SetComponentCommands<Self, Self::Context, R>,
//         ) {
//         }
//     }

//     #[derive(Debug, Clone, PartialEq)]
//     struct Leaf {
//         number: u32,
//         linger: bool,
//     }

//     impl HierarchyNode for Leaf {
//         type Context = NoContext;

//         fn on_deleted<'r>(&self, _commands: &mut impl ComponentCommands) -> DeletionPolicy {
//             if self.linger {
//                 DeletionPolicy::linger(1.0)
//             } else {
//                 DeletionPolicy::DeleteImmediately
//             }
//         }

//         fn set_components<'r, R: HierarchyRoot>(
//             commands: SetComponentCommands<Self, Self::Context, R>,
//         ) {
//         }

//         fn set_children<'r, R: HierarchyRoot>(
//             commands: SetChildrenCommands<Self, Self::Context, R>,
//         ) {
//         }
//     }
// }
