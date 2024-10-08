use std::{
    ops::Deref,
    sync::{Arc, OnceLock},
};

use bevy::ecs::system::{ReadOnlySystemParam, SystemParam};

use crate::{has_changed::HasChanged, prelude::*};

pub trait CacheableResource: Send + Sync + 'static {
    type Argument<'world, 'state>: SystemParam + ReadOnlySystemParam;
    fn calculate<'w, 's>(arg: &<Self::Argument<'w, 's> as SystemParam>::Item<'w, 's>) -> Self;
}

pub struct Cached<'w, 's, T: CacheableResource> {
    data: Arc<OnceLock<T>>,
    item: <<T as CacheableResource>::Argument<'w, 's> as SystemParam>::Item<'w, 's>,
    previous_data: Arc<OnceLock<T>>,
}

impl<'w, 's, T: CacheableResource> Clone for Cached<'w, 's, T>
where
    <<T as CacheableResource>::Argument<'w, 's> as SystemParam>::Item<'w, 's>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            item: self.item.clone(),
            previous_data: self.previous_data.clone(),
        }
    }
}

impl<'w, 's, T: CacheableResource> HasChanged for Cached<'w, 's, T>
where
    <T::Argument<'w, 's> as SystemParam>::Item<'w, 's>: HasChanged,
    T: PartialEq,
{
    fn has_changed(&self) -> bool {
        //Logic:
        // If the item hasn't changed, then this has not changed
        // If the item has changed
        // If the previous cached value was not calculated, then this has changed
        // The the previous cached value was calculated, then compare the two values

        if !self.item.has_changed() {
            return false;
        }

        if let Some(prev) = self.previous_data.get() {
            let current = self.get_data();

            current != prev
        } else {
            return true;
        }
    }
}

impl<'w, 's, T: CacheableResource + std::fmt::Debug> std::fmt::Debug for Cached<'w, 's, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get_data().fmt(f)
    }
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
    fn get_data(&self) -> &T {
        let d = self.data.get_or_init(|| T::calculate(&self.item));
        d
    }
}

unsafe impl<'w, 's, T: CacheableResource> ReadOnlySystemParam for Cached<'w, 's, T> where
    <T::Argument<'static, 'static> as SystemParam>::Item<'static, 'static>: HasChanged
{
}

pub struct CachedState<T: CacheableResource> {
    pub(crate) inner_state: <T::Argument<'static, 'static> as SystemParam>::State,
    pub(crate) data: Arc<OnceLock<T>>,
}

