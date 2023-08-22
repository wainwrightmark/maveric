pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug, Default, Clone)]
pub struct ButtonNode<Marker:Clone + IntoComponents<Context = NoContext>, S: Clone + IntoComponents<Context = NoContext, B = Style>> {
    pub style: S,
    pub visibility: Visibility,
    pub border_color: Color,
    pub background_color: Color,
    pub marker: Marker,

}

impl<Marker:Clone + IntoComponents<Context = NoContext>, S:Clone + IntoComponents<Context = NoContext, B = Style>> IntoComponents for ButtonNode<Marker, S> {
    type B = ButtonBundle;
    type Context = NoContext;

    fn set<R: HierarchyRoot>(
        data: NodeData<Self, Self::Context, R, false>,
        commands: &mut NodeCommands,
    ) {

        data.clone().ignore_args().insert(commands, ButtonBundle::default());

        data.clone().map_args(|x|&x.style).insert_components(commands);
        data.clone().map_args(|x|&x.visibility).insert_components(commands);
        data.clone().map_args(|x|&x.marker).insert_components(commands);


        data.clone().map_args(|x|&x.background_color).insert_with_args(commands, |color| BackgroundColor(*color));
        data.clone().map_args(|x|&x.border_color).insert_with_args(commands, |color| BorderColor(*color));
    }
}




// #[derive(PartialEq, Debug)]
// pub struct ButtonNode<Marker: Component + PartialEq + Clone> {
//     //TODO refactor
//     pub children: (
//         Option<(String, Arc<TextNodeStyle>)>,
//         Option<(&'static str, Arc<ImageNodeStyle>)>,
//     ),
//     pub button_node_style: Arc<ButtonNodeStyle>,

//     pub marker: Marker,
// }

// impl<Marker: Component + PartialEq + Clone> HierarchyNode for ButtonNode<Marker> {
//     type Context = AssetServer;

//     fn set<R: HierarchyRoot>(
//         data: NodeData<Self, Self::Context, R, true>,
//         commands: &mut NodeCommands,
//     ) {
//         data.clone()
//             .ignore_context()
//             .map_args(|x| &x.button_node_style)
//             .insert_with_args(commands, |args| {
//                 ButtonBundle {
//                     style: args.style.clone(),
//                     border_color: BorderColor(args.border_color),
//                     background_color: BackgroundColor(args.background_color),
//                     //image,
//                     ..default()
//                 }
//             });

//         data.clone()
//             .ignore_context()
//             .map_args(|x| &x.marker)
//             .insert_with_args(commands, |a| a.clone());

//         data.map_args(move |x| &(x.children))
//             .ordered_children_with_args_and_context(commands,|a, c, cc| {
//                 if let Some((text, style)) = &a.0 {
//                     cc.add_child(
//                         0,
//                         TextNode {
//                             style: style.clone(),
//                             text: text.clone(),
//                         },
//                         c,
//                     )
//                 }

//                 if let Some((path, image_node_style)) = &a.1 {
//                     //let texture: Handle<Image> = get_or_load_asset(path, context);

//                     cc.add_child(
//                         1,
//                         ImageNode {
//                             path,
//                             image_node_style: image_node_style.clone(),
//                         },
//                         c,
//                     );
//                 };
//             });
//     }
// }
