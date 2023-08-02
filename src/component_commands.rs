use std::{any::type_name, marker::PhantomData, rc::Rc};

use crate::{create_recursive, prelude::*, update_recursive, DeletionPolicy};
use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};

pub trait ChildCommands: CommandsBase {
    fn add_child<'c, N: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child_context: &<<N as NodeBase>::Context as NodeContext>::Wrapper<'c>,
        child_args: <N as NodeBase>::Args,
    );
}

pub trait CommandsBase {
    fn get<T: Component>(&self) -> Option<&T>;
}

pub trait ComponentCommands: CommandsBase {
    fn insert<T: Bundle>(&mut self, bundle: T);
    fn remove<T: Bundle>(&mut self);
}

pub(crate) struct ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    pub entity_ref: EntityRef<'w_e>,
    ec: &'b mut EntityCommands<'w, 's, 'a>,
}

impl<'w_e, 'w, 's, 'a, 'b> CommandsBase for ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    fn get<T: Component>(&self) -> Option<&T> {
        self.entity_ref.get()
    }
}

impl<'w_e, 'w, 's, 'a, 'b> ComponentCommands for ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
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

pub(crate) struct CreationCommands<'w, 's, 'a, 'b, R: HierarchyRoot> {
    ec: &'b mut EntityCommands<'w, 's, 'a>,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'a, 'b, R: HierarchyRoot> CommandsBase for CreationCommands<'w, 's, 'a, 'b, R> {
    fn get<T: Component>(&self) -> Option<&T> {
        None
    }
}

impl<'w, 's, 'a, 'b, R: HierarchyRoot> ComponentCommands for CreationCommands<'w, 's, 'a, 'b, R> {
    fn insert<T: Bundle>(&mut self, bundle: T) {
        self.ec.insert(bundle);
    }

    fn remove<T: Bundle>(&mut self) {}
}

impl<'w, 's, 'a, 'b, R: HierarchyRoot> CreationCommands<'w, 's, 'a, 'b, R> {
    pub(crate) fn new(ec: &'b mut EntityCommands<'w, 's, 'a>) -> Self {
        Self {
            ec,
            phantom: PhantomData,
        }
    }
}

impl<'w, 's, 'a, 'b, R: HierarchyRoot> ChildCommands for CreationCommands<'w, 's, 'a, 'b, R> {
    fn add_child<'c, N: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child_context: &<<N as NodeBase>::Context as NodeContext>::Wrapper<'c>,
        child_args: <N as NodeBase>::Args,
    ) {
        self.ec.with_children(|cb| {
            let mut cec = cb.spawn(HierarchyChildComponent::<R>::new::<N>(key.into()));
            create_recursive::<R, N>(&mut cec, child_args, &child_context);
        });
    }
}

pub(crate) struct RootCommands<'w, 's, 'b, 'w1, R: HierarchyRoot> {
    commands: &'b mut Commands<'w, 's>,
    remaining_old_entities: HashMap<ChildKey, (EntityRef<'w1>, HierarchyChildComponent<R>)>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, HierarchyChildComponent<R>)>>,
}

impl<'w_e, 'w, 's, 'b, 'w1, R: HierarchyRoot> ChildCommands for RootCommands<'w, 's, 'b, 'w1, R> {
    fn add_child<'c, NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        context: &<<NChild as NodeBase>::Context as NodeContext>::Wrapper<'c>,
        args: <NChild as NodeBase>::Args,
    ) {
        let context_changed = <NChild::Context as NodeContext>::has_changed(context);
        let key = key.into();

        match self.remaining_old_entities.remove(&key) {
            Some((entity_ref, _)) => {
                //check if this node has changed

                match entity_ref.get::<HierarchyNodeComponent<NChild>>() {
                    Some(existing) => {
                        // unschedule it for deletion if necessary

                        let undeleted = if entity_ref.contains::<ScheduledForDeletion>() {
                            let mut cec = self.commands.entity(entity_ref.id());
                            cec.remove::<ScheduledForDeletion>();
                            true
                        } else {
                            false
                        };

                        update_recursive::<R, NChild>(
                            &mut self.commands,
                            entity_ref.clone(),
                            args,
                            context,
                            self.all_child_nodes.clone(),
                            undeleted,
                        );
                    }
                    None => {
                        warn!(
                            "Child with key '{key}' has had node type changed to {}",
                            type_name::<NChild>()
                        );
                        // The node type has changed - delete this entity and readd
                        self.commands.entity(entity_ref.id()).despawn_recursive();

                        let mut cec = self
                            .commands
                            .spawn(HierarchyChildComponent::<R>::new::<NChild>(key));
                        create_recursive::<R, NChild>(&mut cec, args, &context);
                    }
                }
            }
            None => {
                let mut cec = self
                    .commands
                    .spawn(HierarchyChildComponent::<R>::new::<NChild>(key));
                create_recursive::<R, NChild>(&mut cec, args, &context);
            }
        }
    }
}

