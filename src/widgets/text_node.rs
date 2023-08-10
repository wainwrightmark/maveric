use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;
use bevy::text::BreakLineOn;

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
    pub linebreak_behavior: BreakLineOn
}

impl HasContext for TextNode {
    type Context = AssetServer;
}

impl ComponentsAspect for TextNode {
    fn set_components<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        _event: SetComponentsEvent,
    ) {
        let font = context.load(self.style.font);

        let mut bundle = TextBundle::from_section(
            self.text.clone(),
            TextStyle {
                font,
                font_size: self.style.font_size,
                color: self.style.color,
            },
        ).with_text_alignment(self.style.alignment);

        bundle.text.linebreak_behavior = self.style.linebreak_behavior;

        //TODO only update text and node components
        commands.insert(bundle);
    }
}

impl HasChildrenAspect for TextNode {
    type ChildrenAspect = ();

    fn children_context<'a, 'r>(
        _context: &'a <<Self as HasContext>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ChildrenAspect as HasContext>::Context as NodeContext>::Wrapper<'r> {
        &()
    }

    fn as_children_aspect<'a>(&'a self) -> &'a Self::ChildrenAspect {
        &()
    }
}
