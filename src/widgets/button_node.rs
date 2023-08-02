// use std::sync::Arc;

// pub use crate::prelude::*;
// pub use bevy::prelude::*;

// #[derive(PartialEq, Debug)]
// pub struct ButtonNode<
//     Marker: Component + PartialEq + Clone,
//     V: Into<String> + Clone + PartialEq + Send + Sync + 'static,
// > {
//     pub value: V,
//     pub text_node_style: Arc<TextNodeStyle>,
//     pub button_node_style: Arc<ButtonNodeStyle>,

//     pub marker: Marker,
// }

// #[derive(PartialEq, Debug, Default)]
// pub struct ButtonNodeStyle {
//     pub style: Style,
//     pub visibility: Visibility,
//     pub border_color: Color,
//     pub background_color: Color,
// }

// impl<
//         M: Component + PartialEq + Clone,
//         V: Into<String> + Clone + PartialEq + Send + Sync + 'static,
//     > HierarchyNode for ButtonNode<M, V>
// {
//     type Context = AssetServer;

//     fn set_components<'b>(
//         &self,
//         _context: &Res<AssetServer>,
//         commands: &mut impl ComponentCommands,
//         event: SetComponentsEvent,
//     ) {
//         commands.insert(ButtonBundle {
//             style: self.button_node_style.style.clone(),
//             border_color: BorderColor(self.button_node_style.border_color),
//             background_color: BackgroundColor(self.button_node_style.background_color),
//             ..default()
//         });
//         commands.insert(self.marker.clone());
//     }

//     fn set_children<'r>(
//         &self,
//         context: &<Self::Context as NodeContext>::Wrapper<'r>,
//         commands: &mut impl ChildCommands,
//     ) {
//         commands.add_child(
//             0,
//             context,
//             TextNode {
//                 style: self.text_node_style.clone(),
//                 value: self.value.clone(),
//             },
//         )
//     }
// }
