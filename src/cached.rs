use std::{cell::OnceCell, iter, marker::PhantomData, sync::OnceLock};

use bevy::ecs::system::SystemParam;

use crate::prelude::*;

pub struct Cached<'w, 's, R: SystemParam + Send, T: Send + Sync> {
    data: std::sync::Arc<OnceLock<T>>,
    item: <R as SystemParam>::Item<'w, 's>,
    phantom: PhantomData<R>,
}

impl<'w, 's, R: SystemParam + Send, T: Send + Sync> Cached<'w, 's, R, T> {
    pub fn get_data<'a>(&'a self) -> &'a T
    where
        T: From<&'a R::Item<'w, 's>>,
    {
        let d = self.data.get_or_init(|| T::from(&self.item));
        d
    }
}

#[derive(Debug, Resource)]
struct CachedLazyCell<R, T: Send + Sync> {
    cell: std::sync::Arc<OnceLock<T>>,
    data: PhantomData<R>,
}

impl<R, T: Send + Sync> Default for CachedLazyCell<R, T> {
    fn default() -> Self {
        Self {
            cell: Default::default(),
            data: Default::default(),
        }
    }
}

unsafe impl<'w, 's, R: SystemParam + Send + Sync + 'static, T: Send + Sync + 'static> SystemParam
    for Cached<'w, 's, R, T>
where
    R::Item<'static, 'static>: DetectChanges,
{
    type State = R::State;

    type Item<'world, 'state> = Cached<'world, 'state, R, T>;

    fn init_state(
        world: &mut World,
        system_meta: &mut bevy::ecs::system::SystemMeta,
    ) -> Self::State {
        world.insert_resource(CachedLazyCell::<R, T>::default());

        R::init_state(world, system_meta)
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        change_tick: bevy::ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        let item: <R as SystemParam>::Item<'world, 'state> =
            R::get_param(state, system_meta, world, change_tick);

        // Super unsafe transmuting to extend lifetime (just to call is_changed)
        let fake_item = std::mem::transmute::<
            <R as SystemParam>::Item<'world, 'state>,
            <R as SystemParam>::Item<'static, 'static>,
        >(item);

        if fake_item.is_changed() {
            if let Some(mut r) = world.get_resource_mut::<CachedLazyCell<R, T>>() {
                *r = Default::default();
            }
        }

        let item: <R as SystemParam>::Item<'world, 'state> = std::mem::transmute(fake_item);

        let cell = world.get_resource::<CachedLazyCell<R, T>>().unwrap();

        Cached::<'world, 'state, R, T> {
            item,
            data: cell.cell.clone(),
            phantom: PhantomData,
        }
    }
}

//todo readonly system param
//todo detectchanges
//todo deref
//todo asref

#[cfg(test)]
pub mod tests {
    use std::sync::atomic::AtomicUsize;

    use bevy::prelude::*;

    use crate::cached::Cached;

    #[test]
    pub fn go() {
        #[derive(Debug, Resource, Default, PartialEq)]
        pub struct Counter1(usize);

        pub struct CounterDouble(usize);

        static TIMES_UPDATED: AtomicUsize = AtomicUsize::new(0);

        fn assert_times_updated(expected: usize){
            let v = TIMES_UPDATED.load(std::sync::atomic::Ordering::SeqCst);

            assert_eq!(v, expected);
        }

        impl<'a, 'w> From<&'a Res<'w, Counter1>> for CounterDouble {
            fn from(value: &'a Res<'w, Counter1>) -> Self {
                TIMES_UPDATED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Self(value.0 * 2)
            }
        }

        let mut app = App::new();

        app.init_resource::<Counter1>();

        app.add_systems(Update, check_count_and_double);

        fn check_count_and_double(
            count: Res<Counter1>,
            double: Cached<Res<Counter1>, CounterDouble>,
        ) {
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
            let mut r = app.world.resource_mut::<Counter1>();

            r.set_if_neq(Counter1(new_count));
        }
    }
}
