use bevy::prelude::*;

use crate::has_changed::HasChanged;

pub trait NodeContext {
    type Wrapper<'c>: HasChanged;
}

pub trait MavericContext {}

impl<R: Resource + MavericContext> NodeContext for R {
    type Wrapper<'c> = Res<'c, R>;
}

impl NodeContext for () {
    type Wrapper<'c> = ();
}

macro_rules! impl_nc_tuples {
    ($($T:tt $t:tt ),+) => {

        #[allow(clippy::many_single_char_names)]
        impl<$($T,)+> NodeContext for ($($T,)+)
        where
            $($T: NodeContext,)+
         {
            type Wrapper<'c> = ($($T::Wrapper<'c>,)*);
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
impl_nc_tuples!(A a, B b, C c, D d, E e, F f, G g, H h, I i);
impl_nc_tuples!(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j);
impl_nc_tuples!(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k);
impl_nc_tuples!(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l);
