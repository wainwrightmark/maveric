use std::{ops::Deref, sync::OnceLock};

use bevy::ecs::system::{ReadOnlySystemParam, SystemParam};

use crate::{has_changed::HasChanged, prelude::*};

pub trait CacheableResource: Send + Sync + 'static {
    type Argument<'world, 'state>: SystemParam + ReadOnlySystemParam;
    fn calculate<'a, 'w, 's>(
        arg: &'a <Self::Argument<'w, 's> as SystemParam>::Item<'w, 's>,
    ) -> Self;
}

pub struct Cached<'w, 's, T: CacheableResource> {
    data: std::sync::Arc<OnceLock<T>>,
    item: <<T as CacheableResource>::Argument<'w, 's> as SystemParam>::Item<'w, 's>,
}

impl<'w, 's, T: CacheableResource> Deref for Cached<'w, 's, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get_data()
    }
}

impl<'w, 's, T: CacheableResource> AsRef<T> for Cached<'w, 's, T> {
    fn as_ref(&self) -> &T {
        self.get_data()
    }
}

impl<'w, 's, T: CacheableResource> Cached<'w, 's, T> {
    fn get_data<'a>(&'a self) -> &'a T {
        let d = self.data.get_or_init(|| T::calculate(&self.item));
        d
    }
}

impl<'w, 's, T: CacheableResource> HasChanged for Cached<'w, 's, T>
where
    <T::Argument<'w, 's> as SystemParam>::Item<'w, 's>: HasChanged,
{
    fn has_changed(&self) -> bool {
        self.item.has_changed()
    }
}

unsafe impl<'w, 's, T: CacheableResource> ReadOnlySystemParam for Cached<'w, 's, T> where
    <T::Argument<'static, 'static> as SystemParam>::Item<'static, 'static>: HasChanged
{
}

#[derive(Debug, Resource)]
struct CachedLazyCell<T: CacheableResource>(std::sync::Arc<OnceLock<T>>);

impl<T: CacheableResource> Default for CachedLazyCell<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

unsafe impl<'w, 's, T: CacheableResource> SystemParam for Cached<'w, 's, T>
where
    <T::Argument<'static, 'static> as SystemParam>::Item<'static, 'static>: HasChanged,
{
    type State = <T::Argument<'static, 'static> as SystemParam>::State;
    type Item<'world, 'state> = Cached<'world, 'state, T>;

    fn init_state(
        world: &mut World,
        system_meta: &mut bevy::ecs::system::SystemMeta,
    ) -> Self::State {
        world.init_resource::<CachedLazyCell<T>>();

        <T::Argument<'static, 'static> as SystemParam>::init_state(world, system_meta)
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        change_tick: bevy::ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        let item = <T::Argument<'static, 'static> as SystemParam>::get_param(
            state,
            system_meta,
            world,
            change_tick,
        );

        let item: <T::Argument<'static, 'static> as SystemParam>::Item<'static, 'static> =
            std::mem::transmute(item);

        if item.has_changed() {
            if let Some(mut r) = world.get_resource_mut::<CachedLazyCell<T>>() {
                *r = Default::default();
            }
        }

        let item: <T::Argument<'world, 'state> as SystemParam>::Item<'world, 'state> =
            std::mem::transmute(item);

        let cell = world.get_resource::<CachedLazyCell<T>>().unwrap();

        Cached::<'world, 'state, T> {
            item,
            data: cell.0.clone(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::atomic::AtomicUsize;

    use bevy::prelude::*;

    use crate::cached::{CacheableResource, Cached};

    #[test]
    pub fn go() {
        #[derive(Debug, Resource, Default, PartialEq)]
        pub struct Counter(usize);

        pub struct CounterDouble(usize);

        static TIMES_UPDATED: AtomicUsize = AtomicUsize::new(0);

        fn assert_times_updated(expected: usize) {
            let v = TIMES_UPDATED.load(std::sync::atomic::Ordering::SeqCst);

            assert_eq!(v, expected);
        }

        impl CacheableResource for CounterDouble {
            type Argument<'world, 'state> = Res<'world, Counter>;

            fn calculate<'a, 'w, 's>(
                arg: &'a <Self::Argument<'w, 's> as bevy::ecs::system::SystemParam>::Item<'w, 's>,
            ) -> Self {
                TIMES_UPDATED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Self(arg.0 * 2)
            }
        }

        let mut app = App::new();

        app.init_resource::<Counter>();

        app.add_systems(Update, check_count_and_double);

        fn check_count_and_double(count: Res<Counter>, double: Cached<CounterDouble>) {
            let count = count.0;

            let double = double.get_data().0;

            assert_eq!(count * 2, double);
        }

        assert_times_updated(0);
        app.update();
        assert_times_updated(1);

        set_count(&mut app, 1);

        app.update();
        assert_times_updated(2);
        app.update();
        assert_times_updated(2);
        set_count(&mut app, 1);

        app.update();

        assert_times_updated(2);

        set_count(&mut app, 2);
        set_count(&mut app, 3);

        app.update();

        assert_times_updated(3);

        fn set_count(app: &mut App, new_count: usize) {
            let mut r = app.world.resource_mut::<Counter>();

            r.set_if_neq(Counter(new_count));
        }
    }
}
