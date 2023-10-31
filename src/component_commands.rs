use crate::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

pub struct ComponentCommands<'c, 'w, 's, 'a, 'world> {
    ec: &'c mut EntityCommands<'w, 's, 'a>,
    world: &'world World,
    set_event: SetEvent,
}

impl<'c, 'w, 's, 'a, 'world> ComponentCommands<'c, 'w, 's, 'a, 'world> {
    pub(crate) fn new(
        ec: &'c mut EntityCommands<'w, 's, 'a>,
        world: &'world World,
        set_event: SetEvent,
    ) -> Self {
        Self {
            ec,
            world,
            set_event,
        }
    }

    #[must_use] pub fn get<T: Component>(&self) -> Option<&T> {
        if self.set_event == SetEvent::Created {
            None
        } else {
            self.world.get::<T>(self.ec.id())
        }
    }

    pub fn insert<T: Bundle>(&mut self, bundle: T) {
        self.ec.insert(bundle);
    }

    pub fn remove<T: Bundle>(&mut self) {
        if self.set_event == SetEvent::Created {
            return;
        }
        self.ec.remove::<T>();
    }
}
