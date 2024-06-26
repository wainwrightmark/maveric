use crate::prelude::*;
use bevy::prelude::*;

pub trait IntoBundle: PartialEq + Clone + Send + Sync + Sized + 'static {
    type B: Bundle;

    fn into_bundle(self) -> Self::B;
}

impl<T: Bundle + PartialEq + Clone> IntoBundle for T {
    type B = Self;

    fn into_bundle(self) -> Self::B {
        self
    }
}

impl<T: IntoBundle> MavericNode for T {
    type Context<'w, 's> = ();

    fn set_components(data: SetComponentCommands<Self, Self::Context<'_, '_>>) {
        data.ignore_context()
            .insert_with_node(|a| a.clone().into_bundle());
    }

    fn set_children<R: MavericRoot>(
        _commands: SetChildrenCommands<Self, Self::Context<'_, '_>, R>,
    ) {
    }
}
