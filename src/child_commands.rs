use std::{marker::PhantomData, rc::Rc};

use crate::{create_recursive, prelude::*, update_recursive, ChildDeletionPolicy};

use bevy::{ecs::system::EntityCommands, prelude::*, utils::hashbrown::HashMap};

pub trait ChildCommands {
    fn add<'c, N: StateTreeNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child_context: &N::Context<'c>,
        child_node: N,
    );
}

//todo ordered child commands

pub(crate) struct ChildCreationCommands<'w, 's, 'a, 'b, R: StateTreeRoot> {
    ec: &'b mut EntityCommands<'w, 's, 'a>,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'a, 'b, R: StateTreeRoot> ChildCommands for ChildCreationCommands<'w, 's, 'a, 'b, R> {
    fn add<'c, N: StateTreeNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child_context: &N::Context<'c>,
        child_node: N,
    ) {
        self.ec.with_children(|cb| {
            let mut cec = cb.spawn(HierarchyChild::<R>::new(key.into()));
            create_recursive::<R, N>(&mut cec, child_node, &child_context);
        });
    }
}

impl<'w, 's, 'a, 'b, R: StateTreeRoot> ChildCreationCommands<'w, 's, 'a, 'b, R> {
    pub(crate) fn new(ec: &'b mut EntityCommands<'w, 's, 'a>) -> Self {
        Self {
            ec,
            phantom: PhantomData,
        }
    }
}

pub(crate) struct UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, R: StateTreeRoot> {
    ec: &'b mut EntityCommands<'w, 's, 'a>,
    //new_child_entities: Vec<Entity>,
    remaining_old_entities: HashMap<ChildKey, EntityRef<'w1>>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, ChildKey)>>,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'a, 'b, 'w1, R: StateTreeRoot> ChildCommands
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, R>
{
    fn add<'c, N: StateTreeNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child_context: &N::Context<'c>,
        child_node: N,
    ) {
        let key = key.into();
        match self.remaining_old_entities.remove(&key) {
            Some(entity_ref) => {
                //check if this node has changed

                match entity_ref.get::<HierarchyNode<N>>() {
                    Some(existing) => {
                        // unschedule it for deletion if necessary

                        if child_context.has_changed() || existing.node != child_node {
                            //state has changed
                            info!("Child with key '{key}' has changed");

                            update_recursive::<R, N>(
                                &mut self.ec.commands(),
                                entity_ref.clone(),
                                child_node,
                                child_context,
                                self.all_child_nodes.clone(),
                            );
                        } else {
                            //state has not changed - do nothing
                            if entity_ref.contains::<ScheduledForDeletion>() {
                                self.ec
                                    .commands()
                                    .entity(entity_ref.id())
                                    .remove::<ScheduledForDeletion>();
                            }
                        }
                    }
                    None => {
                        warn!("Child with key '{key}' has had node type changed");
                        // The node type has changed - delete this entity and readd
                        self.ec
                            .commands()
                            .entity(entity_ref.id())
                            .despawn_recursive();

                        self.ec.with_children(|cb| {
                            let mut cec = cb.spawn(HierarchyChild::<R>::new(key));
                            create_recursive::<R, N>(&mut cec, child_node, &child_context);
                        });
                    }
                }
            }
            None => {
                self.ec.with_children(|cb| {
                    let mut cec = cb.spawn(HierarchyChild::<R>::new(key));
                    create_recursive::<R, N>(&mut cec, child_node, &child_context);
                });
            }
        }
    }
}

impl<'w, 's, 'a, 'b, 'w1, R: StateTreeRoot> UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, R> {
    pub fn new(
        ec: &'b mut EntityCommands<'w, 's, 'a>,
        children: Option<&Children>,
        all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, ChildKey)>>,
    ) -> Self {
        //let tree = tree.clone();
        match children {
            Some(children) => {
                let remaining_old_entities: HashMap<ChildKey, EntityRef<'w1>> = children
                    .iter()
                    .flat_map(|x| match all_child_nodes.get(x) {
                        Some((er, key)) => Some((key.clone(), er.clone())),
                        None => {
                            //new_entities.push(*x);
                            None
                        }
                    })
                    .collect();

                Self {
                    ec,
                    remaining_old_entities,
                    all_child_nodes,
                    phantom: PhantomData,
                }
            }
            None => Self {
                ec,
                remaining_old_entities: Default::default(),
                all_child_nodes,
                phantom: PhantomData,
            },
        }
    }

    pub fn finish(self, deletion_policy: ChildDeletionPolicy) {
        let ec = self.ec;

        //remove all remaining old entities
        for (key, e_ref) in self.remaining_old_entities {
            match deletion_policy {
                ChildDeletionPolicy::DeleteImmediately => {
                    info!("Despawning Child with key '{key}'");
                    //do nothing
                    ec.commands().entity(e_ref.id()).despawn_recursive();
                }
                ChildDeletionPolicy::Linger(duration) => {
                    if !e_ref.contains::<ScheduledForDeletion>() {
                        info!("Scheduling deletion of Child with key '{key}'");
                        ec.commands()
                            .entity(e_ref.id())
                            .insert(ScheduledForDeletion {
                                timer: Timer::new(duration, TimerMode::Once),
                            });
                    }
                    //new_child_entities.push(e_ref.id());
                }
            }
        }

        //create children

        // match previous_children {
        //     Some(old_children) => {
        //         if new_child_entities.is_empty() {
        //             ec.clear_children();
        //         } else {
        //             let skip = old_children
        //                 .iter()
        //                 .zip(new_child_entities.iter())
        //                 .take_while(|(a, b)| a == b)
        //                 .count();
        //             if skip == new_child_entities.len() {
        //                 //Do nothing
        //             } else if skip > 0 {
        //                 ec.remove_children(&old_children[skip..]);
        //                 ec.push_children(&new_child_entities[skip..]);
        //             } else {
        //                 ec.replace_children(&new_child_entities);
        //             }
        //         }
        //     }
        //     None => {
        //         if !new_child_entities.is_empty() {
        //             ec.push_children(&new_child_entities);
        //         }
        //     }
        // }
    }
}
