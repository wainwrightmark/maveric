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

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.scope(|commands| {
            commands
                .ignore_args()
                .ignore_context()
                .insert(ImageBundle::default())
        });

        commands.scope(|commands| {
            commands
                .map_args(|x| &x.path)
                .insert_with_args_and_context(|path, server| {
                    let texture = get_or_load_asset::<Image>(*path, server);
                    UiImage {
                        texture,
                        flip_x: false,
                        flip_y: false,
                    }
                })
        });

        commands.scope(|commands| {
            commands
                .ignore_context()
                .map_args(|x| &x.style)
                .insert_bundle()
        });
        commands
            .ignore_context()
            .map_args(|x| &x.background_color)
            .insert_with_args(|color| BackgroundColor(*color));
    }

    fn set_children<R: MavericRoot>(_commands: SetChildrenCommands<Self, Self::Context, R>) {}
}
