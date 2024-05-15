use crate::has_changed::HasChanged;

pub trait MavericContext {
    type Wrapper<'w, 's>: HasChanged;
}

impl MavericContext for () {
    type Wrapper<'w, 's> = ();
}

macro_rules! impl_nc_tuples {
    ($($T:tt $t:tt ),+) => {

        #[allow(clippy::many_single_char_names)]
        impl<$($T,)+> MavericContext for ($($T,)+)
        where
            $($T: MavericContext,)+
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
