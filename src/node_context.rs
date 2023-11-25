use bevy::prelude::*;

pub trait NodeContext {
    type Wrapper<'c>;

    fn has_changed(wrapper: &Self::Wrapper<'_>) -> bool;
}

pub trait MavericContext {}

impl<R: Resource + MavericContext> NodeContext for R {
    type Wrapper<'c> = Res<'c, R>;

    fn has_changed<'c>(wrapper: &'c Self::Wrapper<'c>) -> bool {
        DetectChanges::is_changed(wrapper)
    }
}

impl NodeContext for () {
    type Wrapper<'c> = ();

    fn has_changed(_wrapper: &Self::Wrapper<'_>) -> bool {
        false
    }
}

macro_rules! impl_nc_tuples {
    ($($T:tt $t:tt ),+) => {


        impl<$($T,)+> NodeContext for ($($T,)+)
        where
            $($T: NodeContext,)+
         {
            type Wrapper<'c> = ($($T::Wrapper<'c>,)*);

            fn has_changed(wrapper: &Self::Wrapper<'_>) -> bool {
                let ($($t,)*) = wrapper;
                $($T::has_changed($t) ||)* false
            }
        }


    };
}

impl_nc_tuples!(A a);
impl_nc_tuples!(A a, B b);
impl_nc_tuples!(A a, B b, C c);
impl_nc_tuples!(A a, B b, C c, D d);
impl_nc_tuples!(A a, B b, C c, D d, E e);
impl_nc_tuples!(A a, B b, C c, D d, E e, F f);
impl_nc_tuples!(A a, B b, C c, D d, E e, F f, G g);
impl_nc_tuples!(A a, B b, C c, D d, E e, F f, G g, H h);
