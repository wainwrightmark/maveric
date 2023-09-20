use std::{any::type_name, marker::PhantomData};

use crate::prelude::*;
use bevy::{prelude::*, utils::hashbrown::HashMap};

pub(crate) struct RootCommands<'w, 's, 'b, 'q, 'alloc, R: MavericRoot> {
    commands: &'b mut Commands<'w, 's>,
    remaining_old_entities: HashMap<ChildKey, Entity>,
    world: &'q World,
    phantom: PhantomData<R>,
    allocator: &'alloc mut Allocator,
}

impl<'w, 's, 'b, 'w1, 'q: 'w1, 'alloc, R: MavericRoot> RootCommands<'w, 's, 'b, 'q, 'alloc, R> {
    pub(crate) fn new(
        commands: &'b mut Commands<'w, 's>,
        world: &'q World,
        query: &Query<(Entity, &MavericChildComponent<R>), Without<Parent>>,
        allocator: &'alloc mut Allocator,
    ) -> Self {
        let mut remaining_old_entities: HashMap<ChildKey, Entity> =
            allocator.unordered_entities.claim();

        remaining_old_entities.extend(
            query
                .into_iter()
                .map(|x| (x.1.key, x.0))
                .map(|(key, entity)| (key, entity)),
        );

        Self {
            commands,
            remaining_old_entities,
            world,
            phantom: PhantomData,
            allocator,
        }
    }

    pub(crate) fn finish(self) {
        for (_key, er) in self.remaining_old_entities.iter() {
            let _ = delete_recursive::<R>(self.commands, *er, self.world);
        }

        self.allocator
            .unordered_entities
            .reclaim(self.remaining_old_entities);
    }
}

impl<'w, 's, 'b, 'q, 'alloc, R: MavericRoot> ChildCommands
    for RootCommands<'w, 's, 'b, 'q, 'alloc, R>
{
    fn add_child<NChild: MavericNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &<NChild::Context as NodeContext>::Wrapper<'_>,
    ) {
        let key = key.into();

        if let Some(entity) = self.remaining_old_entities.remove(&key) {
            if self
                .world
                .get::<MavericNodeComponent<NChild>>(entity)
                .is_some()
            {
                update_recursive::<R, NChild>(
                    self.commands,
                    entity,
                    child,
                    context,
                    self.world,
                    self.allocator,
                );
            } else {
                warn!(
                    "Child with key '{key}' has had node type changed to {}",
                    type_name::<NChild>()
                );
                // The node type has changed - delete this entity and readd
                self.commands.entity(entity).despawn_recursive();

                let cec = self.commands.spawn_empty();
                create_recursive::<R, NChild>(
                    cec,
                    child,
                    context,
                    key,
                    self.world,
                    self.allocator,
                );
            }
        } else {
                        let cec = self.commands.spawn_empty();
                        create_recursive::<R, NChild>(cec, child, context, key, self.world, self.allocator);
                    }
    }
}
