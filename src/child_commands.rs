use std::{rc::Rc, sync::Arc};

use crate::{prelude::*, ChildDeletionPolicy};

use bevy::{ecs::system::EntityCommands, prelude::*, utils::hashbrown::HashMap};

pub trait ChildCommands {
    fn ensure_child<N: StateTreeNode>(
        &mut self,
        key: ChildKey,
        child_context: N::Context,
        make_child: impl Fn() -> N,
    );
}

pub(crate) struct UnorderedChildCommands<'w, 's, 'a, 'b, 'w1> {
    ec: &'b mut EntityCommands<'w, 's, 'a>,

    new_entities: Vec<Entity>,
    remaining_old_entities: HashMap<ChildKey, EntityRef<'w1>>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, ChildKey)>>,
}

impl<'w, 's, 'a, 'b, 'w1> ChildCommands for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1> {
    fn ensure_child<N: StateTreeNode>(
        &mut self,
        key: ChildKey,
        child_context: N::Context,
        make_child: impl Fn() -> N,
    ) {
        match self.remaining_old_entities.get(&key) {
            Some(entity) => {
                // update this child if necessary
                // unschedule it for deletion if necessary
            }
            None => {
                // create this child
            }
        }
    }
}

impl<'w, 's, 'a, 'b, 'w1> UnorderedChildCommands<'w, 's, 'a, 'b, 'w1> {
    pub fn new(
        ec: &'b mut EntityCommands<'w, 's, 'a>,
        children: Option<&Children>,
        all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, ChildKey)>>,
    ) -> Self {
        //let tree = tree.clone();
        match children {
            Some(children) => {
                let mut new_entities = vec![];

                let remaining_old_entities: HashMap<ChildKey, EntityRef<'w1>> = children
                    .iter()
                    .flat_map(|x| match all_child_nodes.get(x) {
                        Some((er, key)) => Some((key.clone(), er.clone())),
                        None => {
                            new_entities.push(*x);
                            None
                        }
                    })
                    .collect();

                Self {
                    ec,
                    new_entities,
                    remaining_old_entities,
                    all_child_nodes,
                }
            }
            None => Self {
                ec,
                new_entities: vec![],
                remaining_old_entities: Default::default(),
                all_child_nodes,
            },
        }
    }

    pub fn finish(
        self,
        previous_children: Option<&Children>,
        deletion_policy: ChildDeletionPolicy,
    ) {
        let ec = self.ec;
        let mut new_child_entities = self.new_entities;

        //remove all remaining old entities
        for (_, e_ref) in self.remaining_old_entities {
            match deletion_policy {
                ChildDeletionPolicy::DeleteImmediately => {
                    //do nothing
                }
                ChildDeletionPolicy::Linger(duration) => {
                    if !e_ref.contains::<ScheduledForDeletion>() {
                        ec.commands()
                            .entity(e_ref.id())
                            .insert(ScheduledForDeletion {
                                timer: Timer::new(duration, TimerMode::Once),
                            });
                    }
                    new_child_entities.push(e_ref.id());
                }
            }
        }

        //create children

        match previous_children {
            Some(old_children) => {
                if new_child_entities.is_empty() {
                    ec.clear_children();
                } else {
                    let skip = old_children
                        .iter()
                        .zip(new_child_entities.iter())
                        .take_while(|(a, b)| a == b)
                        .count();
                    if skip == new_child_entities.len() {
                        //Do nothing
                    } else if skip > 0 {
                        ec.remove_children(&old_children[skip..]);
                        ec.push_children(&new_child_entities[skip..]);
                    } else {
                        ec.replace_children(&new_child_entities);
                    }
                }
            }
            None => {
                if !new_child_entities.is_empty() {
                    ec.push_children(&new_child_entities);
                }
            }
        }
    }
}
