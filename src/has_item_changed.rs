use bevy::ecs::{
    change_detection::DetectChanges,
    system::{NonSend, NonSendMut, Res, ResMut, Resource, SystemParam},
};

pub trait HasItemChanged: SystemParam {
    fn has_item_changed<'w, 's>(item: &Self::Item<'w, 's>) -> bool;
}

impl HasItemChanged for () {
    fn has_item_changed<'w, 's>(_item: &Self::Item<'w, 's>) -> bool {
        false
    }
}

impl<'a, T: Resource> HasItemChanged for Res<'a, T> {
    fn has_item_changed<'w, 's>(item: &Self::Item<'w, 's>) -> bool {
        item.is_changed()
    }
}

impl<'a, T: Resource> HasItemChanged for ResMut<'a, T> {
    fn has_item_changed<'w, 's>(item: &Self::Item<'w, 's>) -> bool {
        item.is_changed()
    }
}

impl<'a, T: Resource> HasItemChanged for NonSend<'a, T> {
    fn has_item_changed<'w, 's>(item: &Self::Item<'w, 's>) -> bool {
        item.is_changed()
    }
}

impl<'a, T: Resource> HasItemChanged for NonSendMut<'a, T> {
    fn has_item_changed<'w, 's>(item: &Self::Item<'w, 's>) -> bool {
        item.is_changed()
    }
}

macro_rules! impl_has_changed_tuples {
    ($($T:tt $t:tt ),+) => {

        #[allow(clippy::many_single_char_names)]
        impl<$($T,)+> HasItemChanged for ($($T,)+)
        where
            $($T: HasItemChanged,)+
         {

            #[allow(clippy::many_single_char_names)]
            fn has_item_changed<'w,'s>(item: &Self::Item<'w,'s>) -> bool {
                let ($($t,)*) = item;
                $($T::has_item_changed($t) ||)* false
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