unsafe impl<'w, 's, T: CacheableResource> SystemParam for Cached<'w, 's, T>
where
    <T::Argument<'static, 'static> as SystemParam>::Item<'static, 'static>: HasChanged,
{
    type State = CachedState<T>;
    type Item<'world, 'state> = Cached<'world, 'state, T>;

    fn init_state(
        world: &mut World,
        system_meta: &mut bevy::ecs::system::SystemMeta,
    ) -> Self::State {
        let inner_state =
            <T::Argument<'static, 'static> as SystemParam>::init_state(world, system_meta);

        CachedState {
            inner_state,
            data: Arc::new(OnceLock::new()),
        }
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        change_tick: bevy::ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        let item = <T::Argument<'static, 'static> as SystemParam>::get_param(
            &mut state.inner_state,
            system_meta,
            world,
            change_tick,
        );

        let item: <T::Argument<'static, 'static> as SystemParam>::Item<'static, 'static> =
            std::mem::transmute(item);

        let previous_data: Arc<OnceLock<T>> = state.data.clone();

        if item.has_changed() {
            state.data = Default::default();
        }

        let item: <T::Argument<'world, 'state> as SystemParam>::Item<'world, 'state> =
            std::mem::transmute(item);

        Cached::<'world, 'state, T> {
            item,
            data: state.data.clone(),
            previous_data,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::atomic::AtomicUsize;

    use bevy::prelude::*;

    use crate::{
        cached::{CacheableResource, Cached, CanRegisterMaveric},
        has_changed::HasChanged,
        root::MavericRoot,
    };

    #[test]
    pub fn test_in_maveric() {
        //This test is not passing at the moment (exit code: 0xc000001d, STATUS_ILLEGAL_INSTRUCTION)

        #[derive(Debug, PartialEq)]
        pub struct CounterDouble(usize);

        #[derive(Debug, Component, PartialEq, Clone)]
        pub struct CountComponent(usize);

        impl CacheableResource for CounterDouble {
            type Argument<'world, 'state> = Res<'world, Counter>;

            fn calculate<'w, 's>(
                arg: &<Self::Argument<'w, 's> as bevy::ecs::system::SystemParam>::Item<'w, 's>,
            ) -> Self {
                Self(arg.0 * 2)
            }
        }

        struct CounterView;

        impl MavericRoot for CounterView {
            type Context<'w, 's> = Cached<'w, 's, CounterDouble>;

            fn set_children(
                context: &<Self::Context<'_, '_> as bevy::ecs::system::SystemParam>::Item<'_, '_>,
                commands: &mut impl super::ChildCommands,
            ) {
                commands.add_child(0, CountComponent(context.0), &());
            }
        }

        let mut app = App::new();

        app.add_plugins(bevy::time::TimePlugin);

        app.init_resource::<Counter>();

        app.register_maveric::<CounterView>();

        app.update();

        assert_component_count(&mut app, 0);

        set_count(&mut app, 1);

        app.update();
        app.update();
        app.update();

        assert_component_count(&mut app, 2);

        app.update();

        fn assert_component_count(app: &mut App, expected: usize) {
            let world = app.world_mut();
            let mut query = world.query::<&CountComponent>();
            let components: Vec<_> = query.iter(&world).collect();

            assert_eq!(1, components.len(), "Should be exactly one count component");

            for count_component in components.iter() {
                assert_eq!(expected, count_component.0, "Expected component count to be {expected} but was {}", count_component.0);
            }
        }
    }

    #[test]
    pub fn test_with_one_arg() {
        pub struct CounterDouble(usize);

        static TIMES_UPDATED: AtomicUsize = AtomicUsize::new(0);

        fn assert_times_updated(expected: usize) {
            let v = TIMES_UPDATED.load(std::sync::atomic::Ordering::SeqCst);
            assert_eq!(v, expected);
        }

        impl CacheableResource for CounterDouble {
            type Argument<'world, 'state> = Res<'world, Counter>;

            fn calculate<'w, 's>(
                arg: &<Self::Argument<'w, 's> as bevy::ecs::system::SystemParam>::Item<'w, 's>,
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
    }
    fn set_count(app: &mut App, new_count: usize) {
        let mut r = app.world_mut().resource_mut::<Counter>();

        r.set_if_neq(Counter(new_count));
    }

    #[derive(Debug, Resource, Default, PartialEq, Eq)]
    pub struct Counter(usize);

    #[test]
    pub fn test_has_changed() {
        #[derive(PartialEq, Debug)]
        pub struct CounterDiv2(usize);

        static TIMES_CHANGED: AtomicUsize = AtomicUsize::new(0);

        fn assert_times_changed(expected: usize, message: &'static str) {
            let actual = TIMES_CHANGED.load(std::sync::atomic::Ordering::SeqCst);

            assert_eq!(
                actual, expected,
                "actual: {actual} expected: {expected}. {message}"
            );
        }

        impl CacheableResource for CounterDiv2 {
            type Argument<'world, 'state> = Res<'world, Counter>;

            fn calculate<'w, 's>(
                arg: &<Self::Argument<'w, 's> as bevy::ecs::system::SystemParam>::Item<'w, 's>,
            ) -> Self {
                Self(arg.0 / 2)
            }
        }

        let mut app = App::new();

        app.init_resource::<Counter>();

        app.add_systems(Update, check_count_and_half);

        fn check_count_and_half(count: Res<Counter>, half: Cached<CounterDiv2>) {
            let count = count.0;

            if half.has_changed() {
                TIMES_CHANGED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }

            let double = half.get_data().0;

            assert_eq!(count / 2, double);
        }

        assert_times_changed(0, "Initial value");
        app.update();
        assert_times_changed(1, "Has_Changed should be true on first system run"); //changed on initial value changing

        app.update();
        assert_times_changed(1, "The resource has not changed since the system last ran");

        set_count(&mut app, 1);
        app.update();
        assert_times_changed(
            1,
            "The cached value has not changed since the system last ran",
        );

        set_count(&mut app, 2);
        app.update();
        assert_times_changed(2, "The cached value has now changed");

        set_count(&mut app, 4);
        app.update();
        assert_times_changed(3, "The cached value has changed again");
    }

    #[test]
    pub fn test_with_two_args() {
        #[derive(Debug, Resource, Default, PartialEq, Eq)]
        pub struct Counter1(usize);
        #[derive(Debug, Resource, Default, PartialEq, Eq)]
        pub struct Counter2(usize);

        pub struct CounterMult(usize);

        static TIMES_UPDATED: AtomicUsize = AtomicUsize::new(0);

        fn assert_times_updated(expected: usize) {
            let v = TIMES_UPDATED.load(std::sync::atomic::Ordering::SeqCst);

            assert_eq!(v, expected);
        }

        impl CacheableResource for CounterMult {
            type Argument<'world, 'state> = (Res<'world, Counter1>, Res<'world, Counter2>);

            fn calculate<'w, 's>(
                arg: &<Self::Argument<'w, 's> as bevy::ecs::system::SystemParam>::Item<'w, 's>,
            ) -> Self {
                TIMES_UPDATED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                Self(arg.0 .0 * arg.1 .0)
            }
        }

        let mut app = App::new();

        app.init_resource::<Counter1>();
        app.init_resource::<Counter2>();

        app.add_systems(Update, check_count_and_multiple);

        fn check_count_and_multiple(
            count1: Res<Counter1>,
            count2: Res<Counter2>,
            cached_product: Cached<CounterMult>,
        ) {
            let product = count1.0 * count2.0;

            assert_eq!(product, cached_product.0);
        }

        assert_times_updated(0);
        app.update();
        assert_times_updated(1);

        set_counts(&mut app, 1, 1);

        app.update();
        assert_times_updated(2);
        app.update();
        assert_times_updated(2);
        set_counts(&mut app, 1, 1);

        app.update();

        assert_times_updated(2);

        set_counts(&mut app, 2, 2);
        set_counts(&mut app, 3, 3);

        app.update();

        assert_times_updated(3);

        set_counts(&mut app, 4, 3);

        app.update();

        assert_times_updated(4);

        set_counts(&mut app, 4, 4);

        app.update();

        assert_times_updated(5);

        fn set_counts(app: &mut App, new_count1: usize, new_count2: usize) {
            let mut counter1 = app.world_mut().resource_mut::<Counter1>();
            counter1.set_if_neq(Counter1(new_count1));
            let mut counter2 = app.world_mut().resource_mut::<Counter2>();

            counter2.set_if_neq(Counter2(new_count2));
        }
    }
}
