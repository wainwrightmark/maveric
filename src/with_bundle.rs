use crate::{has_changed::HasChanged, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithBundle<N: MavericNode, B: IntoBundle + PartialEq> {
    pub node: N,
    pub bundle: B,
}

impl<N: MavericNode, B: IntoBundle + PartialEq> MavericNode for WithBundle<N, B> {
    type Context<'w, 's> = N::Context<'w, 's>;
    fn on_changed(
        &self,
        previous: &Self,
        context: &Self::Context<'_, '_>,
        world: &World,
        entity_commands: &mut bevy::ecs::system::EntityCommands,
    ) {
        if context.has_changed() | (!self.node.eq(&previous.node)) {
            N::on_changed(&self.node, &previous.node, context, world, entity_commands);
        }
    }

    fn on_created(
        &self,
        context: &Self::Context<'_, '_>,
        world: &World,
        entity_commands: &mut bevy::ecs::system::EntityCommands,
    ) {
        N::on_created(&self.node, context, world, entity_commands);
    }

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context<'_, '_>>) {
        commands.scope(|commands| {
            let commands = commands.map_node(|x| &x.node);
            N::set_components(commands);
        });

        commands
            .ignore_context()
            .map_node(|x| &x.bundle)
            .insert_bundle()
            .finish();
    }

    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context<'_, '_>, R>) {
        N::set_children(commands.map_args(|x| &x.node));
    }

    fn on_deleted(&self, commands: &mut ComponentCommands) -> DeletionPolicy {
        self.node.on_deleted(commands)
    }

    fn should_recreate(&self, previous: &Self, context: &Self::Context<'_, '_>) -> bool {
        self.node.should_recreate(&previous.node, context)
    }
}

pub trait CanWithBundle: MavericNode {
    fn with_bundle<B: IntoBundle + PartialEq>(self, bundle: B) -> WithBundle<Self, B> {
        WithBundle { node: self, bundle }
    }
}

impl<T: MavericNode> CanWithBundle for T {}
