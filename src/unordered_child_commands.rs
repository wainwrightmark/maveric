use std::{any::type_name, marker::PhantomData};

use crate::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*, utils::hashbrown::HashMap};

pub(crate) struct UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'q, 'w_e, R: HierarchyRootChildren> {
    ec: &'b mut EntityCommands<'w, 's, 'a>,
    entity_ref: EntityRef<'w_e>,
    remaining_old_entities: HashMap<ChildKey, EntityRef<'w1>>,
    world: &'q World,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'a, 'b, 'w1, 'q, 'w_e, R: HierarchyRootChildren> ComponentCommands
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'q, 'w_e, R>
{
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

impl<'w, 's, 'a, 'b, 'w1, 'q, 'w_e, R: HierarchyRootChildren> ChildCommands
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'q, 'w_e, R>
{
    fn add_child<NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &<NChild::Context as NodeContext>::Wrapper<'_>,
    ) {
        let key = key.into();

        if let Some(entity_ref) = self.remaining_old_entities.remove(&key) {
            //check if this node has changed

            if entity_ref.contains::<HierarchyNodeComponent<NChild>>() {
                update_recursive::<R, NChild>(
                    self.ec.commands(),
                    entity_ref,
                    child,
                    context,
                    self.world,
                );
                return; // do not spawn a new child;
            }
            warn!(
                "Child with key '{key}' has had node type changed to {}",
                type_name::<NChild>()
            );
            // The node type has changed - delete this entity and readd
            self.ec
                .commands()
                .entity(entity_ref.id())
                .despawn_recursive();
        }

        self.ec.with_children(|cb| {
            let mut cec = cb.spawn_empty();
            create_recursive::<R, NChild>(&mut cec, child, context, key);
        });
    }
}

impl<'w, 's, 'a, 'b, 'w1, 'q: 'w1, 'w_e, R: HierarchyRootChildren>
    UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'q, 'w_e, R>
{
    pub(crate) fn new(
        ec: &'b mut EntityCommands<'w, 's, 'a>,
        entity_ref: EntityRef<'w_e>,
        children: Option<&Children>,
        world: &'q World,
    ) -> Self {
        match children {
            Some(children) => {
                let remaining_old_entities: HashMap<ChildKey, EntityRef<'w1>> = children
                    .iter()
                    .flat_map(|x| world.get_entity(*x))
                    .flat_map(|er| {
                        er.get::<HierarchyChildComponent<R>>()
                            .map(|hcc| (hcc.key, er))
                    })
                    .collect();

                Self {
                    ec,
                    entity_ref,
                    remaining_old_entities,
                    world,
                    phantom: PhantomData,
                }
            }
            None => Self {
                ec,
                entity_ref,
                remaining_old_entities: Default::default(),
                world,
                phantom: PhantomData,
            },
        }
    }

    pub fn finish(self) {
        let ec = self.ec;

        //remove all remaining old entities
        for (_key, entity_ref) in self.remaining_old_entities {
            let _ = delete_recursive::<R>(ec.commands(), entity_ref);
        }
    }
}
