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
    pub fn get_res_untracked<R: Resource>(&self) -> Option<&R> {
        self.world.get_resource()
    }

    /// Perform an action on the first child matching a predicate
    /// //TODO only available on deletion?
    pub fn try_modify_child(
        &mut self,
        predicate: impl Fn(EntityRef) -> bool,
        action: impl FnOnce(EntityCommands)
    ){
        let Some(children) = self.world.get_entity(self.ec.id()).and_then(|x|x.get::<Children>()) else{
            //warn!("Could not get children");
            return;};

        for child_entity in children.iter() {
            if let Some(child) = self.world.get_entity(*child_entity) {
                if predicate(child) {
                    let child_commands = self.ec.commands().entity(*child_entity);
                    action(child_commands);
                    return;
                }
            }
        }

        //warn!("No child matched predicate");

    }
}
