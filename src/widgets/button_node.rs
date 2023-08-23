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

    fn set<R: MavericRoot>(
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