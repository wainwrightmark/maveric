pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub struct TextNode<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> {
    pub text: T,
    pub font: &'static str,
    pub font_size: f32,
    pub color: Color,
    pub alignment: TextAlignment,
    pub linebreak_behavior: bevy::text::BreakLineOn,
} // TODO style


impl<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> MavericNode for TextNode<T> {
    type Context = AssetServer;

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.scope(|commands| {
            commands
                .ignore_node()
                .ignore_context()
                .insert(TextBundle::default());
        });

        commands.insert_with_node_and_context(|args, server| {
            let font = server.load(args.font);
            let mut bundle = Text::from_section(
                args.text.clone(),
                TextStyle {
                    font,
                    font_size: args.font_size,
                    color: args.color,
                },
            )
            .with_alignment(args.alignment);

            bundle.linebreak_behavior = args.linebreak_behavior;
            bundle
        });
    }

    fn set_children<R: MavericRoot>(_commands: SetChildrenCommands<Self, Self::Context, R>) {}
}