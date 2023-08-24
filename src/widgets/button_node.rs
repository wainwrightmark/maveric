pub use crate::prelude::*;
pub use bevy::prelude::*;

#[derive(PartialEq, Debug, Default, Clone)]
pub struct ButtonNode<Marker: IntoBundle, S: IntoBundle<B = Style>, C : ChildTuple> {
    pub style: S,
    pub visibility: Visibility,
    pub border_color: Color,
    pub background_color: Color,
    pub marker: Marker,
    pub children: C

}

impl<Marker: IntoBundle, S: IntoBundle<B = Style>, C: ChildTuple> MavericNode for ButtonNode<Marker, S, C> {
    type Context = C::Context;

    fn set<R: MavericRoot>(data: NodeData<Self, Self::Context, R, true>, commands: &mut NodeCommands) {


        data.clone().ignore_args().ignore_context().insert(commands, ButtonBundle::default());

        data.clone().ignore_context().map_args(|x|&x.style).insert_bundle(commands);
        data.clone().ignore_context().map_args(|x|&x.visibility).insert_bundle(commands);
        data.clone().ignore_context().map_args(|x|&x.marker).insert_bundle(commands);


        data.clone().ignore_context().map_args(|x|&x.background_color).insert_with_args(commands, |color| BackgroundColor(*color));
        data.clone().ignore_context().map_args(|x|&x.border_color).insert_with_args(commands, |color| BorderColor(*color));

        data.map_args(|x|&x.children).add_children(commands);
    }


}