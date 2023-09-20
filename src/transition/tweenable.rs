use std::{
    fmt::Debug,
    time::{Duration, TryFromFloatSecsError},
};

use bevy::prelude::*;

use super::speed::{AngularSpeed, LinearSpeed, ScalarSpeed, Speed};

pub trait Tweenable: Debug + Clone + Send + Sync + PartialEq + 'static {
    type Speed: Speed;

    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError>;

    fn transition_towards(&self, rhs: &Self, speed: &Self::Speed, delta_seconds: &f32) -> Self;
}

impl Tweenable for f32 {
    type Speed = ScalarSpeed;

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
        let clamped = diff.clamp_length_max(speed.units_per_second * delta_seconds);
        //info!("From: {self:?} to {rhs:?} {diff} {clamped:?} {speed} * {delta_seconds}");
        *self + clamped
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
}

#[cfg(any(feature = "bevy_ui", test))]
impl Tweenable for Val {
    type Speed = <f32 as Tweenable>::Speed;

    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError> {
        match (self, rhs) {
            (Val::Px(l), Val::Px(r))
            | (Val::Percent(l), Val::Percent(r))
            | (Val::Vw(l), Val::Vw(r))
            | (Val::Vh(l), Val::Vh(r))
            | (Val::VMin(l), Val::VMin(r))
            | (Val::VMax(l), Val::VMax(r)) => <f32 as Tweenable>::duration_to(&l, r, speed),
            _ => Ok(Duration::ZERO),
        }
    }

    fn transition_towards(&self, rhs: &Self, speed: &Self::Speed, delta_seconds: &f32) -> Self {
        match (self, rhs) {
            (Val::Px(l), Val::Px(r)) => Val::Px(<f32 as Tweenable>::transition_towards(
                &l,
                r,
                speed,
                delta_seconds,
            )),
            (Val::Percent(l), Val::Percent(r)) => Val::Percent(
                <f32 as Tweenable>::transition_towards(&l, r, speed, delta_seconds),
            ),
            (Val::Vw(l), Val::Vw(r)) => Val::Vw(<f32 as Tweenable>::transition_towards(
                &l,
                r,
                speed,
                delta_seconds,
            )),
            (Val::Vh(l), Val::Vh(r)) => Val::Vh(<f32 as Tweenable>::transition_towards(
                &l,
                r,
                speed,
                delta_seconds,
            )),
            (Val::VMin(l), Val::VMin(r)) => Val::VMin(<f32 as Tweenable>::transition_towards(
                &l,
                r,
                speed,
                delta_seconds,
            )),
            (Val::VMax(l), Val::VMax(r)) => Val::VMax(<f32 as Tweenable>::transition_towards(
                &l,
                r,
                speed,
                delta_seconds,
            )),
            _ => *rhs,
        }
    }
}

#[cfg(any(feature = "bevy_render", test))]
impl Tweenable for Color {
    type Speed = ScalarSpeed;

    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError> {
        let differences: [f32; 4] = match (self, rhs) {
            (
                Color::Rgba {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Color::Rgba {
                    red: red2,
                    green: green2,
                    blue: blue2,
                    alpha: alpha2,
                },
            ) => [
                (red - red2).abs(),
                (green - green2).abs(),
                (blue - blue2).abs(),
                (alpha - alpha2).abs(),
            ],
            (
                Color::RgbaLinear {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Color::RgbaLinear {
                    red: red2,
                    green: green2,
                    blue: blue2,
                    alpha: alpha2,
                },
            ) => [
                (red - red2).abs(),
                (green - green2).abs(),
                (blue - blue2).abs(),
                (alpha - alpha2).abs(),
            ],

            (
                Color::Hsla {
                    hue,
                    saturation,
                    lightness,
                    alpha,
                },
                Color::Hsla {
                    hue: hue2,
                    saturation: saturation2,
                    lightness: lightness2,
                    alpha: alpha2,
                },
            ) => [
                (hue - hue2).abs(),
                (saturation - saturation2).abs(),
                (lightness - lightness2).abs(),
                (alpha - alpha2).abs(),
            ],
            (
                Color::Lcha {
                    lightness,
                    chroma,
                    hue,
                    alpha,
                },
                Color::Lcha {
                    lightness: lightness2,
                    chroma: chroma2,
                    hue: hue2,
                    alpha: alpha2,
                },
            ) => [
                (hue - hue2).abs(),
                (chroma - chroma2).abs(),
                (lightness - lightness2).abs(),
                (alpha - alpha2).abs(),
            ],
            _ => {
                return Duration::try_from_secs_f32(f32::NAN);
            }
        };

        let difference = differences.into_iter().max_by(f32::total_cmp).unwrap();

        let seconds = difference / speed.amount_per_second;
        //info!("Color transitions {self:?} {rhs:?} {seconds}");
        Duration::try_from_secs_f32(seconds)
    }

    fn transition_towards(&self, rhs: &Self, speed: &Self::Speed, delta_seconds: &f32) -> Self {
        match (self, rhs) {
            (
                Color::Rgba {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Color::Rgba {
                    red: red2,
                    green: green2,
                    blue: blue2,
                    alpha: alpha2,
                },
            ) => Color::Rgba {
                red: red.transition_towards(red2, speed, delta_seconds),
                green: green.transition_towards(green2, speed, delta_seconds),
                blue: blue.transition_towards(blue2, speed, delta_seconds),
                alpha: alpha.transition_towards(alpha2, speed, delta_seconds),
            },
            (
                Color::RgbaLinear {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Color::RgbaLinear {
                    red: red2,
                    green: green2,
                    blue: blue2,
                    alpha: alpha2,
                },
            ) => Color::RgbaLinear {
                red: red.transition_towards(red2, speed, delta_seconds),
                green: green.transition_towards(green2, speed, delta_seconds),
                blue: blue.transition_towards(blue2, speed, delta_seconds),
                alpha: alpha.transition_towards(alpha2, speed, delta_seconds),
            },

            (
                Color::Hsla {
                    hue,
                    saturation,
                    lightness,
                    alpha,
                },
                Color::Hsla {
                    hue: hue2,
                    saturation: saturation2,
                    lightness: lightness2,
                    alpha: alpha2,
                },
            ) => Color::Hsla {
                lightness: lightness.transition_towards(lightness2, speed, delta_seconds),
                saturation: saturation.transition_towards(saturation2, speed, delta_seconds),
                hue: hue.transition_towards(hue2, speed, delta_seconds),
                alpha: alpha.transition_towards(alpha2, speed, delta_seconds),
            },
            (
                Color::Lcha {
                    lightness,
                    chroma,
                    hue,
                    alpha,
                },
                Color::Lcha {
                    lightness: lightness2,
                    chroma: chroma2,
                    hue: hue2,
                    alpha: alpha2,
                },
            ) => Color::Lcha {
                lightness: lightness.transition_towards(lightness2, speed, delta_seconds),
                chroma: chroma.transition_towards(chroma2, speed, delta_seconds),
                hue: hue.transition_towards(hue2, speed, delta_seconds),
                alpha: alpha.transition_towards(alpha2, speed, delta_seconds),
            },
            _ => rhs.clone(),
        }
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

impl_tweenable!((T0, t0, r0, s0));
impl_tweenable!((T0, t0, r0, s0), (T1, t1, r1, s1));
impl_tweenable!((T0, t0, r0, s0), (T1, t1, r1, s1), (T2, t2, r2, s2));
impl_tweenable!(
    (T0, t0, r0, s0),
    (T1, t1, r1, s1),
    (T2, t2, r2, s2),
    (T3, t3, r3, s3)
);
