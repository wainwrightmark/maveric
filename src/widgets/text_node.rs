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

    fn set_components<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world,>(commands: SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world,Self, Self::Context>)-> SetComponentsFinishToken<'w,'s,'a,'world> {
        commands.insert_with_args_and_context(|args,  context| {
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
        }).finish()
    }

    fn set_children< R: HierarchyRoot>(
        commands: SetChildrenCommands<Self, Self::Context, R>
    ) {
    }




}
