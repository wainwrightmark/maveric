use crate::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*};

pub struct ComponentCommands<'c, 'a, 'world> {
    ec: &'c mut EntityCommands<'a>,
    world: &'world World,
    set_event: SetEvent,
}

impl<'c, 'a, 'world> ComponentCommands<'c, 'a, 'world> {
    pub(crate) fn new(
        ec: &'c mut EntityCommands<'a>,
        world: &'world World,
        set_event: SetEvent,
    ) -> Self {
        Self {
            ec,
            world,
            set_event,
        }
    }

    #[must_use]
    pub fn get<T: Component>(&self) -> Option<&T> {
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

    /// Gets a resource.
    /// This resource usage is not tracked, meaning changes to this resource will not result in recalculating components
    #[must_use]
    pub fn get_res_untracked<R: Resource>(&self) -> Option<&R> {
        self.world.get_resource()
    }

    /// Insert a resource into the world.
    /// You probably shouldn't use this unless you know what you are doing but it can be useful in implementing `on_deleted`
    pub fn insert_resource<R: Resource>(&mut self, resource: R) {
        self.ec.commands().insert_resource(resource);
    }

    pub fn modify_children(&mut self, action: impl Fn(EntityRef, EntityCommands)) {
        let Some(children) = self
            .world
            .get_entity(self.ec.id())
            .and_then(|x| x.get::<Children>())
        else {
            //warn!("Could not get children");
            return;
        };

        for child_entity in children {
            if let Some(child) = self.world.get_entity(*child_entity) {
                let mut commands = self.ec.commands();
                let child_ec = commands.entity(*child_entity);
                action(child, child_ec);
            }
        }
    }
}
