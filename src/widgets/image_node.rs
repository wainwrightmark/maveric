use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

use super::get_or_load_asset;

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

impl HierarchyNode for ImageNode {
    type Context = AssetServer;


    fn set_components<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world,>(commands: SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world,Self, Self::Context>)-> SetComponentsFinishToken<'w,'s,'a,'world> {
        commands.insert_with_args_and_context(|args, context| {
            let texture: Handle<Image> = get_or_load_asset(args.path, context);

            let bundle = ImageBundle {
                style: args.image_node_style.style.clone(),
                background_color: BackgroundColor(args.image_node_style.background_color),
                image: UiImage {
                    texture,
                    flip_x: false,
                    flip_y: false,
                },
                ..default()
            };
            bundle
        }).finish()
    }

    fn set_children< R: HierarchyRoot>(
        commands: SetChildrenCommands<Self, Self::Context, R>
    ) {
    }
}
