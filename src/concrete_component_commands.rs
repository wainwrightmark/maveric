use crate::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

pub struct ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    ec: &'b mut EntityCommands<'w, 's, 'a>,
    world: &'w_e World,

}

impl<'w_e, 'w, 's, 'a, 'b> ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    pub fn new(ec: &'b mut EntityCommands<'w, 's, 'a>, world: &'w_e World, ) -> Self { Self { world, ec } }
}

impl<'w_e, 'w, 's, 'a, 'b> ComponentCommands for ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    fn get<T: Component>(&self) -> Option<&T> {
        self.world.get::<T>(self.ec.id())
    }

    fn insert<T: Bundle>(&mut self, bundle: T) {
        self.ec.insert(bundle);
    }

    fn remove<T: Bundle>(&mut self) {
        self.ec.remove::<T>();
    }
}
