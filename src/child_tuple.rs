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

#[derive(Debug, PartialEq)]
pub struct NodeWithChildren<B: IntoComponents, C: ChildTuple, ContextType> {
    pub (crate) bundle: B,
    pub (crate) children: C,
    pub (crate) context_type: ContextType,
}


impl<B: IntoComponents, C: ChildTuple<Context = B::Context>> MavericNode
    for NodeWithChildren<B, C, SameContext>
{
    type Context = B::Context;

    fn set<R: MavericRoot>(
        data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        B::set(data.clone().map_args(|x| &x.bundle), commands);

        data.map_args(|x| &x.children)
            .unordered_children_with_args_and_context(commands, |args, context, commands| {
                args.add_children(context, commands)
            });
    }
}

impl<B: IntoComponents, C: ChildTuple> MavericNode for NodeWithChildren<B, C, DifferentContexts> {
    type Context = NC2<B::Context, C::Context>;

    fn set<R: MavericRoot>(
        data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        B::set(
            data.clone().map_context(|x| &x.0).map_args(|x| &x.bundle),
            commands,
        );

        data.map_args(|x| &x.children)
            .map_context::<C::Context>(|x| &x.1)
            .unordered_children_with_args_and_context(commands, |args, context, commands| {
                args.add_children(context, commands)
            });
    }
}

impl<B: IntoComponents<Context = NoContext>, C: ChildTuple> MavericNode
    for NodeWithChildren<B, C, NoBundleContext>
{
    type Context = C::Context;

    fn set<R: MavericRoot>(
        data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        B::set(
            data.clone().ignore_context().map_args(|x| &x.bundle),
            commands,
        );

        data.map_args(|x| &x.children)
            .unordered_children_with_args_and_context(commands, |args, context, commands| {
                args.add_children(context, commands)
            });
    }
}

impl<B: IntoComponents, C: ChildTuple<Context = NoContext>> MavericNode
    for NodeWithChildren<B, C, NoChildrenContext>
{
    type Context = B::Context;

    fn set<R: MavericRoot>(
        data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        B::set(data.clone().map_args(|x| &x.bundle), commands);

        data.ignore_context()
            .map_args(|x| &x.children)
            .unordered_children_with_args_and_context(commands, |args, context, commands| {
                args.add_children(context, commands)
            });
    }
}
