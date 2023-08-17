use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug)]
pub struct ImageNode {
    pub path: &'static str,
    pub image_node_style: Arc<ImageNodeStyle>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ImageNodeStyle {
    pub background_color: Color,
    pub style: Style,
}

impl HasContext for ImageNode {
    type Context = AssetServer;
}

impl ComponentsAspect for ImageNode {
    fn set_components<'r>(
        &self,
        _previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        _event: SetComponentsEvent,
    ) {
        let texture = context.load(self.path);

        let bundle = ImageBundle {

            style: self.image_node_style.style.clone(),
            background_color: BackgroundColor(self.image_node_style.background_color),
            image: UiImage { texture, flip_x: false, flip_y: false },
            ..default()
        };
        commands.insert(bundle);
    }
}

impl HasNoChildren for ImageNode {}
