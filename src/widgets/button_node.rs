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

    pub children: (Option<(String, Arc<TextNodeStyle>)>, Option<(&'static str, Arc<ImageNodeStyle>)>),
    pub button_node_style: Arc<ButtonNodeStyle>,

    pub marker: Marker,
}

impl<Marker: Component + PartialEq + Clone> HierarchyNode for ButtonNode<Marker> {
    type Context = AssetServer;

    fn set_components<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world>(
        commands: SetComponentCommands<'n, 'p, 'c1, 'c2, 'w, 's, 'a, 'world,Self, Self::Context>,
    ) -> SetComponentsFinishToken<'w, 's, 'a, 'world> {
        commands.scope(|commands| {
            commands
                .ignore_context()
                .map_args(|x| &x.button_node_style)
                .insert_with_args(|args| {
                    ButtonBundle {
                        style: args.style.clone(),
                        border_color: BorderColor(args.border_color),
                        background_color: BackgroundColor(args.background_color),
                        //image,
                        ..default()
                    }
                })
                .finish()
        })
            .ignore_context()
            .map_args(|x| &x.marker)
            .insert_with_args(|a| a.clone()).finish()
    }

    fn set_children<R: HierarchyRoot>(commands: SetChildrenCommands<Self, Self::Context, R>) {
        commands
            .map_args(move |x| &(x.children))
            .ordered_children_with_args_and_context(|a, c, cc| {
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
            })
    }

    // fn set_components<'r, R: HierarchyRoot>(mut commands: SetComponentCommands<Self, Self::Context, R>) {

    // }

    // fn set_children<'r, R: HierarchyRoot>(set_args: SetChildrenCommands<Self, Self::Context, R>) {

    // }
}
