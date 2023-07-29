use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

use self::text_node::{TextNode, TextNodeStyle};

#[derive(PartialEq, Debug)]
pub struct ButtonNode<Marker: Component + PartialEq + Clone> {
    pub text: String,
    pub text_node_style: Arc<TextNodeStyle>,
    pub button_node_style: Arc<ButtonNodeStyle>,

    pub marker: Marker,
}

#[derive(PartialEq, Debug)]
pub struct ButtonNodeStyle {
    pub style: Style,
    pub visibility: Visibility,
    pub border_color: Color,
    pub background_color: Color,
}

impl<M: Component + PartialEq + Clone> StateTreeNode for ButtonNode<M> {
    type Context<'c> = Res<'c, AssetServer>;

    fn get_components<'b>(
        &self,
        _context: &Self::Context<'b>,
        component_commands: &mut impl ComponentCommands,
    ) {
        component_commands.insert(ButtonBundle {
            style: self.button_node_style.style.clone(),
            border_color: BorderColor(self.button_node_style.border_color),
            background_color: BackgroundColor(self.button_node_style.background_color),
            ..default()
        });
        component_commands.insert(self.marker.clone());
    }

    fn get_children<'b>(
        &self,
        context: &Self::Context<'b>,
        child_commands: &mut impl ChildCommands,
    ) {
        child_commands.add(
            0,
            context,
            TextNode {
                style: self.text_node_style.clone(),
                text: self.text.clone()
            },
        )
    }

    fn on_deleted(&self,  _component_commands: &mut impl ComponentCommands) -> DeletionPolicy {
        DeletionPolicy::DeleteImmediately //You can override this by wrapping in `Animated`
    }
}
