use crate::prelude::*;
use bevy::prelude::*;

pub trait IntoBundle: PartialEq + Clone + Send + Sync + Sized + 'static {
    type B: Bundle;

    fn into_bundle(self) -> Self::B;
}

impl<T: Bundle + PartialEq + Clone> IntoBundle for T {
    type B = Self;

    fn into_bundle(self) -> Self::B {
        self.clone()
    }
}

impl<T: IntoBundle> MavericNode for T {
    type Context = NoContext;

    fn set_components<R: MavericRoot>(data: NodeCommands<Self, Self::Context, R, false>) {
        data.ignore_context()
            .insert_with_args(|a| a.clone().into_bundle());
    }

    fn set_children<R: MavericRoot>(_commands: NodeCommands<Self, Self::Context, R, true>) {}
}
