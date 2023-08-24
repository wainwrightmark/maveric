pub use crate::prelude::*;

pub trait ChildTuple: PartialEq + Send + Sync + 'static {
    type Context: NodeContext;

    fn add_children<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    );
}

macro_rules! impl_child_tuple {
    ($T0:ident, $(($T:ident, $idx:tt)),*) => {
        impl<$T0: Clone + MavericNode, $($T :Clone + MavericNode<Context = $T0::Context>,)*> ChildTuple for ($T0, $($T,)*){
            type Context = $T0::Context;

            fn add_children<'r>(&self, context: &<Self::Context as NodeContext>::Wrapper<'r>, commands: &mut impl ChildCommands,) {
                commands.add_child(0, self.0.clone(), &context);

                $(
                    commands.add_child($idx, self.$idx.clone(), &context);
                )*

            }
        }
    };
}

impl_child_tuple!(T0,);
impl_child_tuple!(T0, (T1, 1));
impl_child_tuple!(T0, (T1, 1), (T2, 2));
impl_child_tuple!(T0, (T1, 1), (T2, 2), (T3, 3));

