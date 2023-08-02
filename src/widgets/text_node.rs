use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug)]
pub struct TextNode<V: Into<String> + Clone + PartialEq + Send + Sync + 'static> {
    pub value: V,
    pub style: Arc<TextNodeStyle>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TextNodeStyle {
    pub font_size: f32,
    pub color: Color,
    pub font: &'static str,
}

// impl NodeBase for TextNodeStyle {
//     type Context;

//     type Args;
// }

// impl<V: Into<String> + PartialEq + Clone + Send + Sync + 'static> HierarchyNode for TextNode<V> {
//     type Context = AssetServer;

//     fn set_components(
//         &self,
//         context: &Res<AssetServer>,
//         component_commands: &mut impl ComponentCommands,
//         event: SetComponentsEvent,
//     ) {
//         let font = context.load(self.style.font);

//         component_commands.insert(TextBundle::from_section(
//             self.value.clone(),
//             TextStyle {
//                 font,
//                 font_size: self.style.font_size,
//                 color: self.style.color,
//             },
//         ));
//     }

//     fn set_children<'r>(
//         &self,
//         context: &<Self::Context as NodeContext>::Wrapper<'r>,
//         commands: &mut impl ChildCommands,
//     ) {
//     }
// }
