use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

use super::get_or_load_asset;

#[derive(PartialEq, Debug, Default)]
pub struct ButtonNodeStyle {
    pub style: Style,
    pub visibility: Visibility,
    pub border_color: Color,
    pub background_color: Color,
}

#[derive(PartialEq, Debug)]
pub struct ButtonNode<Marker: Component + PartialEq + Clone> {
    pub text: Option<(String, Arc<TextNodeStyle>)>,

    pub image_handle: Option<&'static str>,
    pub button_node_style: Arc<ButtonNodeStyle>,

    pub marker: Marker,
}

impl<Marker: Component + PartialEq + Clone> HasContext for ButtonNode<Marker> {
    type Context = AssetServer;
}

impl<Marker: Component + PartialEq + Clone> ChildrenAspect for ButtonNode<Marker> {
    fn set_children<'r>(
        &self,
        _previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    ) {
        if let Some((text, style)) = &self.text {
            commands.add_child(
                0,
                TextNode {
                    style: style.clone(),
                    text: text.clone(),
                },
                context,
            )
        }
    }
}

impl<Marker: Component + PartialEq + Clone> ComponentsAspect for ButtonNode<Marker> {
    fn set_components<'r>(
        &self,
        _previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        event: SetComponentsEvent,
    ) {
        let image: UiImage = if let Some(path) = self.image_handle {
            let texture: Handle<Image> = get_or_load_asset(path, context);
            UiImage {
                texture,
                flip_x: false,
                flip_y: false,
            }
        } else {
            UiImage::default()
        };

        commands.insert(ButtonBundle {
            style: self.button_node_style.style.clone(),
            border_color: BorderColor(self.button_node_style.border_color),
            background_color: BackgroundColor(self.button_node_style.background_color),
            image,
            ..default()
        });

        if event == SetComponentsEvent::Created {
            commands.insert(self.marker.clone());
        }
    }
}
