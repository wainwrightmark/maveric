pub use crate::prelude::*;
pub use bevy::prelude::*;

/// A node that will draw an image in the UI tree.
/// For non-ui image, use SpriteNode
#[derive(PartialEq, Debug, Clone)]
pub struct ImageNode<S: IntoBundle<B = Style>> {
    pub path: &'static str,
    pub background_color: Color,
    pub style: S,
}

impl<S: IntoBundle<B = Style>> MavericNode for ImageNode<S> {
    type Context<'w, 's> = ();

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context<'_, '_>>) {
        commands.insert_static_bundle(ImageBundle::default());

        commands.scope(|commands| {
            commands.map_node(|x| &x.path).advanced(|args, commands| {
                let path = args.node;
                let server: &AssetServer = commands
                    .get_res_untracked()
                    .expect("Could not get asset server");
                let texture = server.load(*path);
                let bundle = UiImage {
                    texture,
                    flip_x: false,
                    flip_y: false,
                    color: Color::WHITE,
                };
                commands.insert(bundle);
            });
        });

        commands.node_to_bundle(|x| &x.style);

        commands
            .ignore_context()
            .map_node(|x| &x.background_color)
            .insert_with_node(|color| BackgroundColor(*color));
    }

    fn set_children<R: MavericRoot>(
        _commands: SetChildrenCommands<Self, Self::Context<'_, '_>, R>,
    ) {
    }
}
