use bevy::{ecs::system::EntityCommands, prelude::*};

pub trait ComponentCommands {
    fn insert<T: Bundle>(&mut self, bundle: T);
    fn remove<T: Bundle>(&mut self);

    // todo insert or update component
}

//#[derive(Debug)]
pub(crate) struct ComponentCreationCommands<'w, 's, 'a, 'b> {
    ec: &'b mut EntityCommands<'w, 's, 'a>,
}

impl<'w, 's, 'a, 'b> ComponentCreationCommands<'w, 's, 'a, 'b> {
    pub(crate) fn new(ec: &'b mut EntityCommands<'w, 's, 'a>) -> Self {
        Self { ec }
    }
}

impl<'w, 's, 'a, 'b> ComponentCommands for ComponentCreationCommands<'w, 's, 'a, 'b> {
    fn insert<T: Bundle>(&mut self, bundle: T) {
        self.ec.insert(bundle);
    }

    fn remove<T: Bundle>(&mut self) {
        //Do nothing
    }
}

//#[derive(Debug)]
pub(crate) struct ComponentUpdateCommands<'w_e, 'w, 's, 'a, 'b> {
    entity_ref: EntityRef<'w_e>,
    ec: &'b mut EntityCommands<'w, 's, 'a>,
}

impl<'w_e, 'w, 's, 'a, 'b> ComponentUpdateCommands<'w_e, 'w, 's, 'a, 'b> {
    pub(crate) fn new(entity_ref: EntityRef<'w_e>, ec: &'b mut EntityCommands<'w, 's, 'a>) -> Self {
        Self { entity_ref, ec }
    }
}

impl<'w_e, 'w, 's, 'a, 'b> ComponentCommands for ComponentUpdateCommands<'w_e, 'w, 's, 'a, 'b> {
    fn insert<T: Bundle>(&mut self, bundle: T) {
        self.ec.insert(bundle);
    }

    fn remove<T: Bundle>(&mut self) {
        self.ec.remove::<T>();
    }
}
