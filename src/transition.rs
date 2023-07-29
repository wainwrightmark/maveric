use std::{marker::PhantomData, ops::Add, time::Duration};

use crate::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct TransitionPlugin<V: ComponentVelocity> {
    phantom: PhantomData<V>,
}

impl<V: ComponentVelocity> Plugin for TransitionPlugin<V> {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, step_transition::<V>);
    }
}

fn step_transition<V: ComponentVelocity>(
    time: Res<Time>,
    mut query: Query<(&mut TransitionPathComponent<V>, &mut V::C)>,
) {
    let delta_seconds = time.delta_seconds();

    for (mut tp, mut t) in query.iter_mut() {
        let Some(step) = tp.current_step() else {continue;};
        let component = t.as_mut();
        step.velocity
            .advance(&step.destination, delta_seconds, component);
        if step.destination == *t {
            tp.go_to_next_step();
        }
    }
}

pub trait ComponentVelocity: PartialEq + Clone + Send + Sync + 'static {
    type C: Component + PartialEq + Clone;

    /// Advance the component towards the destination
    fn advance(&self, destination: &Self::C, delta_seconds: f32, component: &mut Self::C);

    /// How long it will take to get from the start to the destination
    fn duration(&self, destination: &Self::C, start: &Self::C) -> Duration;
}

/// This required the animation plugin
#[derive(PartialEq)]
pub struct WithTransformTransition<N: StateTreeNode, V: ComponentVelocity> {
    pub node: N,
    pub initial: V::C,
    pub path: TransitionPath<V>,
    pub deletion_path: Option<TransitionPath<V>>,
}

impl<N: StateTreeNode, V: ComponentVelocity> StateTreeNode for WithTransformTransition<N, V> {
    type Context<'c> = N::Context<'c>;

    fn get_components<'c>(
        &self,
        context: &Self::Context<'c>,
        component_commands: &mut impl ComponentCommands,
    ) {
        self.node.get_components(context, component_commands);

        if let Some(previous) = component_commands.get::<V::C>() {
            component_commands.insert(previous.clone()); //prevent this being overwritten by component_commands
        } else {
            component_commands.insert(self.initial.clone());
        }

        let new_path_index: Option<usize> = if let Some(suspended_path) =
            component_commands.get::<SuspendedPathComponent<V>>()
        {
            let i = suspended_path
                .index
                .min(self.path.steps.len().saturating_sub(1));

            //info!("Restoring suspended path index {i}");
            component_commands.remove::<SuspendedPathComponent<V>>();
            Some(i)
        } else if let Some(previous_path) = component_commands.get::<TransitionPathComponent<V>>() {
            if previous_path.path != self.path {
                //info!("New path found");
                Some(0)
            } else {
                //info!("Same path found");
                None
            }
        } else {
            //info!("No preexisting path found");
            Some(0)
        };

        if let Some(index) = new_path_index {
            component_commands.insert(TransitionPathComponent {
                path: self.path.clone(),
                index,
            });
        }
    }

    fn get_children<'c>(
        &self,
        context: &Self::Context<'c>,
        child_commands: &mut impl ChildCommands,
    ) {
        self.node.get_children(context, child_commands)
    }

    fn on_deleted(&self, component_commands: &mut impl ComponentCommands) -> DeletionPolicy {
        let base = self.node.on_deleted(component_commands);

        let Some(deletion_path) = &self.deletion_path else{return  base;};

        let transform = component_commands.get::<V::C>().unwrap_or(&self.initial);
        let duration = deletion_path.remaining_duration(transform);

        let duration = match base {
            DeletionPolicy::DeleteImmediately => duration,
            DeletionPolicy::Linger(d) => duration.max(d),
        };
        let current_path = component_commands.get::<TransitionPathComponent<V>>();

        if let Some(current_path) = current_path {
            component_commands.insert(SuspendedPathComponent::<V> {
                index: current_path.index,
                phantom: PhantomData,
            })
        }

        component_commands.insert(TransitionPathComponent {
            path: deletion_path.clone(),
            index: 0,
        });

        DeletionPolicy::Linger(duration)
    }
}

