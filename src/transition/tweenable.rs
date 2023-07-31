use std::{
    fmt::Debug,
    time::{Duration, TryFromFloatSecsError},
};

use bevy::prelude::*;

use super::speed::{AngularSpeed, LinearSpeed, ScalarSpeed};

pub trait Tweenable: Debug + Clone + Send + Sync + 'static {
    type Speed: core::fmt::Debug + Clone + PartialEq + Send + Sync + 'static;
    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError>;
    fn transition_towards(&self, rhs: &Self, speed: &Self::Speed, delta_seconds: &f32) -> Self;

    fn approx_eq(&self, rhs: &Self) -> bool;
}

impl Tweenable for f32 {
    type Speed = ScalarSpeed;

    fn approx_eq(&self, rhs: &Self) -> bool {
        (self - rhs).abs() < f32::EPSILON
    }

    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError> {
        if self == rhs {
            return Ok(Duration::ZERO);
        }

        Duration::try_from_secs_f32((self - rhs).abs() / speed.amount_per_second)
    }

    fn transition_towards(&self, rhs: &Self, speed: &Self::Speed, elapsed_seconds: &f32) -> Self {
        let diff = rhs - *self;

        self + (diff.abs().min(speed.amount_per_second * elapsed_seconds) * diff.signum())
    }
}

impl Tweenable for Vec2 {
    type Speed = LinearSpeed;

    fn transition_towards(&self, rhs: &Self, speed: &Self::Speed, delta_seconds: &f32) -> Self {
        let diff = *rhs - *self;
        *self + diff.clamp_length_max(speed.units_per_second * delta_seconds)
    }

    fn approx_eq(&self, rhs: &Self) -> bool {
        self.abs_diff_eq(*rhs, f32::EPSILON)
    }

    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError> {
        if self == rhs {
            return Ok(Duration::ZERO);
        }

        Duration::try_from_secs_f32(self.distance(*rhs) / speed.units_per_second)
    }
}

impl Tweenable for Vec3 {
    type Speed = LinearSpeed;

    fn transition_towards(&self, rhs: &Self, speed: &Self::Speed, delta_seconds: &f32) -> Self {
        let diff = *rhs - *self;
        *self + diff.clamp_length_max(speed.units_per_second * delta_seconds)
    }

    fn approx_eq(&self, rhs: &Self) -> bool {
        self.abs_diff_eq(*rhs, f32::EPSILON)
    }

    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError> {
        if self == rhs {
            return Ok(Duration::ZERO);
        }

        Duration::try_from_secs_f32(self.distance(*rhs) / speed.units_per_second)
    }
}

impl Tweenable for Quat {
    type Speed = AngularSpeed;

    fn transition_towards(&self, rhs: &Self, speed: &Self::Speed, delta_seconds: &f32) -> Self {
        let diff = *rhs - *self;
        *self + quat_clamp_length_max(diff, speed.radians_per_second * delta_seconds)
    }

    fn approx_eq(&self, rhs: &Self) -> bool {
        self.abs_diff_eq(*rhs, f32::EPSILON)
    }

    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError> {
        if self == rhs {
            return Ok(Duration::ZERO);
        }

        Duration::try_from_secs_f32(self.angle_between(*rhs) / speed.radians_per_second)
    }
}

impl Tweenable for Transform {
    type Speed = (LinearSpeed, AngularSpeed, LinearSpeed);

    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError> {
        (self.translation, self.rotation, self.scale)
            .duration_to(&(rhs.translation, rhs.rotation, rhs.scale), speed)
    }

    fn transition_towards(&self, rhs: &Self, speed: &Self::Speed, delta_seconds: &f32) -> Self {
        let (translation, rotation, scale) = (self.translation, self.rotation, self.scale)
            .transition_towards(
                &(rhs.translation, rhs.rotation, rhs.scale),
                speed,
                delta_seconds,
            );
        Self {
            translation,
            rotation,
            scale,
        }
    }

    fn approx_eq(&self, rhs: &Self) -> bool {
        (self.translation, self.rotation, self.scale).approx_eq(&(
            rhs.translation,
            rhs.rotation,
            rhs.scale,
        ))
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

macro_rules! impl_tweenable {
    ($(($T:ident, $t:ident, $r:ident, $s:ident)),*) => {
        impl<$($T : Tweenable,)*> Tweenable for ($($T,)*) {

            type Speed = ($($T::Speed,)*);

            fn approx_eq(&self, rhs: &Self) -> bool {
                let ($($t,)*) = self;
                let ($($r,)*) = rhs;

                $($t.approx_eq($r) &&)* true
            }

            fn duration_to(&self, rhs: &Self, speed: &Self::Speed) -> Result<Duration, TryFromFloatSecsError> {
                let ($($t,)*) = self;
                let ($($r,)*) = rhs;
                let ($($s,)*) = speed;

                let result = Duration::default() $(.max($t.duration_to($r, $s)?))*;

                Ok(result)
            }

            fn transition_towards(&self, rhs: &Self, speed: &Self::Speed, _delta_seconds: &f32) -> Self {
                let ($($t,)*) = self;
                let ($($r,)*) = rhs;
                let ($($s,)*) = speed;

                (
                    $($t.transition_towards($r, $s, _delta_seconds),)*
                )
            }
        }
    };
}

impl_tweenable!();
impl_tweenable!((T0, t0, r0, s0));
impl_tweenable!((T0, t0, r0, s0), (T1, t1, r1, s1));
impl_tweenable!((T0, t0, r0, s0), (T1, t1, r1, s1), (T2, t2, r2, s2));
impl_tweenable!(
    (T0, t0, r0, s0),
    (T1, t1, r1, s1),
    (T2, t2, r2, s2),
    (T3, t3, r3, s3)
);
