use bevy::ecs::{
    change_detection::DetectChanges,
    system::{NonSend, NonSendMut, Res, ResMut, Resource},
    world::{Mut, Ref},
};

pub trait HasChanged {
    fn has_changed(&self) -> bool;
}

impl HasChanged for () {
    fn has_changed(&self) -> bool {
        false
    }
}

impl<'a, T: Resource> HasChanged for Res<'a, T> {
    fn has_changed(&self) -> bool {
        self.is_changed()
    }
}

impl<'a, T: Resource> HasChanged for ResMut<'a, T> {
    fn has_changed(&self) -> bool {
        self.is_changed()
    }
}

impl<'a, T: Resource> HasChanged for Ref<'a, T> {
    fn has_changed(&self) -> bool {
        self.is_changed()
    }
}

impl<'a, T: Resource> HasChanged for Mut<'a, T> {
    fn has_changed(&self) -> bool {
        self.is_changed()
    }
}

impl<'a, T: Resource> HasChanged for NonSend<'a, T> {
    fn has_changed(&self) -> bool {
        self.is_changed()
    }
}

impl<'a, T: Resource> HasChanged for NonSendMut<'a, T> {
    fn has_changed(&self) -> bool {
        self.is_changed()
    }
}

macro_rules! impl_has_changed_tuples {
    ($($T:tt $t:tt ),+) => {

        #[allow(clippy::many_single_char_names)]
        impl<$($T,)+> HasChanged for ($($T,)+)
        where
            $($T: HasChanged,)+
         {

            #[allow(clippy::many_single_char_names)]
            fn has_changed(&self) -> bool {
                let ($($t,)*) = self;
                $($T::has_changed($t) ||)* false
            }
        }


    };
}

impl_has_changed_tuples!(A a);
impl_has_changed_tuples!(A a, B b);
impl_has_changed_tuples!(A a, B b, C c);
impl_has_changed_tuples!(A a, B b, C c, D d);
impl_has_changed_tuples!(A a, B b, C c, D d, E e);
impl_has_changed_tuples!(A a, B b, C c, D d, E e, F f);
impl_has_changed_tuples!(A a, B b, C c, D d, E e, F f, G g);
impl_has_changed_tuples!(A a, B b, C c, D d, E e, F f, G g, H h);
impl_has_changed_tuples!(A a, B b, C c, D d, E e, F f, G g, H h, I i);
impl_has_changed_tuples!(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j);
impl_has_changed_tuples!(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k);
impl_has_changed_tuples!(A a, B b, C c, D d, E e, F f, G g, H h, I i, J j, K k, L l);
