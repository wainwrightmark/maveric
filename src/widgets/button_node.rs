use std::sync::Arc;

pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug, Default)]
pub struct ButtonNodeStyle {
    pub style: Style,
    pub visibility: Visibility,
    pub border_color: Color,
    pub background_color: Color,
}

#[derive(PartialEq, Debug)]
pub struct ButtonNode<Marker: Component + PartialEq + Clone> {
    //TODO refactor
    pub children: (
        Option<(String, Arc<TextNodeStyle>)>,
        Option<(&'static str, Arc<ImageNodeStyle>)>,
    ),
    pub button_node_style: Arc<ButtonNodeStyle>,

    pub marker: Marker,
}

impl<Marker: Component + PartialEq + Clone> HierarchyNode for ButtonNode<Marker> {
    type Context = AssetServer;

    fn set<R: HierarchyRoot>(
        data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        data.clone()
            .ignore_context()
            .map_args(|x| &x.button_node_style)
            .insert_with_args(commands, |args| {
                ButtonBundle {
                    style: args.style.clone(),
                    border_color: BorderColor(args.border_color),
                    background_color: BackgroundColor(args.background_color),
                    //image,
                    ..default()
                }
            });

        data.clone()
            .ignore_context()
            .map_args(|x| &x.marker)
            .insert_with_args(commands, |a| a.clone());

        data.map_args(move |x| &(x.children))
            .ordered_children_with_args_and_context(commands,|a, c, cc| {
                if let Some((text, style)) = &a.0 {
                    cc.add_child(
                        0,
                        TextNode {
                            style: style.clone(),
                            text: text.clone(),
                        },
                        c,
                    )
                }

                if let Some((path, image_node_style)) = &a.1 {
                    //let texture: Handle<Image> = get_or_load_asset(path, context);

                    cc.add_child(
                        1,
                        ImageNode {
                            path,
                            image_node_style: image_node_style.clone(),
                        },
                        c,
                    );
                };
            });
    }
}