#[derive(Component)]
pub(crate) struct TransitionPathComponent<V: ComponentVelocity> {
    pub path: TransitionPath<V>,
    pub index: usize,
}

impl<V: ComponentVelocity> TransitionPathComponent<V> {
    pub fn current_step(&self) -> Option<&TransitionStep<V>> {
        self.path.steps.get(self.index)
    }

    pub fn go_to_next_step(&mut self) {
        self.index += 1;
    }
}

#[derive(Debug, Component)]
pub(crate) struct SuspendedPathComponent<V: ComponentVelocity> {
    pub index: usize,
    phantom: PhantomData<V>,
}

#[derive(Clone, PartialEq)]
pub struct TransitionPath<V: ComponentVelocity> {
    pub steps: Vec<TransitionStep<V>>,
}

impl<V: ComponentVelocity> From<TransitionStep<V>> for TransitionPath<V> {
    fn from(value: TransitionStep<V>) -> Self {
        Self { steps: vec![value] }
    }
}

impl<V: ComponentVelocity> TransitionPath<V> {
    pub fn remaining_duration(&self, start: &V::C) -> Duration {
        let mut total: Duration = Duration::default();
        let mut current: &V::C = start;

        for step in self.steps.iter() {
            total += step.velocity.duration(&step.destination, current);
            current = &step.destination;
        }

        total
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransitionStep<V: ComponentVelocity> {
    pub destination: V::C,
    pub velocity: V,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TransformVelocity {
    max_linear: f32,
    max_angular: f32,
    max_scale: f32,
}

impl ComponentVelocity for TransformVelocity {
    type C = Transform;

    fn advance(&self, destination: &Self::C, delta_seconds: f32, component: &mut Self::C) {
        if destination.translation != component.translation {
            let t = destination.translation - component.translation;
            let change = t.clamp_length_max(delta_seconds * self.max_linear);
            component.translation += change;
        }

        // rotation
        if destination.rotation != component.rotation {
            let change = quat_clamp_length_max(
                destination.rotation - component.rotation,
                self.max_angular * delta_seconds,
            );

            //info!("Updating rotation {} + {}", transform.rotation, change);
            component.rotation = component.rotation.add(change);
        }

        // scale
        if destination.scale != component.scale {
            let change = (destination.scale - component.scale)
                .clamp_length_max(delta_seconds * self.max_scale);

            //info!("Updating scale {} + {}", transform.scale, change);
            component.scale += change;
        }
    }

    fn duration(&self, destination: &Self::C, start: &Self::C) -> Duration {
        let translate_seconds =
            (destination.translation - start.translation).length() / self.max_linear;
        let angular_seconds =
            (destination.rotation.angle_between(start.rotation)) / self.max_angular;
        let scale_seconds = (destination.scale.distance(start.scale)) / self.max_scale;

        let seconds = [translate_seconds, angular_seconds, scale_seconds]
            .into_iter()
            .filter(|x| x.is_finite())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or_default();

        Duration::from_secs_f32(seconds)
    }
}

impl TransformVelocity {
    pub fn from_linear(max_linear: f32) -> Self {
        Self {
            max_linear,
            ..Default::default()
        }
    }

    pub fn with_linear(mut self, max_linear: f32) -> Self {
        self.max_linear = max_linear;
        self
    }

    pub fn from_angular(max_angular: f32) -> Self {
        Self {
            max_angular,
            ..Default::default()
        }
    }

    pub fn with_angular(mut self, max_angular: f32) -> Self {
        self.max_angular = max_angular;
        self
    }

    pub fn from_scale(max_scale: f32) -> Self {
        Self {
            max_scale,
            ..Default::default()
        }
    }

    pub fn with_scale(mut self, max_scale: f32) -> Self {
        self.max_scale = max_scale;
        self
    }
}

fn quat_clamp_length_max(q: Quat, max: f32) -> Quat {
    let length_sq = q.length_squared();
    if length_sq > max * max {
        (q / f32::sqrt(length_sq)) * max
    } else {
        q
    }
}
