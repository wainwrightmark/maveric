use std::{any::type_name, marker::PhantomData};

use crate::prelude::*;
use bevy::{prelude::*, utils::hashbrown::HashMap};

pub(crate) struct RootCommands<'w, 's, 'b, 'q, R: MavericRoot> {
    commands: &'b mut Commands<'w, 's>,
    remaining_old_entities: HashMap<ChildKey, Entity>,
    world: &'q World,
    phantom: PhantomData<R>
}

impl<'w, 's, 'b, 'w1,'q : 'w1, R: MavericRoot> RootCommands<'w, 's, 'b, 'q, R> {
    pub(crate) fn new<'w2, 's2>(
        commands: &'b mut Commands<'w, 's>,
        world: &'q World,
        query: Query< (Entity, &MavericChildComponent<R>), Without<Parent>>,
    ) -> Self {
        let remaining_old_entities: HashMap<ChildKey, Entity> = query
            .into_iter()
            .map(|x| (x.1.key, x.0))
            .map(|(key, entity)| (key, entity) )
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
            let _= delete_recursive::<R>(self.commands, er, self.world);
        }
    }
}

impl<'w, 's, 'b, 'w1,'q, R: MavericRoot> ChildCommands for RootCommands<'w, 's, 'b, 'q, R> {
    fn add_child<NChild: MavericNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &<NChild::Context as NodeContext>::Wrapper<'_>,
    ) {
        let key = key.into();

        match self.remaining_old_entities.remove(&key) {
            Some(entity) => {
                if self.world.get::<MavericNodeComponent<NChild>>(entity).is_some() {
                    update_recursive::<R, NChild>(
                        self.commands,
                        entity,
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
                    self.commands.entity(entity).despawn_recursive();

                    let cec = self.commands.spawn_empty();
                    create_recursive::<R, NChild>(cec, child, context, key, self.world);
                }
            }
            None => {
                let cec = self.commands.spawn_empty();
                create_recursive::<R, NChild>(cec, child, context, key, self.world);
            }
        }
    }
}
