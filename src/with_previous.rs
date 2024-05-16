use bevy::{
    ecs::{
        component::ComponentId,
        system::{ReadOnlySystemParam, SystemParam},
    },
    prelude::*,
};

use crate::{has_changed::HasChanged, has_item_changed::HasItemChanged};

pub struct Cached {}

#[derive(Debug)]
pub struct WithPrevious<'w, 's, T: Resource + Clone> {
    inner: Res<'w, T>,
    previous: &'s T,
}

impl<'w1, 's1, T: Resource + Clone> HasItemChanged for WithPrevious<'w1, 's1, T> {
    fn has_item_changed<'w, 's>(item: &Self::Item<'w, 's>) -> bool {
        item.has_changed()
    }
}

impl<'w, 's, T: Resource + Clone> HasChanged for WithPrevious<'w, 's, T> {
    fn has_changed(&self) -> bool {
        self.is_changed()
    }
}

impl<'w, 's, T: Resource + Clone> AsRef<T> for WithPrevious<'w, 's, T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<'w, 's, T: Resource + Clone> WithPrevious<'w, 's, T> {
    #[must_use]
    pub fn previous_if_changed(&self) -> Option<&'s T> {
        if self.is_changed() {
            Some(self.previous)
        } else {
            None
        }
    }
}

impl<'w, 's, T: Resource + Clone> std::ops::Deref for WithPrevious<'w, 's, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

unsafe impl<'w, 's, T: Resource + Clone> ReadOnlySystemParam for WithPrevious<'w, 's, T> {}

impl<'w, 's, T: Resource + Clone> DetectChanges for WithPrevious<'w, 's, T> {
    fn is_added(&self) -> bool {
        self.inner.is_added()
    }

    fn is_changed(&self) -> bool {
        self.inner.is_changed()
    }

    fn last_changed(&self) -> bevy::ecs::component::Tick {
        self.inner.last_changed()
    }
}

pub struct WithPreviousState<T> {
    component_id: ComponentId,
    current_prev: T,
    next_prev: Option<T>,
}

unsafe impl<'w, 's, T: Resource + Clone> SystemParam for WithPrevious<'w, 's, T> {
    type State = WithPreviousState<T>;

    type Item<'world, 'state> = WithPrevious<'world, 'state, T>;

    fn init_state(
        world: &mut World,
        system_meta: &mut bevy::ecs::system::SystemMeta,
    ) -> Self::State {
        let component_id = Res::<'w, T>::init_state(world, system_meta);
        let t = world.resource::<T>();
        let current_prev = t.clone();

        WithPreviousState {
            component_id,
            current_prev,
            next_prev: None,
        }
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        change_tick: bevy::ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        let res = Res::<'w, T>::get_param(&mut state.component_id, system_meta, world, change_tick);

        if let Some(p) = state.next_prev.take() {
            state.current_prev = p;
        }

        if res.is_changed() {
            state.next_prev = Some(res.as_ref().clone());
        }

        WithPrevious {
            inner: res,
            previous: &state.current_prev,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{prelude::*, with_previous::WithPrevious};

    #[test]
    pub fn test_with_previous() {
        let mut app = App::new();

        #[derive(Debug, Default, Resource, Clone)]
        struct Counter {
            pub count: usize,
        }

        #[derive(Debug, Default, Resource, Clone, PartialEq)]
        struct State {
            pub current: usize,
            pub previous: Option<usize>,
        }

        fn update_state(counter: WithPrevious<Counter>, mut state: ResMut<State>) {
            state.previous = counter.previous_if_changed().map(|x| x.count);
            state.current = counter.as_ref().count;
        }

        app.init_resource::<Counter>();
        app.init_resource::<State>();

        app.add_systems(Update, update_state);

        assert_state(&app, 0, None); //initial value

        app.update();
        assert_state(&app, 0, Some(0)); //after update, previous is 0

        app.update();
        assert_state(&app, 0, None);
        increment_counter(&mut app);
        assert_state(&app, 0, None); //state should not change immediately
        app.update();
        assert_state(&app, 1, Some(0));
        app.update();
        assert_state(&app, 1, None);

        increment_counter(&mut app);
        increment_counter(&mut app);
        app.update();
        assert_state(&app, 3, Some(1));
        app.update();
        assert_state(&app, 3, None);

        fn assert_state(app: &App, expected_count: usize, expected_prev: Option<usize>) {
            let state = app.world.resource::<State>();

            assert_eq!(
                state,
                &State {
                    current: expected_count,
                    previous: expected_prev
                }
            );
        }

        fn increment_counter(app: &mut App) {
            let mut counter = app.world.resource_mut::<Counter>();

            counter.count += 1;
        }
    }
}
