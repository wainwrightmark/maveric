use crate::transition::prelude::*;
use bevy::{ecs::component::ComponentId, prelude::*, utils::HashSet};
use std::time::Duration;

pub trait CanRegisterTransition {
    fn register_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Component,
        L::Value: Tweenable;

    fn register_resource_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Resource,
        L::Value: Tweenable;
}

impl CanRegisterTransition for App {
    fn register_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Component,
        L::Value: Tweenable,
    {
        self.add_systems(PreUpdate, step_transition::<L>);

        #[cfg(feature = "tracing")]
        {
            if !self.is_plugin_added::<crate::tracing::TracingPlugin>() {
                self.add_plugins(crate::tracing::TracingPlugin::default());
            }
        }

        #[cfg(debug_assertions)]
        {
            let component_id = self.world.init_component::<Transition<L>>();

            if let Some(mut rt) = self.world.get_resource_mut::<RegisteredTransitions>() {
                rt.0.insert(component_id);
            } else {
                let mut set = HashSet::new();
                set.insert(component_id);
                self.insert_resource(RegisteredTransitions(set));
            }
        }

        self
    }

    fn register_resource_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Resource,
        L::Value: Tweenable,
    {
        self.init_resource::<ResourceTransition<L>>();
        self.add_systems(
            PreUpdate,
            step_resource_transition::<L>
                .run_if(|r: Res<ResourceTransition<L>>| r.transition.is_some()),
        );

        #[cfg(feature = "tracing")]
        {
            if !self.is_plugin_added::<crate::tracing::TracingPlugin>() {
                self.add_plugins(crate::tracing::TracingPlugin::default());
            }
        }

        self
    }
}

#[derive(Resource, Clone)]
pub struct ResourceTransition<L: Lens + GetValueLens + SetValueLens>
where
    L::Object: Resource,
    L::Value: Tweenable,
{
    pub transition: Option<Transition<L>>,
}

impl<L: Lens + GetValueLens + SetValueLens> Default for ResourceTransition<L>
where
    L::Object: Resource,
    L::Value: Tweenable,
{
    fn default() -> Self {
        Self { transition: None }
    }
}

fn step_resource_transition<L: Lens + GetValueLens + SetValueLens>(
    mut resource: ResMut<L::Object>,
    mut resource_transition: ResMut<ResourceTransition<L>>,
    time: Res<Time>,
) where
    L::Object: Resource,
    L::Value: Tweenable,
{
    if let Some(transition) = resource_transition.transition.as_mut() {
        #[cfg(feature = "tracing")]
        {
            crate::tracing::TRANSITIONS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        let remaining_delta = time.delta();

        if transition.step(resource.as_mut(), remaining_delta) {
            resource_transition.transition = None;
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn step_transition<L: Lens + GetValueLens + SetValueLens>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transition<L>, &mut L::Object)>,
) where
    L::Object: Component,
    L::Value: Tweenable,
{
    #[cfg(feature = "tracing")]
    let mut count: usize = 0;

    for (entity, mut transition, mut object) in &mut query {
        #[cfg(feature = "tracing")]
        {
            count += 1;
        }

        let remaining_delta = time.delta();

        if transition.step(&mut object, remaining_delta) {
            commands.entity(entity).remove::<Transition<L>>();
        }
    }

    #[cfg(feature = "tracing")]
    {
        if count > 0 {
            crate::tracing::TRANSITIONS.fetch_add(count, std::sync::atomic::Ordering::Relaxed);
        }
    }
}

/// A plugin that checks all transition components are registered
/// Should only be added on debug mode
pub(crate) struct CheckTransitionsPlugin;

#[derive(Debug, Resource)]
struct RegisteredTransitions(HashSet<ComponentId>);

impl Plugin for CheckTransitionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_transitions);
    }
}

fn check_transitions(
    world: &World,
    transitions: Option<Res<RegisteredTransitions>>,
    time: Res<Time>,
    mut remaining_time: Local<Duration>,
) {
    if let Some(new_remaining) = remaining_time.checked_sub(time.delta()) {
        *remaining_time = new_remaining
    } else {
        *remaining_time = Duration::from_secs(3);

        for component in world.components().iter().filter(|x| {
            x.name()
                .starts_with("maveric::transition::step::Transition<")
        }) {
            let is_registered = match &transitions {
                Some(r) => r.0.contains(&component.id()),
                None => false,
            };

            if !is_registered {
                warn!("Unregistered Transition: {}", component.name());
            }
        }
    }
}
