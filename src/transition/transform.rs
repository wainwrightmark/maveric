use std::{marker::PhantomData, ops::Add, time::Duration};

use crate::prelude::*;
use crate::transition::prelude::*;
use bevy::prelude::*;



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
