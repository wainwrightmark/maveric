pub use crate::prelude::*;

pub trait ChildTuple: PartialEq + Send + Sync + 'static {
    type Context<'w, 's>: MavericContext;

    fn add_children(&self, context: &Self::Context<'_, '_>, commands: &mut impl ChildCommands);
}

impl<T0: Clone + MavericNode> ChildTuple for (T0,) {
    type Context<'w, 's> = T0::Context<'w, 's>;
    fn add_children(&self, context: &Self::Context<'_, '_>, commands: &mut impl ChildCommands) {
        commands.add_child(0, self.0.clone(), context);
    }
}

macro_rules! impl_child_tuple {
    ($T0:ident, $(($T:ident, $idx:tt)),*) => {
        impl<$T0: Clone + MavericNode, $($T :Clone + MavericNode<Context<'static, 'static> = $T0::Context<'static, 'static>>,)*> ChildTuple for ($T0, $($T,)*){
            type Context<'w, 's> = $T0::Context<'w, 's>;

            fn add_children<'a, 'b, 'c, 'w, 's>(
                &'a self,
                context: &'b Self::Context<'w, 's>,
                commands: &'c mut impl ChildCommands,
            ) {
                commands.add_child(0, self.0.clone(), &context);

                //Safety: the lifetime is extended to static but it can only be used up to 'b
                unsafe {
                    let extended_context: &'b Self::Context<'static, 'static> =
                        std::mem::transmute(context);

                        $(
                            commands.add_child($idx, self.$idx.clone(), &extended_context);
                        )*
                }
            }
        }
    };
}

impl_child_tuple!(T0, (T1, 1));
impl_child_tuple!(T0, (T1, 1), (T2, 2));
impl_child_tuple!(T0, (T1, 1), (T2, 2), (T3, 3));
