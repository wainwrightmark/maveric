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
    type Context = NoContext;

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.insert_static_bundle(TextBundle::default());

        commands.advanced(|args, commands| {
            let node = args.node;
            let server: &AssetServer = commands.get_res_untracked().expect("Could not get asset server");
            let font = server.load(node.font);
            let mut bundle = Text::from_section(
                node.text.clone(),
                TextStyle {
                    font,
                    font_size: node.font_size,
                    color: node.color,
                },
            )
            .with_alignment(node.alignment);

            bundle.linebreak_behavior = node.linebreak_behavior;
            commands.insert(bundle);
        });
    }

    fn set_children<R: MavericRoot>(_commands: SetChildrenCommands<Self, Self::Context, R>) {}
}