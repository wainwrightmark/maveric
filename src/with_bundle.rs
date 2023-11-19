use crate::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct WithBundle<N: MavericNode, B: IntoBundle + PartialEq>{
    pub node: N,
    pub bundle: B
}

impl<N: MavericNode, B: IntoBundle + PartialEq> MavericNode for WithBundle<N, B> {
    type Context = N::Context;

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.scope(|commands|{
            let commands = commands.map_args(|x|&x.node);
            N::set_components(commands)
        });

        commands.ignore_context().map_args(|x|&x.bundle).insert_bundle().finish()
    }

    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context, R>) {
        N::set_children(commands.map_args(|x|&x.node));
    }
}

