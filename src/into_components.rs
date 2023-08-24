use crate::prelude::*;
use bevy::prelude::*;

pub trait  IntoBundle: PartialEq + Clone + Send + Sync + Sized + 'static {
    type B: Bundle;

    fn into_bundle(self)-> Self::B;
}


impl<T: Bundle + PartialEq + Clone> IntoBundle for T {
    type B = Self;

    fn into_bundle(self)-> Self::B {
        self.clone()
    }
}

impl<T: IntoBundle> MavericNode for T {
    type Context = NoContext;

    fn set<R: MavericRoot>(
        data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        data.ignore_context().insert_with_args(commands, |a| a.clone().into_bundle());
    }
}
