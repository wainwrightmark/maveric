pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug)]
pub struct TextNode {
    text: String,
    style: TextNodeStyle,
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
        context: &<Self::Context as NodeContext>::Ref<'r>,
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

impl HierarchyNode for TextNode {
    type ComponentsAspect = Self;

    type AncestorAspect = ();

    fn components_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ComponentsAspect as NodeBase>::Context as NodeContext>::Wrapper<'r> {
        context
    }

    fn ancestor_context<'a, 'r>(
        _context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::AncestorAspect as NodeBase>::Context as NodeContext>::Wrapper<'r> {
        &()
    }

    fn as_component_aspect<'a>(&'a self) -> &'a Self::ComponentsAspect {
        self
    }

    fn as_ancestor_aspect<'a>(&'a self) -> &'a Self::AncestorAspect {
        &()
    }
}
