use std::{any::type_name, marker::PhantomData, rc::Rc};

use crate::{prelude::*, DeletionPolicy};
use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};

pub  struct ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    pub entity_ref: EntityRef<'w_e>,
    ec: &'b mut EntityCommands<'w, 's, 'a>,
}

impl<'w_e, 'w, 's, 'a, 'b> CommandsBase for ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    fn get<T: Component>(&self) -> Option<&T> {
        self.entity_ref.get()
    }
}

impl<'w_e, 'w, 's, 'a, 'b> ComponentCommands for ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    fn insert<T: Bundle>(&mut self, bundle: T) {
        self.ec.insert(bundle);
    }

    fn remove<T: Bundle>(&mut self) {
        self.ec.remove::<T>();
    }
}

impl<'w_e, 'w, 's, 'a, 'b> ConcreteComponentCommands<'w_e, 'w, 's, 'a, 'b> {
    pub(crate) fn new(entity_ref: EntityRef<'w_e>, ec: &'b mut EntityCommands<'w, 's, 'a>) -> Self {
        Self { entity_ref, ec }
    }
}
