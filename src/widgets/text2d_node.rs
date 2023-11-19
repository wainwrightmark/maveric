pub use crate::prelude::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Text2DNode<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> {
    pub text: TextNode<T>, //TODO refactor,
    /// The transform of the text.
    pub transform: Transform,
}

impl<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> MavericNode for Text2DNode<T> {
    type Context = NoContext;

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
                .advanced(|args, commands| {
                    let node = args.node;
                    let server: &AssetServer = commands.get_res_untracked().expect("Could not get asset server");
                    let font = server.load(node.font);
                    let mut bundle = Text::from_section(
                        node.text.clone(),
                        TextStyle {
                            font,
                            font_size: node.font_size,
                            color: node.color,
                        },
                    )
                    .with_alignment(node.alignment);

                    bundle.linebreak_behavior = node.linebreak_behavior;
                    commands.insert(bundle);
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

