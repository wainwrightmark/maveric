use std::{any::type_name, marker::PhantomData, rc::Rc};

use crate::{prelude::*, DeletionPolicy};
use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};

pub(crate) struct UnorderedChildCommands<
    'w,
    's,
    'a,
    'b,
    'w1,
    'w_e,
    'd,
    'r,
    R: HierarchyRoot,
    NParent: AncestorAspect,
> {
    entity_ref: EntityRef<'w_e>,
    ec: &'b mut EntityCommands<'w, 's, 'a>,
    context: &'d <NParent::Context as NodeContext>::Wrapper<'r>,

    remaining_old_entities: HashMap<ChildKey, (EntityRef<'w1>, HierarchyChildComponent<R>)>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, HierarchyChildComponent<R>)>>,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R: HierarchyRoot, NParent: AncestorAspect>
    ChildCommands<NParent>
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R, NParent>
{
    fn add_child<'c, NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child_args: <NChild as NodeBase>::Args,
    ) where
        NParent: HasChild<NChild>,
    {
        let child_context = <NParent as HasChild<NChild>>::convert_context(self.context);
        //let context_changed = <NChild::Context as NodeContext>::has_changed(context);
        let key = key.into();

        match self.remaining_old_entities.remove(&key) {
            Some((entity_ref, _)) => {
                //check if this node has changed

                if entity_ref.contains::<HierarchyNodeComponent<NChild>>() {
                    update_recursive::<R, NChild>(
                        &mut self.ec.commands(),
                        entity_ref.clone(),
                        child_args,
                        child_context,
                        self.all_child_nodes.clone(),
                    );
                } else {
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
                        let mut cec = cb.spawn(HierarchyChildComponent::<R>::new::<NChild>(key));
                        create_recursive::<R, NChild>(&mut cec, child_args, &child_context);
                    });
                }
            }
            None => {
                self.ec.with_children(|cb| {
                    //info!("Creating new Child {} with key '{key}'", type_name::<N>());
                    let mut cec = cb.spawn(HierarchyChildComponent::<R>::new::<NChild>(key));
                    create_recursive::<R, NChild>(&mut cec, child_args, &child_context);
                });
            }
        }
    }
}

impl<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R: HierarchyRoot, NParent: AncestorAspect>
    UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R, NParent>
{
    pub(crate) fn new(
        ec: &'b mut EntityCommands<'w, 's, 'a>,
        entity_ref: EntityRef<'w_e>,
        children: Option<&Children>,
        context: &'d <NParent::Context as NodeContext>::Wrapper<'r>,
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
                    context,
                }
            }
            None => Self {
                ec,
                entity_ref,
                remaining_old_entities: Default::default(),
                all_child_nodes,
                phantom: PhantomData,
                context,
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

// impl<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R: HierarchyRoot, NParent: AncestorAspect> ComponentCommands
//     for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R, NParent>
// {
//     fn insert<T: Bundle>(&mut self, bundle: T) {
//         self.ec.insert(bundle);
//     }

//     fn remove<T: Bundle>(&mut self) {
//         self.ec.remove::<T>();
//     }
// }

impl<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R: HierarchyRoot, NParent: AncestorAspect> CommandsBase
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R, NParent>
{
    fn get<T: Component>(&self) -> Option<&T> {
        self.entity_ref.get()
    }
}
