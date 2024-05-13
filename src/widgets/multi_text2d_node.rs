use bevy::{
    sprite::Anchor,
    text::{Text2dBounds, TextLayoutInfo},
};

pub use crate::prelude::*;

/// A text node in 2d space.
/// Note that you will need to attach a transform as well
#[derive(Debug, Clone)]
pub struct MultiText2DNode<
    const SECTIONS: usize,
    T: core::fmt::Display + PartialEq + Clone + Send + Sync + 'static,
> {
    pub sections: [Option<TextSectionData<T>>; SECTIONS],
    pub justify_text: JustifyText,
    pub linebreak_behavior: bevy::text::BreakLineOn,
    pub text_anchor: Anchor,
    pub text_2d_bounds: Text2dBounds,
}

impl<const SECTIONS: usize, T: core::fmt::Display + PartialEq + Clone + Send + Sync + 'static>
    PartialEq for MultiText2DNode<SECTIONS, T>
{
    fn eq(&self, other: &Self) -> bool {
        self.sections == other.sections
            && self.justify_text == other.justify_text
            && self.linebreak_behavior == other.linebreak_behavior
            && self.text_anchor == other.text_anchor
            && text_2d_bound_compare(&self.text_2d_bounds, &other.text_2d_bounds)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextSectionData<T: core::fmt::Display + PartialEq + Clone + Send + Sync + 'static> {
    pub text: T,
    pub font: &'static str,
    pub font_size: f32,
    pub color: Color,
}

fn text_2d_bound_compare(l: &Text2dBounds, r: &Text2dBounds) -> bool {
    l.size == r.size
}

impl<const SECTIONS: usize, T: core::fmt::Display + PartialEq + Clone + Send + Sync + 'static>
    MavericNode for MultiText2DNode<SECTIONS, T>
{
    type Context = ();

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.insert_static_bundle((SpatialBundle::default(), TextLayoutInfo::default()));
        commands.node_to_bundle(|x| &x.text_anchor);
        commands.node_to_component(|x| &x.text_2d_bounds, text_2d_bound_compare);

        commands.scope(|commands| {
            commands
                .advanced(|args, commands| {
                    if args.is_hot() {
                        let node = args.node;
                        let server: &AssetServer = commands
                            .get_res_untracked()
                            .expect("Could not get asset server");

                        let mut bundle = Text::default().with_justify(node.justify_text);
                        bundle.linebreak_behavior = node.linebreak_behavior;

                        for section in node.sections.iter().flatten() {
                            let font = server.load(section.font);

                            let style = TextStyle {
                                font,
                                font_size: section.font_size,
                                color: section.color,
                            };

                            bundle.sections.push(TextSection {
                                value: section.text.to_string(),
                                style,
                            });
                        }

                        commands.insert(bundle);
                    }
                })
                .finish();
        });
    }

    fn set_children<R: MavericRoot>(_commands: SetChildrenCommands<Self, Self::Context, R>) {}
}
