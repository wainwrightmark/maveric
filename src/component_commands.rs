use crate::prelude::*;
use bevy::prelude::*;

pub struct ComponentCommands<'c, 'w, 's, 'a, 'world> {
    commands: &'c mut NodeCommands<'w, 's, 'a, 'world>,
    set_event: SetEvent
}

impl<'c, 'w, 's, 'a, 'world> ComponentCommands<'c, 'w, 's, 'a, 'world> {
    pub (crate) fn new(commands: &'c mut NodeCommands<'w, 's, 'a, 'world>, set_event: SetEvent) -> Self { Self { commands, set_event } }

    pub fn get<T: Component>(&self) -> Option<&T> {

        if self.set_event == SetEvent::Created{
            None
        }
        else{
            self.commands.world.get::<T>(self.commands.ec.id())
        }


    }

    pub fn insert<T: Bundle>(&mut self, bundle: T) {
        self.commands.ec.insert(bundle);
    }

    pub fn remove<T: Bundle>(&mut self) {
        if self.set_event == SetEvent::Created{
            return;
        }
        self.commands.ec.remove::<T>();
    }
}

