pub use crate::prelude::*;
pub use bevy::prelude::*;

use super::get_or_load_asset;

#[derive(PartialEq, Debug, Clone)]
pub struct ImageNode<S: IntoBundle<B = Style>> {
    pub path: &'static str,
    pub background_color: Color,
    pub style: S,
}

impl<S: IntoBundle<B = Style>> MavericNode for ImageNode<S> {
    type Context = AssetServer;

    fn set<R: MavericRoot>(
        data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        data.clone()
            .ignore_args()
            .ignore_context()
            .insert(commands, ImageBundle::default());

        data.clone()
            .map_args(|x| &x.path)
            .insert_with_args_and_context(commands, |path, server| {
                let texture = get_or_load_asset::<Image>(*path, server);
                UiImage {
                    texture,
                    flip_x: false,
                    flip_y: false,
                }
            });

        data.clone()
            .ignore_context()
            .map_args(|x| &x.style)
            .insert_bundle(commands);
        data.clone()
            .ignore_context()
            .map_args(|x| &x.background_color)
            .insert_with_args(commands, |color| BackgroundColor(*color));
    }
}
