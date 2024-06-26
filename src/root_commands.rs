use std::{any::type_name, marker::PhantomData};

use crate::prelude::*;
use bevy::{
    prelude::*,
    utils::hashbrown::{hash_map::DefaultHashBuilder, HashMap},
};

pub(crate) struct RootCommands<'w, 's, 'b, 'q, 'alloc, R: MavericRoot> {
    commands: &'b mut Commands<'w, 's>,
    remaining_old_entities: HashMap<ChildKey, Entity, DefaultHashBuilder, &'alloc Allocator>,
    world: &'q World,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'b, 'w1, 'q: 'w1, 'alloc, R: MavericRoot> RootCommands<'w, 's, 'b, 'q, 'alloc, R> {
    pub(crate) fn new(
        commands: &'b mut Commands<'w, 's>,
        world: &'q World,
        query: &Query<(Entity, &MavericChildComponent<R>), Without<Parent>>,
        allocator: &'alloc Allocator,
    ) -> Self {
        let mut remaining_old_entities: HashMap<
            ChildKey,
            Entity,
            DefaultHashBuilder,
            &'alloc Allocator,
        > = HashMap::new_in(allocator);

        remaining_old_entities.extend(query.into_iter().map(|x| (x.1.key, x.0)));

        Self {
            commands,
            remaining_old_entities,
            world,
            phantom: PhantomData,
        }
    }

    pub(crate) fn finish(self) {
        for (_key, er) in &self.remaining_old_entities {
            let _ = delete_recursive::<R>(self.commands, *er, self.world);
        }
    }
}

impl<'w, 's, 'b, 'q, 'alloc, R: MavericRoot> ChildCommands
    for RootCommands<'w, 's, 'b, 'q, 'alloc, R>
{
    fn remove_child(&mut self, key: impl Into<ChildKey>) {
        let key: ChildKey = key.into();

        if let Some(entity) = self.remaining_old_entities.remove(&key) {
            self.commands.entity(entity).despawn_recursive();
        }
    }

    fn add_child<NChild: MavericNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
        context: &NChild::Context<'_, '_>,
    ) {
        let key = key.into();

        if let Some(entity) = self.remaining_old_entities.remove(&key) {
            if let Some(previous) = self.world.get::<MavericNodeComponent<NChild>>(entity) {
                if !child.should_recreate(&previous.node, context) {
                    update_recursive::<R, NChild>(
                        self.commands,
                        entity,
                        child,
                        context,
                        self.world,
                        self.remaining_old_entities.allocator(),
                    );
                    return;
                }
            } else {
                warn!(
                    "Child with key '{key}' has had node type changed to {}",
                    type_name::<NChild>()
                );
            }

            // The node type has changed - delete this entity and readd
            self.commands.entity(entity).despawn_recursive();
        }

        let cec = self.commands.spawn_empty();
        create_recursive::<R, NChild>(
            cec,
            child,
            context,
            key,
            self.world,
            self.remaining_old_entities.allocator(),
        );
    }
}
