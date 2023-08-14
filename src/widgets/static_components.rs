use bevy::prelude::Bundle;

use super::prelude::{ComponentsAspect, HasContext, SetComponentsEvent};

pub trait StaticComponentsAspect: HasContext {
    type B: Bundle;
    fn get_bundle() -> Self::B;
}

impl<T: StaticComponentsAspect> ComponentsAspect for T {
    fn set_components<'r>(
        &self,
        _context: &<Self::Context as super::prelude::NodeContext>::Wrapper<'r>,
        commands: &mut impl super::prelude::ComponentCommands,
        event: super::prelude::SetComponentsEvent,
    ) {
        if event == SetComponentsEvent::Created {
            commands.insert(Self::get_bundle());
        }
    }
}
