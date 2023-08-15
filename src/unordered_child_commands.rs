use std::{any::type_name, marker::PhantomData};

use crate::prelude::*;
use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    utils::hashbrown::HashMap,
};

pub(crate) struct UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'q, R: HierarchyRoot> {
    ec: &'b mut EntityCommands<'w, 's, 'a>,

    remaining_old_entities: HashMap<ChildKey, EntityRef<'w1>>,
    world: &'q World,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'a, 'b, 'w1, 'q, R: HierarchyRoot> ChildCommands
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'q, R>
{
    fn add_child<NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &<NChild::Context as NodeContext>::Wrapper<'_>,
    ) {
        //let child_context = <NParent as HasChild<NChild>>::convert_context(self.context);
        //let context_changed = <NChild::Context as NodeContext>::has_changed(context);
        let key = key.into();

        match self.remaining_old_entities.remove(&key) {
            Some(entity_ref) => {
                //check if this node has changed

                if entity_ref.contains::<HierarchyNodeComponent<NChild>>() {
                    update_recursive::<R, NChild>(
                        self.ec.commands(),
                        entity_ref,
                        child,
                        context,
                        self.world,
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
                        let mut cec = cb.spawn_empty();
                        create_recursive::<R, NChild>(&mut cec, child, context, key);
                    });
                }
            }
            None => {
                self.ec.with_children(|cb| {
                    let mut cec = cb.spawn_empty();
                    create_recursive::<R, NChild>(&mut cec, child, context, key);
                });
            }
        }
    }
}

impl<'w, 's, 'a, 'b, 'w1, 'q : 'w1, R: HierarchyRoot> UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'q, R> {
    pub(crate) fn new(
        ec: &'b mut EntityCommands<'w, 's, 'a>,
        children: Option<&Children>,
        world: &'q World,
    ) -> Self {
        //let tree = tree.clone();
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
        let ec = self.ec;

        //remove all remaining old entities
        for (_key, entity_ref) in self.remaining_old_entities {
            delete_recursive::<R>(ec.commands(), entity_ref);
        }
    }
}
