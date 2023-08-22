use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

use super::get_or_load_asset;

#[derive(PartialEq, Debug)]
pub struct TextNode {
    pub text: String,
    pub style: Arc<TextNodeStyle>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TextNodeStyle {
    pub font_size: f32,
    pub color: Color,
    pub alignment: TextAlignment,
    pub font: &'static str,
    pub linebreak_behavior: bevy::text::BreakLineOn,
}

impl HierarchyNode for TextNode {
    type Context = AssetServer;

    fn set<R: HierarchyRoot>(
        mut data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        data.insert_with_args_and_context(commands, |args, context| {
            let font = get_or_load_asset(args.style.font, &context);

            let mut bundle = TextBundle::from_section(
                args.text.clone(),
                TextStyle {
                    font,
                    font_size: args.style.font_size,
                    color: args.style.color,
                },
            )
            .with_text_alignment(args.style.alignment);

            bundle.text.linebreak_behavior = args.style.linebreak_behavior;

            bundle
        })
    }
}
