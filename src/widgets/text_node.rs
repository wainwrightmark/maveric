pub use crate::prelude::*;
pub use bevy::prelude::*;

use super::get_or_load_asset;

#[derive(PartialEq, Debug, Clone)]
pub struct TextNode<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> {
    pub text: T,
    pub font: &'static str,
    pub font_size: f32,
    pub color: Color,
    pub alignment: TextAlignment,
    pub linebreak_behavior: bevy::text::BreakLineOn,
}

impl<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> MavericNode for TextNode<T> {
    type Context = AssetServer;

    fn set<R: MavericRoot>(
        mut data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        data.clone()
            .ignore_args()
            .ignore_context()
            .insert(commands, TextBundle::default());

        data.insert_with_args_and_context(commands, |args, server| {
            let font = get_or_load_asset(args.font, &server);
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
}
