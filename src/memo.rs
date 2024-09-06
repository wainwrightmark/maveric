use crate::{has_changed::HasChanged, prelude::*};
use bevy::ecs::system::{ReadOnlySystemParam, SystemParam};
use std::ops::Deref;

/// A value that can be used inside a memo
pub trait MemoValue: PartialEq + Send + Sync + 'static {
    type Argument<'world, 'state>: SystemParam + ReadOnlySystemParam;
    fn calculate<'w, 's>(arg: &<Self::Argument<'w, 's> as SystemParam>::Item<'w, 's>) -> Self; //todo take previous value???
}

/// A memo contains a value that is derived from one or more other `SystemParam`s
/// It is recalculated every time one of those arguments changes but it is only marked as changed when the value changes
#[derive(Clone, PartialEq)]
pub struct Memo<'s, T: MemoValue> {
    data: &'s T,
    has_changed: bool,
}

impl<'s, T: MemoValue> HasChanged for Memo<'s, T> {
    fn has_changed(&self) -> bool {
        self.has_changed
    }
}

impl<'s, T: MemoValue + std::fmt::Debug> std::fmt::Debug for Memo<'s, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<'s, T: MemoValue> Deref for Memo<'s, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'s, T: MemoValue> AsRef<T> for Memo<'s, T> {
    fn as_ref(&self) -> &T {
        self.data
    }
}

unsafe impl<'s, T: MemoValue> ReadOnlySystemParam for Memo<'s, T> where
    <T::Argument<'static, 'static> as SystemParam>::Item<'static, 'static>: HasChanged
{
}

pub struct MemoState<T: MemoValue> {
    pub(crate) inner_state: <T::Argument<'static, 'static> as SystemParam>::State,
    pub(crate) data: Option<T>,
}

unsafe impl<'s, T: MemoValue> SystemParam for Memo<'s, T>
where
    <T::Argument<'static, 'static> as SystemParam>::Item<'static, 'static>: HasChanged,
{
    type State = MemoState<T>;
    type Item<'world, 'state> = Memo<'state, T>;

    fn init_state(
        world: &mut World,
        system_meta: &mut bevy::ecs::system::SystemMeta,
    ) -> Self::State {
        let inner_state =
            <T::Argument<'static, 'static> as SystemParam>::init_state(world, system_meta);

        MemoState {
            inner_state,
            data: None,
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

        let has_changed: bool;

        if let Some(prev_data) = &state.data {
            if item.has_changed() {
                let new_value = <T as MemoValue>::calculate(&item);

                if &new_value == prev_data {
                    has_changed = false;
                } else {
                    state.data = Some(new_value);
                    has_changed = true;
                }
            } else {
                has_changed = false;
            }
        } else {
            has_changed = true;
            let new_value = <T as MemoValue>::calculate(&item);
            state.data = Some(new_value);
        }

        Memo {
            data: state.data.as_ref().unwrap(),
            has_changed,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::atomic::AtomicUsize;

    use bevy::prelude::*;

    use crate::{
        has_changed::HasChanged,
        memo::{CanRegisterMaveric, Memo, MemoValue},
        root::MavericRoot,
    };

    #[test]
    pub fn test_in_maveric() {
        //This test is not passing at the moment (exit code: 0xc000001d, STATUS_ILLEGAL_INSTRUCTION)

        #[derive(Debug, PartialEq)]
        pub struct CounterDouble(usize);

        #[derive(Debug, Component, PartialEq, Clone)]
        pub struct CountComponent(usize);

        impl MemoValue for CounterDouble {
            type Argument<'world, 'state> = Res<'world, Counter>;

            fn calculate<'w, 's>(
                arg: &<Self::Argument<'w, 's> as bevy::ecs::system::SystemParam>::Item<'w, 's>,
            ) -> Self {
                Self(arg.0 * 2)
            }
        }

        struct CounterView;

        impl MavericRoot for CounterView {
            type Context<'w, 's> = Memo<'s, CounterDouble>;

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
                assert_eq!(
                    expected, count_component.0,
                    "Expected component count to be {expected} but was {}",
                    count_component.0
                );
            }
        }
    }

    #[test]
    pub fn test_with_one_arg() {
        #[derive(Debug, PartialEq)]
        pub struct CounterDouble(usize);

        static TIMES_UPDATED: AtomicUsize = AtomicUsize::new(0);

        fn assert_times_updated(expected: usize) {
            let v = TIMES_UPDATED.load(std::sync::atomic::Ordering::SeqCst);
            assert_eq!(v, expected);
        }

        impl MemoValue for CounterDouble {
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

        fn check_count_and_double(count: Res<Counter>, double: Memo<CounterDouble>) {
            let count = count.0;

            let double = double.data.0;

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

        impl MemoValue for CounterDiv2 {
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

        fn check_count_and_half(count: Res<Counter>, half: Memo<CounterDiv2>) {
            let count = count.0;

            if half.has_changed() {
                TIMES_CHANGED.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }

            let double = half.data.0;

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

        #[derive(PartialEq)]
        pub struct CounterMult(usize);

        static TIMES_UPDATED: AtomicUsize = AtomicUsize::new(0);

        fn assert_times_updated(expected: usize) {
            let v = TIMES_UPDATED.load(std::sync::atomic::Ordering::SeqCst);

            assert_eq!(v, expected);
        }

        impl MemoValue for CounterMult {
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
            cached_product: Memo<CounterMult>,
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
