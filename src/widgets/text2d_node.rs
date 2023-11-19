pub use crate::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Text2DNode<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> {
    pub text: TextNode<T>, //TODO refactor,
    /// The transform of the text.
    pub transform: Transform,
}

impl<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> MavericNode for Text2DNode<T> {
    type Context = AssetServer;

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.scope(|commands| {
            commands
                .ignore_node()
                .ignore_context()
                .insert(Text2dBundle::default());
        });

        commands.scope(|commands| {
            commands
                .map_args(|x| &x.text)
                .insert_with_node_and_context(|args, server| {
                    let font = server.load(args.font);
                    let mut bundle = Text::from_section(
                        args.text.clone(),
                        TextStyle {
                            font,
                            font_size: args.font_size,
                            color: args.color,
                        },
                    )
                    .with_alignment(args.alignment);

                    bundle.linebreak_behavior = args.linebreak_behavior;
                    bundle
                })
                .finish()
        });

        commands
            .map_args(|x| &x.transform)
            .ignore_context()
            .insert_with_node(|args| args.clone())
            .finish();
    }

    fn set_children<R: MavericRoot>(_commands: SetChildrenCommands<Self, Self::Context, R>) {}
}

