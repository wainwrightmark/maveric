use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug)]
pub struct TextNode {
    pub text: String,
    pub style: Arc<TextNodeStyle>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TextNodeStyle {
    pub font_size: f32,
    pub color: Color,
    pub font: &'static str,
}

impl NodeBase for TextNode {
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

        //TODO only update text and node components
        commands.insert(TextBundle::from_section(
            self.text.clone(),
            TextStyle {
                font,
                font_size: self.style.font_size,
                color: self.style.color,
            },
        ));
    }
}

impl HasChildrenAspect for TextNode {
    type ChildrenAspect = ();

    fn children_context<'a, 'r>(
        _context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ChildrenAspect as NodeBase>::Context as NodeContext>::Wrapper<'r> {
        &()
    }

    fn as_children_aspect<'a>(&'a self) -> &'a Self::ChildrenAspect {
        &()
    }
}
