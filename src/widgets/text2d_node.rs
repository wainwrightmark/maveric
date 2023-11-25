use bevy::{sprite::Anchor, text::TextLayoutInfo};

pub use crate::prelude::*;



/// A text node in 2d space.
/// Note that you will need to attach a transform as well
#[derive(PartialEq, Debug, Clone)]
pub struct Text2DNode<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> {
    pub text: T,
    pub font: &'static str,
    pub font_size: f32,
    pub color: Color,
    pub alignment: TextAlignment,
    pub linebreak_behavior: bevy::text::BreakLineOn,
}

impl<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> MavericNode for Text2DNode<T> {
    type Context = ();

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.scope(|commands| {
            commands
                .ignore_node()
                .ignore_context()
                .insert(Text2dBundle::default());
        });

        commands.insert_static_bundle((
            Anchor::default(),
            GlobalTransform::default(),
            VisibilityBundle::default(),
            TextLayoutInfo::default()
        ));

        commands.scope(|commands| {
            commands
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


    }

    fn set_children<R: MavericRoot>(_commands: SetChildrenCommands<Self, Self::Context, R>) {}
}

