use bevy::{ecs::system::EntityCommands, prelude::*};

pub trait ComponentCommands {
    fn ensure_present<T: Component + Eq>(&mut self, component: T);
    fn ensure_not_present<T: Component>(&mut self);
}

//#[derive(Debug)]
pub(crate) struct ComponentCreationCommands<'w, 's, 'a, 'b> {
    pub ec: &'b mut EntityCommands<'w, 's, 'a>,
}

impl<'w, 's, 'a, 'b> ComponentCommands for ComponentCreationCommands<'w, 's, 'a, 'b> {
    fn ensure_present<T: Component + Eq>(&mut self, component: T) {
        self.ec.insert(component);
    }

    fn ensure_not_present<T: Component>(&mut self) {
        //Do nothing
    }
}

//#[derive(Debug)]
pub(crate) struct ComponentUpdateCommands<'w_e, 'w, 's, 'a, 'b> {
    pub entity_ref: EntityRef<'w_e>,
    pub ec: &'b mut EntityCommands<'w, 's, 'a>,
}

impl<'w_e, 'w, 's, 'a, 'b> ComponentCommands for ComponentUpdateCommands<'w_e, 'w, 's, 'a, 'b> {

    fn ensure_present<T: Component + Eq>(&mut self, component: T) {
        if let Some(existing) = self.entity_ref.get::<T>() {
            if !existing.eq(&component) {
                self.ec.insert(component);
            }
        } else {
            self.ec.insert(component);
        }
    }

    fn ensure_not_present<T: Component>(&mut self) {
        if self.entity_ref.contains::<T>() {
            self.ec.remove::<T>();
        }
    }
}
