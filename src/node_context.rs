use std::marker::PhantomData;

use bevy::prelude::*;

pub trait NodeContext {
    type Wrapper<'c>;

    fn has_changed<'c>(wrapper: &Self::Wrapper<'c>) -> bool;
}

impl<R: Resource> NodeContext for R {
    type Wrapper<'c> = Res<'c, R>;

    fn has_changed<'c>(wrapper: &'c Self::Wrapper<'c>) -> bool {
        DetectChanges::is_changed(wrapper)
    }
}

macro_rules! impl_nc {
    ($NC:ident, $(($T:ident, $t:ident)),*) => {
        pub struct $NC<$($T,)*>(PhantomData<($($T,)*)>);

        impl<$($T : NodeContext,)*> NodeContext for $NC<$($T,)*> {
            type Wrapper<'c> = ($($T::Wrapper<'c>,)*);

            fn has_changed<'c>(wrapper: &Self::Wrapper<'c>) -> bool {
                let ($($t,)*) = wrapper;
                $($T::has_changed($t) ||)* false
            }
        }


    };
}

impl_nc!(NC2, (T0, t0), (T1, t1));
impl_nc!(NC3, (T0, t0), (T1, t1), (T2, t2));
impl_nc!(NC4, (T0, t0), (T1, t1), (T2, t2), (T3, t3));

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NoContext;

impl NodeContext for NoContext {
    type Wrapper<'c> = ();

    fn has_changed<'c>(_wrapper: &Self::Wrapper<'c>) -> bool {
        false
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NoBundleContext;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NoChildrenContext;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SameContext;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DifferentContexts;

pub trait ContextType {}

impl ContextType for NoContext {}
impl ContextType for NoBundleContext {}
impl ContextType for NoChildrenContext {}
impl ContextType for SameContext {}
impl ContextType for DifferentContexts {}
