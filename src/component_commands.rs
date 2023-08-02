use std::{any::type_name, marker::PhantomData, rc::Rc};

use crate::{create_recursive, prelude::*, update_recursive, DeletionPolicy};
use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};

pub trait UpdateCommands: ComponentCommands {
    fn add_child<'c,  N: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child_context: &<N::Context as NodeContext>::Wrapper<'c>,
        child_node: N,
    );
}

pub trait ComponentCommands {
    fn get<T: Component>(&self) -> Option<&T>;
    fn insert<T: Bundle>(&mut self, bundle: T);
    fn remove<T: Bundle>(&mut self);
}

pub(crate) struct ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    pub entity_ref: EntityRef<'w_e>,
    ec: &'b mut EntityCommands<'w, 's, 'a>,
}

impl<'w_e, 'w, 's, 'a, 'b> ComponentCommands for ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    fn get<T: Component>(&self) -> Option<&T> {
        self.entity_ref.get()
    }

    fn insert<T: Bundle>(&mut self, bundle: T) {
        self.ec.insert(bundle);
    }

    fn remove<T: Bundle>(&mut self) {
        self.ec.remove::<T>();
    }
}

impl<'w_e, 'w, 's, 'a, 'b> ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    pub(crate) fn new(entity_ref: EntityRef<'w_e>, ec: &'b mut EntityCommands<'w, 's, 'a>) -> Self {
        Self { entity_ref, ec }
    }
}

pub(crate) struct CreationHierarchyCommands<'w, 's, 'a, 'b, R: HierarchyRoot> {
    ec: &'b mut EntityCommands<'w, 's, 'a>,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'a, 'b, R: HierarchyRoot> ComponentCommands
    for CreationHierarchyCommands<'w, 's, 'a, 'b, R>
{
    fn get<T: Component>(&self) -> Option<&T> {
        None
    }

    fn insert<T: Bundle>(&mut self, bundle: T) {
        self.ec.insert(bundle);
    }

    fn remove<T: Bundle>(&mut self) {}
}

impl<'w, 's, 'a, 'b, R: HierarchyRoot> CreationHierarchyCommands<'w, 's, 'a, 'b, R> {
    pub(crate) fn new(ec: &'b mut EntityCommands<'w, 's, 'a>) -> Self {
        Self {
            ec,
            phantom: PhantomData,
        }
    }
}

impl<'w, 's, 'a, 'b, R: HierarchyRoot> UpdateCommands
    for CreationHierarchyCommands<'w, 's, 'a, 'b, R>
{
    fn add_child<'c, N: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child_context: &<N::Context as NodeContext>::Wrapper<'c>,
        child_node: N,
    ) {
        self.ec.with_children(|cb| {
            let mut cec = cb.spawn(HierarchyChildComponent::<R>::new::<N>(key.into()));
            create_recursive::<R, N>(&mut cec, child_node, &child_context);
        });
    }
}

pub(crate) struct UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, R: HierarchyRoot> {
    entity_ref: EntityRef<'w_e>,
    ec: &'b mut EntityCommands<'w, 's, 'a>,
    remaining_old_entities: HashMap<ChildKey, (EntityRef<'w1>, HierarchyChildComponent<R>)>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, HierarchyChildComponent<R>)>>,
    phantom: PhantomData<R>,

    added_children: HashSet<ChildKey>,
}

impl<'w, 's, 'a, 'b, 'w1, 'w_e, R: HierarchyRoot> ComponentCommands for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, R> {
    fn get<T: Component>(&self) -> Option<&T> {
        self.entity_ref.get()
    }

    fn insert<T: Bundle>(&mut self, bundle: T) {
        self.ec.insert(bundle);
    }

