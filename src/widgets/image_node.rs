pub use crate::prelude::*;
pub use bevy::prelude::*;



#[derive(PartialEq, Debug, Clone)]
pub struct ImageNode<S: IntoBundle<B = Style>> {
    pub path: &'static str,
    pub background_color: Color,
    pub style: S,
}

impl<S: IntoBundle<B = Style>> MavericNode for ImageNode<S> {
    type Context = NoContext;

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.scope(|commands| {
            commands
                .ignore_node()
                .ignore_context()
                .insert(ImageBundle::default());
        });

        commands.scope(|commands| {
            commands
                .map_args(|x| &x.path)
                .advanced(|args, commands| {
                    let path = args.node;
                    let server: &AssetServer = commands.get_res_untracked().expect("Could not get asset server");
                    let texture = server.load(*path);
                    let bundle = UiImage {
                        texture,
                        flip_x: false,
                        flip_y: false,
                    };
                    commands.insert(bundle);
                });
        });

        commands.scope(|commands| {
            commands
                .ignore_context()
                .map_args(|x| &x.style)
                .insert_bundle().finish();
        });
        commands
            .ignore_context()
            .map_args(|x| &x.background_color)
            .insert_with_node(|color| BackgroundColor(*color));
    }

    fn set_children<R: MavericRoot>(_commands: SetChildrenCommands<Self, Self::Context, R>) {}
}
