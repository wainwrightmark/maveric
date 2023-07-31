use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug)]
pub struct TextNode<V : Into<String> + Clone + PartialEq + Send + Sync + 'static> {
    pub value: V,
    pub style: Arc<TextNodeStyle>
}

#[derive(PartialEq, Debug, Clone)]
pub struct TextNodeStyle{

    pub font_size: f32,
    pub color: Color,
    pub font: &'static str,
}

impl<V: Into<String> + PartialEq + Clone + Send + Sync + 'static> HierarchyNode for TextNode<V> {
    type Context<'c> = Res<'c, AssetServer>;

    fn get_components<'c>(
        &self,
        context: &Self::Context<'c>,
        component_commands: &mut impl ComponentCommands,
    ) {
        let font =context.load(self.style.font);

        component_commands.insert(TextBundle::from_section(
            self.value.clone(),
            TextStyle {
                font,
                font_size: self.style.font_size,
                color: self.style.color,
            },
        ));
    }

    fn get_children<'c>(
        &self,
        _context: &Self::Context<'c>,
        _child_commands: &mut impl ChildCommands,
    ) {
    }

    fn on_deleted(&self, _component_commands: &mut impl ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately
    }
}