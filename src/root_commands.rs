use std::{any::type_name, marker::PhantomData};

use crate::prelude::*;
use bevy::{prelude::*, utils::hashbrown::HashMap};

pub(crate) struct RootCommands<'w, 's, 'b, 'w1, 'q, R: HierarchyRoot> {
    commands: &'b mut Commands<'w, 's>,
    remaining_old_entities: HashMap<ChildKey, EntityRef<'w1>>,
    world: &'q World,
    phantom: PhantomData<R>
}

impl<'w, 's, 'b, 'w1,'q : 'w1, R: HierarchyRoot> RootCommands<'w, 's, 'b, 'w1,'q, R> {
    pub(crate) fn new<'w2, 's2>(
        commands: &'b mut Commands<'w, 's>,
        world: &'q World,
        query: Query< (Entity, &HierarchyChildComponent<R>), Without<Parent>>,
    ) -> Self {
        let remaining_old_entities: HashMap<ChildKey, EntityRef<'w1>> = query
            .into_iter()
            .map(|x| (x.1.key, x.0))
            .flat_map(|(key, entity)| world.get_entity(entity).map(|er| (key, er)) )
            .collect();

        Self {
            commands,
            remaining_old_entities,
            world,
            phantom: PhantomData::default(),
        }
    }

    pub(crate) fn finish(self) {
        for (_key, er) in self.remaining_old_entities {
            delete_recursive::<R>(self.commands, er);
        }
    }
}

impl<'w, 's, 'b, 'w1,'q, R: HierarchyRoot> ChildCommands for RootCommands<'w, 's, 'b, 'w1,'q, R> {
    fn add_child<NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &<NChild::Context as NodeContext>::Wrapper<'_>,
    ) {
        let key = key.into();

        match self.remaining_old_entities.remove(&key) {
            Some(entity_ref) => {
                if entity_ref.contains::<HierarchyNodeComponent<NChild>>() {
                    update_recursive::<R, NChild>(
                        self.commands,
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
                    self.commands.entity(entity_ref.id()).despawn_recursive();

                    let mut cec = self.commands.spawn_empty();
                    create_recursive::<R, NChild>(&mut cec, child, context, key);
                }
            }
            None => {
                let mut cec = self.commands.spawn_empty();
                create_recursive::<R, NChild>(&mut cec, child, context, key);
            }
        }
    }
}