    fn remove<T: Bundle>(&mut self) {
        self.ec.remove::<T>();
    }
}

impl<'w, 's, 'a, 'b, 'w1, 'w_e, R: HierarchyRoot>
    UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, R>
{
        pub (crate) fn new(
        ec: &'b mut EntityCommands<'w, 's, 'a>,
        entity_ref: EntityRef<'w_e>,
        children: Option<&Children>,
        all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, HierarchyChildComponent<R>)>>,
    ) -> Self {
        //let tree = tree.clone();
        match children {
            Some(children) => {
                let remaining_old_entities: HashMap<
                    ChildKey,
                    (EntityRef<'w1>, HierarchyChildComponent<R>),
                > = children
                    .iter()
                    .flat_map(|x| match all_child_nodes.get(x) {
                        Some((er, child)) => Some((child.key, (er.clone(), child.clone()))),
                        None => {
                            //new_entities.push(*x);
                            None
                        }
                    })
                    .collect();

                Self {
                    ec,
                    entity_ref,
                    remaining_old_entities,
                    all_child_nodes,
                    added_children: Default::default(),
                    phantom: PhantomData,
                }
            }
            None => Self {
                ec,
                entity_ref,
                remaining_old_entities: Default::default(),
                all_child_nodes,
                added_children: Default::default(),
                phantom: PhantomData,
            },
        }
    }

    pub fn finish(self) {
        let ec = self.ec;

        //remove all remaining old entities
        for (_key, (er, child)) in self.remaining_old_entities {
            let mut child_ec = ec.commands().entity(er.id());
            let mut update_commands = ConcreteComponentCommands::new(er, &mut child_ec);
            let deletion_policy = child
                .deleter
                .on_deleted(&mut update_commands, &self.added_children);

            match deletion_policy {
                DeletionPolicy::DeleteImmediately => {
                    //info!("Despawning Child with key '{key}'");
                    //do nothing
                    child_ec.despawn_recursive();
                }
                DeletionPolicy::Linger(duration) => {
                    if !er.contains::<ScheduledForDeletion>() {
                        //info!("Scheduling deletion of Child with key '{key}'");
                        child_ec.insert(ScheduledForDeletion {
                            timer: Timer::new(duration, TimerMode::Once),
                        });
                    }
                }
            }
        }
    }
}


impl<'w, 's, 'a, 'b, 'w1, 'w_e, R: HierarchyRoot> UpdateCommands
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, R>
{
    fn add_child<'c, N: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child_context: &<N::Context as NodeContext>::Wrapper<'c>,
        child_node: N,
    ) {
        let context_changed = <N::Context as NodeContext>::has_changed(child_context);
        let key = key.into();

        if !self.added_children.insert(key) {
            debug_assert!(
                false,
                "{n} added two children with key {key}",
                n = type_name::<N>()
            );
            warn!(
                "{n} added two children with key {key}",
                n = type_name::<N>()
            );
        }

        match self.remaining_old_entities.remove(&key) {
            Some((entity_ref, _)) => {
                //check if this node has changed

                match entity_ref.get::<HierarchyNodeComponent<N>>() {
                    Some(existing) => {
                        // unschedule it for deletion if necessary

                        if context_changed || existing.node != child_node {
                            //state has changed
                            //info!("Child {} with key '{key}' has changed", type_name::<N>());

                            update_recursive::<R, N>(
                                &mut self.ec.commands(),
                                entity_ref.clone(),
                                child_node,
                                child_context,
                                self.all_child_nodes.clone(),
                            );
                        } else {
                            //state has not changed
                            if entity_ref.contains::<ScheduledForDeletion>() {
                                let mut cec = self.ec.commands().entity(entity_ref.id());
                                cec.remove::<ScheduledForDeletion>();
                                let mut commands = ConcreteComponentCommands::new(entity_ref, &mut cec);
                                child_node.on_undeleted(child_context, &mut commands);
                            }
                        }
                    }
                    None => {
                        warn!(
                            "Child with key '{key}' has had node type changed to {}",
                            type_name::<N>()
                        );
                        // The node type has changed - delete this entity and readd
                        self.ec
                            .commands()
                            .entity(entity_ref.id())
                            .despawn_recursive();

                        self.ec.with_children(|cb| {
                            let mut cec = cb.spawn(HierarchyChildComponent::<R>::new::<N>(key));
                            create_recursive::<R, N>(&mut cec, child_node, &child_context);
                        });
                    }
                }
            }
            None => {
                self.ec.with_children(|cb| {
                    //info!("Creating new Child {} with key '{key}'", type_name::<N>());
                    let mut cec = cb.spawn(HierarchyChildComponent::<R>::new::<N>(key));
                    create_recursive::<R, N>(&mut cec, child_node, &child_context);
                });
            }
        }
    }
}
