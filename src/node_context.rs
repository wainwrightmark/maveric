use bevy::prelude::*;

use crate::has_changed::HasChanged;

pub trait NodeContext {
    type Wrapper<'w,'s>: HasChanged;
}

pub trait MavericContext {}

impl<R: Resource + MavericContext> NodeContext for R {
    type Wrapper<'w, 's> = Res<'w, R>;
}

impl NodeContext for () {
    type Wrapper<'w, 's> = ();
}

macro_rules! impl_nc_tuples {
    ($($T:tt $t:tt ),+) => {

        #[allow(clippy::many_single_char_names)]
        impl<$($T,)+> NodeContext for ($($T,)+)
        where
            $($T: NodeContext,)+
         {
            type Wrapper<'w, 's> = ($($T::Wrapper<'w, 's>,)*);
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
