use std::{ops::Add, time::Duration};

use crate::prelude::*;
use bevy::prelude::*;

pub struct TransitionPlugin;

impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, step_transition);
    }
}

fn step_transition(
    time: Res<Time>,
    mut query: Query<(&mut TransformPathComponent, &mut Transform)>,
) {
    let delta = time.delta_seconds();

    for (mut tp, mut t) in query.iter_mut() {
        let Some(step) = tp.current_step() else {continue;};
        let transform = t.as_mut();
        step.advance_transform(transform, delta);
        if step.destination == *t {
            tp.go_to_next_step();
        }
    }
}

/// This required the animation plugin
#[derive(Debug, PartialEq)]
pub struct WithTransformTransition<N: StateTreeNode> {
    pub node: N,
    pub inserted_transform: Transform,
    pub path: TransformPath,
    pub deletion_path: Option<TransformPath>,
}

impl<N: StateTreeNode> StateTreeNode for WithTransformTransition<N> {
    type Context<'c> = N::Context<'c>;

    fn get_components<'c>(
        &self,
        context: &Self::Context<'c>,
        component_commands: &mut impl ComponentCommands,
    ) {
        self.node.get_components(context, component_commands);

        if component_commands.get::<Transform>().is_none() {
            component_commands.insert(self.inserted_transform);
        }

        let new_path_index: Option<usize> =
            if let Some(suspended_path) = component_commands.get::<SuspendedPathComponent>() {
                let i = suspended_path
                    .index
                    .min(self.path.steps.len().saturating_sub(1));

                //info!("Restoring suspended path index {i}");
                component_commands.remove::<SuspendedPathComponent>();
                Some(i)
            } else if let Some(previous_path) = component_commands.get::<TransformPathComponent>() {
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
            component_commands.insert(TransformPathComponent {
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

        let transform = component_commands
            .get::<Transform>()
            .unwrap_or(&self.inserted_transform);
        let duration = deletion_path.remaining_duration(transform);

        let duration = match base {
            DeletionPolicy::DeleteImmediately => duration,
            DeletionPolicy::Linger(d) => duration.max(d),
        };
        let current_path = component_commands.get::<TransformPathComponent>();

        if let Some(current_path) = current_path {
            component_commands.insert(SuspendedPathComponent {
                index: current_path.index,
            })
        }

        component_commands.insert(TransformPathComponent {
            path: deletion_path.clone(),
            index: 0,
        });

        DeletionPolicy::Linger(duration)
    }
}

#[derive(Debug, Component)]
pub(crate) struct TransformPathComponent {
    pub path: TransformPath,
    pub index: usize,
}

impl TransformPathComponent {
    pub fn current_step(&self) -> Option<&TransformStep> {
        self.path.steps.get(self.index)
    }

    pub fn go_to_next_step(&mut self) {
        self.index += 1;
    }
}

#[derive(Debug, Component)]
pub(crate) struct SuspendedPathComponent {
    pub index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransformPath {
    pub steps: Vec<TransformStep>,
}

impl From<TransformStep> for TransformPath {
    fn from(value: TransformStep) -> Self {
        Self { steps: vec![value] }
    }
}

impl TransformPath {
    pub fn remaining_duration(&self, start: &Transform) -> Duration {
        let mut total: Duration = Duration::default();
        let mut current: &Transform = start;

        for step in self.steps.iter() {
            total += step.duration(current);
            current = &step.destination;
        }

        total
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransformStep {
    pub destination: Transform,
    pub velocity: Velocity,
}

impl TransformStep {
    pub fn duration(&self, start: &Transform) -> Duration {
        let translate_seconds =
            (self.destination.translation - start.translation).length() / self.velocity.max_linear;
        let angular_seconds =
            (self.destination.rotation.angle_between(start.rotation)) / self.velocity.max_angular;
        let scale_seconds =
            (self.destination.scale.distance(start.scale)) / self.velocity.max_scale;

        let seconds = [translate_seconds, angular_seconds, scale_seconds]
            .into_iter()
            .filter(|x| x.is_finite())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or_default();

        Duration::from_secs_f32(seconds)
    }

    pub fn advance_transform(&self, transform: &mut Transform, delta_seconds: f32) {
        // info!(
        //     "Advance transform {transform:?} {vel:?}",
        //     vel = self.velocity
        // );

        // translation
        if self.destination.translation != transform.translation {
            let t = self.destination.translation - transform.translation;
            let change = t.clamp_length_max(delta_seconds * self.velocity.max_linear);
            transform.translation += change;
        }

        // rotation
        if self.destination.rotation != transform.rotation {
            let change = quat_clamp_length_max(
                self.destination.rotation - transform.rotation,
                self.velocity.max_angular * delta_seconds,
            );

            //info!("Updating rotation {} + {}", transform.rotation, change);
            transform.rotation = transform.rotation.add(change);
        }

        // scale
        if self.destination.scale != transform.scale {
            let change = (self.destination.scale - transform.scale)
                .clamp_length_max(delta_seconds * self.velocity.max_scale);

            //info!("Updating scale {} + {}", transform.scale, change);
            transform.scale += change;
        }
    }
}

pub fn quat_clamp_length_max(q: Quat, max: f32) -> Quat {
    let length_sq = q.length_squared();
    if length_sq > max * max {
        (q / f32::sqrt(length_sq)) * max
    } else {
        q
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Velocity {
    max_linear: f32,
    max_angular: f32,
    max_scale: f32,
}

impl Velocity {
    pub fn from_linear(max_linear: f32) -> Self {
        Self {
            max_linear,
            ..Default::default()
        }
    }

    pub fn from_angular(max_angular: f32) -> Self {
        Self {
            max_angular,
            ..Default::default()
        }
    }

    pub fn from_scale(max_scale: f32) -> Self {
        Self {
            max_scale,
            ..Default::default()
        }
    }
}
