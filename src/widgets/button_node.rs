pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug, Default, Clone)]
pub struct ButtonNode<Marker: IntoBundle, S: IntoBundle<B = Style>, C: ChildTuple> {
    pub style: S,
    pub visibility: Visibility,
    pub border_color: Color,
    pub background_color: Color,
    pub marker: Marker,
    pub children: C,
}

impl<Marker: IntoBundle, S: IntoBundle<B = Style>, C: ChildTuple> MavericNode
    for ButtonNode<Marker, S, C>
{
    type Context = C::Context;

    fn set_components(commands: SetComponentCommands<Self, Self::Context>) {
        let mut commands = commands.ignore_context();

        commands.insert_static_bundle(ButtonBundle::default());
        commands.node_to_bundle(|x| &x.style);
        commands.node_to_bundle(|x| &x.visibility);
        commands.node_to_bundle(|x| &x.marker);

        commands.scope(|commands| {
            commands
                .map_node(|x| &x.background_color)
                .insert_with_node(|color| BackgroundColor(*color));
        });
        commands.scope(|commands| {
            commands
                .map_node(|x| &x.border_color)
                .insert_with_node(|color| BorderColor(*color));
        });
    }

    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context, R>) {
        commands.map_args(|x| &x.children).add_children();
    }
}
