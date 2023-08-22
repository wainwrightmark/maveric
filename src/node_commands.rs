use bevy::ecs::system::EntityCommands;

use crate::prelude::*;

pub struct NodeCommands<'w, 's, 'a, 'world> {
    pub(crate) ec: EntityCommands<'w, 's, 'a>,
    pub(crate) world: &'world World,
}

impl<'w, 's, 'a, 'world> NodeCommands<'w, 's, 'a, 'world> {
    pub(crate) fn new(commands: EntityCommands<'w, 's, 'a>, world: &'world World) -> Self {
        Self {
            ec: commands,
            world,
        }
    }
}
