use bevy::{
    sprite::Anchor,
    text::{Text2dBounds, TextLayoutInfo},
};

pub use crate::prelude::*;

/// A text node in 2d space.
/// Note that you will need to attach a transform as well
#[derive(Debug, Clone)]
pub struct Text2DNode<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> {
    pub text: T,
    pub font: &'static str,
    pub font_size: f32,
    pub color: Color,
    pub alignment: TextAlignment,
    pub linebreak_behavior: bevy::text::BreakLineOn,
    pub text_anchor: Anchor,
    pub text_2d_bounds: Text2dBounds,
}

impl<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> PartialEq for Text2DNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
            && self.font == other.font
            && self.font_size == other.font_size
            && self.color == other.color
            && self.alignment == other.alignment
            && self.linebreak_behavior == other.linebreak_behavior
            && anchor_compare(&self.text_anchor, &other.text_anchor)
            && text_2d_bound_compare(&self.text_2d_bounds, &other.text_2d_bounds)
    }
}

fn anchor_compare(l: &Anchor, r: &Anchor) -> bool {
    l.as_vec() == r.as_vec()
}

fn text_2d_bound_compare(l: &Text2dBounds, r: &Text2dBounds) -> bool {
    l.size == r.size
}

impl<T: Into<String> + PartialEq + Clone + Send + Sync + 'static> MavericNode for Text2DNode<T> {
    type Context = ();

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.insert_static_bundle((SpatialBundle::default(), TextLayoutInfo::default()));
        commands.node_to_component(|x| &x.text_anchor, anchor_compare);
        commands.node_to_component(|x| &x.text_2d_bounds, text_2d_bound_compare);



        commands.scope(|commands| {
            commands
                .advanced(|args, commands| {
                    if args.is_hot() {
                        let node = args.node;
                        let server: &AssetServer = commands
                            .get_res_untracked()
                            .expect("Could not get asset server");
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
                    }
                })
                .finish()
        });
    }

    fn set_children<R: MavericRoot>(_commands: SetChildrenCommands<Self, Self::Context, R>) {}
}