impl<'w, 's, 'b, 'w1, R: HierarchyRoot> CommandsBase for RootCommands<'w, 's, 'b, 'w1, R> {
    fn get<T: Component>(&self) -> Option<&T> {
        None
    }
}

impl<'w, 's, 'b, 'w1, R: HierarchyRoot> RootCommands<'w, 's, 'b, 'w1, R> {
    pub(crate) fn new(
        commands: &'b mut Commands<'w, 's>,
        all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, HierarchyChildComponent<R>)>>,
        query: Query<Entity, (Without<Parent>, With<HierarchyChildComponent<R>>)>,
    ) -> Self {
        let remaining_old_entities: HashMap<
            ChildKey,
            (EntityRef<'w1>, HierarchyChildComponent<R>),
        > = query
            .into_iter()
            .flat_map(|x| match all_child_nodes.get(&x) {
                Some((er, child)) => Some((child.key, (er.clone(), child.clone()))),
                None => {
                    //new_entities.push(*x);
                    None
                }
            })
            .collect();

        Self {
            commands,
            remaining_old_entities,
            all_child_nodes,
        }
    }

    pub(crate) fn finish(self) {
        //remove all remaining old entities
        for (_key, (er, child)) in self.remaining_old_entities {
            let mut child_ec = self.commands.entity(er.id());
            // todo linger

            child_ec.despawn_recursive();
        }
    }
}

pub(crate) struct UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, R: HierarchyRoot> {
    entity_ref: EntityRef<'w_e>,
    ec: &'b mut EntityCommands<'w, 's, 'a>,
    remaining_old_entities: HashMap<ChildKey, (EntityRef<'w1>, HierarchyChildComponent<R>)>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, HierarchyChildComponent<R>)>>,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'a, 'b, 'w1, 'w_e, R: HierarchyRoot> CommandsBase
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, R>
{
    fn get<T: Component>(&self) -> Option<&T> {
        self.entity_ref.get()
    }
}

impl<'w, 's, 'a, 'b, 'w1, 'w_e, R: HierarchyRoot> ComponentCommands
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, R>
{
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
    pub(crate) fn new(
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
                    phantom: PhantomData,
                }
            }
            None => Self {
                ec,
                entity_ref,
                remaining_old_entities: Default::default(),
                all_child_nodes,
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
            //todo linger
            child_ec.despawn_recursive();
            // let deletion_policy = child
            //     .deleter
            //     .on_deleted(&mut update_commands, &self.added_children);

            // match deletion_policy {
            //     DeletionPolicy::DeleteImmediately => {
            //         //info!("Despawning Child with key '{key}'");
            //         //do nothing
            //         child_ec.despawn_recursive();
            //     }
            //     DeletionPolicy::Linger(duration) => {
            //         if !er.contains::<ScheduledForDeletion>() {
            //             //info!("Scheduling deletion of Child with key '{key}'");
            //             child_ec.insert(ScheduledForDeletion {
            //                 timer: Timer::new(duration, TimerMode::Once),
            //             });
            //         }
            //     }
            // }
        }
    }
}

impl<'w, 's, 'a, 'b, 'w1, 'w_e, R: HierarchyRoot> ChildCommands
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, R>
{
    fn add_child<'c, NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        context: &<<NChild as NodeBase>::Context as NodeContext>::Wrapper<'c>,
        args: <NChild as NodeBase>::Args,
    ) {
        let context_changed = <NChild::Context as NodeContext>::has_changed(context);
        let key = key.into();

        match self.remaining_old_entities.remove(&key) {
            Some((entity_ref, _)) => {
                //check if this node has changed

                match entity_ref.get::<HierarchyNodeComponent<NChild>>() {
                    Some(existing) => {
                        // unschedule it for deletion if necessary

                        let undeleted = if entity_ref.contains::<ScheduledForDeletion>() {
                            let mut cec = self.ec.commands().entity(entity_ref.id());
                            cec.remove::<ScheduledForDeletion>();
                            true
                        } else {
                            false
                        };

                        update_recursive::<R, NChild>(
                            &mut self.ec.commands(),
                            entity_ref.clone(),
                            args,
                            context,
                            self.all_child_nodes.clone(),
                            undeleted,
                        );
                    }
                    None => {
                        warn!(
                            "Child with key '{key}' has had node type changed to {}",
                            type_name::<NChild>()
                        );
                        // The node type has changed - delete this entity and readd
                        self.ec
                            .commands()
                            .entity(entity_ref.id())
                            .despawn_recursive();

                        self.ec.with_children(|cb| {
                            let mut cec =
                                cb.spawn(HierarchyChildComponent::<R>::new::<NChild>(key));
                            create_recursive::<R, NChild>(&mut cec, args, &context);
                        });
                    }
                }
            }
            None => {
                self.ec.with_children(|cb| {
                    //info!("Creating new Child {} with key '{key}'", type_name::<N>());
                    let mut cec = cb.spawn(HierarchyChildComponent::<R>::new::<NChild>(key));
                    create_recursive::<R, NChild>(&mut cec, args, &context);
                });
            }
        }
    }
}
