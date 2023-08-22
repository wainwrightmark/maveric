use std::{any::type_name, marker::PhantomData};

use crate::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*, utils::hashbrown::HashMap};

pub(crate) struct UnorderedChildCommands<'w, 's, 'a, 'q, R: HierarchyRoot> {
    ec: EntityCommands<'w, 's, 'a>,
    remaining_old_entities: HashMap<ChildKey, Entity>,
    world: &'q World,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'a, 'q, R: HierarchyRoot> ChildCommands for UnorderedChildCommands<'w, 's, 'a, 'q, R> {
    fn add_child<NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &<NChild::Context as NodeContext>::Wrapper<'_>,
    ) {
        let key = key.into();

        if let Some(entity) = self.remaining_old_entities.remove(&key) {
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
                return; // do not spawn a new child;
            }
            warn!(
                "Child with key '{key}' has had node type changed to {}",
                type_name::<NChild>()
            );
            // The node type has changed - delete this entity and readd
            self.ec.commands().entity(entity).despawn_recursive();
        }

        self.ec.with_children(|cb| {
            let cec = cb.spawn_empty();
            create_recursive::<R, NChild>(cec, child, context, key, self.world);
        });
    }
}

impl<'w, 's, 'a, 'q, R: HierarchyRoot> UnorderedChildCommands<'w, 's, 'a, 'q, R> {
    pub(crate) fn new(
        ec: EntityCommands<'w, 's, 'a>,
        world: &'q World,
    ) -> Self {
        let children = world.get::<Children>(ec.id());

        match children {
            Some(children) => {
                let remaining_old_entities: HashMap<ChildKey, Entity> = children
                    .iter()
                    .flat_map(|entity| {
                        world
                            .get::<HierarchyChildComponent<R>>(*entity)
                            .map(|hcc| (hcc.key, *entity))
                    })
                    .collect();

                Self {
                    ec,
                    remaining_old_entities,
                    world,
                    phantom: PhantomData,
                }
            }
            None => Self {
                ec,
                remaining_old_entities: Default::default(),
                world,
                phantom: PhantomData,
            },
        }
    }

    pub fn finish(self) {
        let mut ec = self.ec;

        //remove all remaining old entities
        for (_key, entity) in self.remaining_old_entities {
            let _ = delete_recursive::<R>(ec.commands(), entity, self.world);
        }
    }
}
