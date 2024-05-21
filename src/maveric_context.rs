use bevy::ecs::system::{ReadOnlySystemParam, SystemParam};

use crate::has_changed::HasChanged;

pub trait MavericContext: ReadOnlySystemParam + HasChanged {
    fn has_item_changed<'a, 'w, 's>(item: &'a <Self as SystemParam>::Item<'w, 's>) -> bool;
}

impl<R: ReadOnlySystemParam + HasChanged> MavericContext for R
where
    <R as SystemParam>::Item<'static, 'static>: HasChanged,
{
    fn has_item_changed<'a, 'w, 's>(item: &'a <Self as SystemParam>::Item<'w, 's>) -> bool {
        unsafe {
            let transmuted_item = std::mem::transmute_copy::<
                <Self as SystemParam>::Item<'w, 's>,
                <Self as SystemParam>::Item<'static, 'static>,
            >(&item);

            transmuted_item.has_changed()
        }
    }
}
