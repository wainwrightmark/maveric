use std::marker::PhantomData;

use crate::prelude::*;

//TODO remove all this

#[derive(Debug)]
pub struct WithIgnoredContext<N: MavericNode<Context = NoContext>, C: NodeContext + 'static> {
    node: N,
    phantom: PhantomData<fn() -> C>,
}

impl<N: MavericNode<Context = NoContext>, C: NodeContext + 'static> MavericNode
    for WithIgnoredContext<N, C>
{
    type Context = C;

    fn set<R: MavericRoot>(
        data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        N::set(data.ignore_context().map_args(|x| &x.node), commands);
    }
}

impl<N: MavericNode<Context = NoContext>, C: NodeContext + 'static> PartialEq
    for WithIgnoredContext<N, C>
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.phantom == other.phantom
    }
}

pub trait CanIgnoreContext: MavericNode<Context = NoContext> {
    fn with_ignored_context<C: NodeContext>(self) -> WithIgnoredContext<Self, C> {
        WithIgnoredContext {
            node: self,
            phantom: PhantomData,
        }
    }
}

impl<T: MavericNode<Context = NoContext>> CanIgnoreContext for T {}

pub trait CanCoerceContext: MavericNode{
    fn with_coerced_context<C: NodeContext, const INDEX: usize>(self) -> WithCoercedContext<Self, C, INDEX>{
        WithCoercedContext { node: self, phantom: PhantomData }
    }
}

impl<T: MavericNode> CanCoerceContext for T {}

#[derive(Debug)]
pub struct WithCoercedContext<N: MavericNode, C: NodeContext + 'static, const INDEX: usize> {
    node: N,
    phantom: PhantomData<fn() -> C>,
}

impl<N: MavericNode, C: NodeContext, const INDEX: usize> PartialEq for WithCoercedContext<N, C, INDEX> {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.phantom == other.phantom
    }
}

macro_rules! impl_node_for_coerced {
    ($NC:ident;  $idx:tt;   $($PreC:ident),*; $T:ident; $($PostC:ident),*) => {
        impl<$($PreC: NodeContext,)*  $T: MavericNode, $($PostC: NodeContext,)*> MavericNode
            for WithCoercedContext<$T, $NC<$($PreC,)* $T::Context, $($PostC,)*>, $idx>
        {
            type Context = $NC<$($PreC,)* $T::Context, $($PostC,)*>;

            fn set<R: MavericRoot>(
                data: NodeData<Self, Self::Context, R, true>,
                commands: &mut NodeCommands,
            ) {
                let data = data.map_context(|x| &x.$idx).map_args(|x| &x.node);
                $T::set(data, commands)
            }
        }
    };
}

impl_node_for_coerced!(NC2;  0; ; T0;  C1 );
impl_node_for_coerced!(NC2;  1; C0 ; T1;   );


impl_node_for_coerced!(NC3;  0; ; T0;  C1, C2 );
impl_node_for_coerced!(NC3;  1; C0 ; T1; C2  );
impl_node_for_coerced!(NC3;  2; C0, C1 ; T2;   );

impl_node_for_coerced!(NC4;  0; ; T0;  C1, C2, C3 );
impl_node_for_coerced!(NC4;  1; C0 ; T1; C2, C3  );
impl_node_for_coerced!(NC4;  2; C0, C1 ; T2; C3  );
impl_node_for_coerced!(NC4;  3; C0, C1, C2 ; T3;);