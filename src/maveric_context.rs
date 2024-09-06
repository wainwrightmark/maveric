use bevy::ecs::system::{ReadOnlySystemParam, SystemParam};

use crate::has_changed::HasChanged;

pub trait MavericContext: ReadOnlySystemParam + HasChanged {
    fn has_item_changed(item: &<Self as SystemParam>::Item<'_, '_>) -> bool;
}

impl<R: ReadOnlySystemParam + HasChanged> MavericContext for R
where
    <R as SystemParam>::Item<'static, 'static>: HasChanged + 'static,
{
    fn has_item_changed<'a, 'w, 's>(item: &'a <Self as SystemParam>::Item<'w, 's>) -> bool {
        unsafe {

            let ti = std::mem::transmute::<
            &'a <Self as SystemParam>::Item<'w, 's>,
                &'static <Self as SystemParam>::Item<'static, 'static>,
            >(item);

            ti.has_changed()
        }
    }
}


