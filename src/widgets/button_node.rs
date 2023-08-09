use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug, Default)]
pub struct ButtonNodeStyle {
    pub style: Style,
    pub visibility: Visibility,
    pub border_color: Color,
    pub background_color: Color,
}

#[derive(PartialEq, Debug)]
pub struct ButtonNode<Marker: Component + PartialEq + Clone> {
    pub text: String,
    pub text_node_style: Arc<TextNodeStyle>,
    pub button_node_style: Arc<ButtonNodeStyle>,

    pub marker: Marker,
}

impl<Marker: Component + PartialEq + Clone> NodeBase for ButtonNode<Marker> {
    type Context = AssetServer;
}

impl<Marker: Component + PartialEq + Clone> ChildrenAspect for ButtonNode<Marker> {
    fn set_children<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    ) {
        commands.add_child(
            0,
            TextNode {
                style: self.text_node_style.clone(),
                text: self.text.clone(),
            },
            context,
        )
    }
}

impl<Marker: Component + PartialEq + Clone> ComponentsAspect for ButtonNode<Marker> {
    fn set_components<'r>(
        &self,
        _context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        _event: SetComponentsEvent,
    ) {
        commands.insert(ButtonBundle {
            style: self.button_node_style.style.clone(),
            border_color: BorderColor(self.button_node_style.border_color),
            background_color: BackgroundColor(self.button_node_style.background_color),
            ..default()
        });
        commands.insert(self.marker.clone());
    }
}
