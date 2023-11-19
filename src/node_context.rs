use std::marker::PhantomData;

use bevy::prelude::*;

pub trait NodeContext {
    type Wrapper<'c>;

    fn has_changed(wrapper: &Self::Wrapper<'_>) -> bool;
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

            fn has_changed(wrapper: &Self::Wrapper<'_>) -> bool {
                let ($($t,)*) = wrapper;
                $($T::has_changed($t) ||)* false
            }
        }


    };
}

impl_nc!(NC2, (T0, t0), (T1, t1));
impl_nc!(NC3, (T0, t0), (T1, t1), (T2, t2));
impl_nc!(NC4, (T0, t0), (T1, t1), (T2, t2), (T3, t3));

impl_nc!(NC5, (T0, t0), (T1, t1), (T2, t2), (T3, t3), (T4, t4));
impl_nc!(
    NC6,
    (T0, t0),
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5)
);
impl_nc!(
    NC7,
    (T0, t0),
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5),
    (T6, t6)
);
impl_nc!(
    NC8,
    (T0, t0),
    (T1, t1),
    (T2, t2),
    (T3, t3),
    (T4, t4),
    (T5, t5),
    (T6, t6),
    (T7, t7)
);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NoContext;

impl NodeContext for NoContext {
    type Wrapper<'c> = ();

    fn has_changed(_wrapper: &Self::Wrapper<'_>) -> bool {
        false
    }
}