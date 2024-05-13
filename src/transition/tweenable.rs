use std::{
    fmt::Debug,
    time::{Duration, TryFromFloatSecsError},
};

use bevy::prelude::*;

use super::speed::{AngularSpeed, LinearSpeed, ScalarSpeed, Speed};

pub trait Tweenable: Debug + Clone + Send + Sync + PartialEq + 'static {
    type Speed: Speed;

    /// # Errors
    /// If speed is zero
    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError>;

    /// Transition this value towards `destination` at the given speed for `delta_seconds`
    /// Returns Some(s) if the destination was reached where `s` is the remaining seconds
    #[must_use]
    fn transition_towards(
        &mut self,
        destination: &Self,
        speed: &Self::Speed,
        delta_seconds: &f32,
    ) -> Option<f32>;

    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    fn lerp_value(&self, rhs: &Self, s: f32) -> Self;
}

impl Tweenable for f32 {
    type Speed = ScalarSpeed;

    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError> {
        if (self - rhs).abs() < Self::EPSILON {
            return Ok(Duration::ZERO);
        }

        Duration::try_from_secs_f32((self - rhs).abs() / speed.amount_per_second)
    }

    fn transition_towards(
        &mut self,
        destination: &Self,
        speed: &Self::Speed,
        delta_seconds: &f32,
    ) -> Option<f32> {
        let distance = *destination - *self;

        let change = speed.amount_per_second * delta_seconds;
        if change < distance.abs() {
            *self += change * distance.signum();
            None
        } else {
            *self = *destination;
            Some(delta_seconds - (distance.abs() / speed.amount_per_second))
        }
    }

    fn lerp_value(&self, rhs: &Self, s: f32) -> Self {
        self + ((rhs - self) * s)
    }
}

impl Tweenable for Vec2 {
    type Speed = LinearSpeed;

    fn transition_towards(
        &mut self,
        destination: &Self,
        speed: &Self::Speed,
        delta_seconds: &f32,
    ) -> Option<f32> {
        let distance = *destination - *self;

        let change = speed.units_per_second * delta_seconds;
        if change < distance.length() {
            *self += change * distance.normalize_or_zero();
            None
        } else {
            *self = *destination;
            Some(delta_seconds - (distance.length() / speed.units_per_second))
        }
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

    fn lerp_value(&self, rhs: &Self, s: f32) -> Self {
        self.lerp(*rhs, s)
    }
}

impl Tweenable for Vec3 {
    type Speed = LinearSpeed;

    fn transition_towards(
        &mut self,
        destination: &Self,
        speed: &Self::Speed,
        delta_seconds: &f32,
    ) -> Option<f32> {
        let distance = *destination - *self;

        let change = speed.units_per_second * delta_seconds;
        if change < distance.length() {
            *self += change * distance.normalize_or_zero();
            None
        } else {
            *self = *destination;
            Some(delta_seconds - (distance.length() / speed.units_per_second))
        }
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

    fn lerp_value(&self, rhs: &Self, s: f32) -> Self {
        self.lerp(*rhs, s)
    }
}

impl Tweenable for Quat {
    type Speed = AngularSpeed;

    fn transition_towards(
        &mut self,
        destination: &Self,
        speed: &Self::Speed,
        delta_seconds: &f32,
    ) -> Option<f32> {
        let radians = self.angle_between(*destination);

        let change = speed.radians_per_second * delta_seconds;
        if change < radians {
            *self = self.lerp(*destination, change / radians);
            None
        } else {
            *self = *destination;
            Some(delta_seconds - (radians / speed.radians_per_second))
        }
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

    fn lerp_value(&self, rhs: &Self, s: f32) -> Self {
        self.lerp(*rhs, s)
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

    fn transition_towards(
        &mut self,
        destination: &Self,
        speed: &Self::Speed,
        delta_seconds: &f32,
    ) -> Option<f32> {
        let mut tuple = (self.translation, self.rotation, self.scale);

        let result = tuple.transition_towards(
            &(
                destination.translation,
                destination.rotation,
                destination.scale,
            ),
            speed,
            delta_seconds,
        );
        self.translation = tuple.0;
        self.rotation = tuple.1;
        self.scale = tuple.2;

        result
    }

    fn lerp_value(&self, rhs: &Self, s: f32) -> Self {
        Self {
            translation: self.translation.lerp(rhs.translation, s),
            rotation: self.rotation.lerp(rhs.rotation, s),
            scale: self.scale.lerp(rhs.scale, s),
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
            (Self::Px(l), Self::Px(r))
            | (Self::Percent(l), Self::Percent(r))
            | (Self::Vw(l), Self::Vw(r))
            | (Self::Vh(l), Self::Vh(r))
            | (Self::VMin(l), Self::VMin(r))
            | (Self::VMax(l), Self::VMax(r)) => <f32 as Tweenable>::duration_to(l, r, speed),
            _ => Ok(Duration::ZERO),
        }
    }

    fn transition_towards(
        &mut self,
        rhs: &Self,
        speed: &Self::Speed,
        delta_seconds: &f32,
    ) -> Option<f32> {
        match (self, rhs) {
            (Self::Px(l), Self::Px(r)) => {
                <f32 as Tweenable>::transition_towards(l, r, speed, delta_seconds)
            }

            (Self::Percent(l), Self::Percent(r)) => {
                <f32 as Tweenable>::transition_towards(l, r, speed, delta_seconds)
            }

            (Self::Vw(l), Self::Vw(r)) => {
                <f32 as Tweenable>::transition_towards(l, r, speed, delta_seconds)
            }

            (Self::Vh(l), Self::Vh(r)) => {
                <f32 as Tweenable>::transition_towards(l, r, speed, delta_seconds)
            }

            (Self::VMin(l), Self::VMin(r)) => {
                <f32 as Tweenable>::transition_towards(l, r, speed, delta_seconds)
            }

            (Self::VMax(l), Self::VMax(r)) => {
                <f32 as Tweenable>::transition_towards(l, r, speed, delta_seconds)
            }

            (s, rhs) => {
                *s = *rhs;
                Some(*delta_seconds)
            }
        }
    }

    fn lerp_value(&self, rhs: &Self, s: f32) -> Self {
        match (self, rhs) {
            (Self::Auto, Self::Auto) => Self::Auto,
            (Self::Px(l), Self::Px(r)) => Self::Px(l.lerp_value(r, s)),
            (Self::Percent(l), Self::Percent(r)) => Self::Percent(l.lerp_value(r, s)),
            (Self::Vw(l), Self::Vw(r)) => Self::Vw(l.lerp_value(r, s)),
            (Self::Vh(l), Self::Vh(r)) => Self::Vh(l.lerp_value(r, s)),
            (Self::VMin(l), Self::VMin(r)) => Self::VMin(l.lerp_value(r, s)),
            (Self::VMax(l), Self::VMax(r)) => Self::VMax(l.lerp_value(r, s)),
            (lhs, rhs) => {
                if s < 0.5 {
                    *lhs
                } else {
                    *rhs
                }
            }
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
                Self::Rgba {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Self::Rgba {
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
                Self::RgbaLinear {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Self::RgbaLinear {
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
                Self::Hsla {
                    hue,
                    saturation,
                    lightness,
                    alpha,
                },
                Self::Hsla {
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
                Self::Lcha {
                    lightness,
                    chroma,
                    hue,
                    alpha,
                },
                Self::Lcha {
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

    fn transition_towards(
        &mut self,
        destination: &Self,
        speed: &Self::Speed,
        delta_seconds: &f32,
    ) -> Option<f32> {
        match (self, destination) {
            (
                Self::Rgba {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Self::Rgba {
                    red: red2,
                    green: green2,
                    blue: blue2,
                    alpha: alpha2,
                },
            ) => transition_4_tuple(
                (red, green, blue, alpha),
                (red2, green2, blue2, alpha2),
                speed,
                delta_seconds,
            ),
            (
                Self::RgbaLinear {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Self::RgbaLinear {
                    red: red2,
                    green: green2,
                    blue: blue2,
                    alpha: alpha2,
                },
            ) => transition_4_tuple(
                (red, green, blue, alpha),
                (red2, green2, blue2, alpha2),
                speed,
                delta_seconds,
            ),

            (
                Self::Hsla {
                    hue,
                    saturation,
                    lightness,
                    alpha,
                },
                Self::Hsla {
                    hue: hue2,
                    saturation: saturation2,
                    lightness: lightness2,
                    alpha: alpha2,
                },
            ) => transition_4_tuple(
                (hue, saturation, lightness, alpha),
                (hue2, saturation2, lightness2, alpha2),
                speed,
                delta_seconds,
            ),
            (
                Self::Lcha {
                    lightness,
                    chroma,
                    hue,
                    alpha,
                },
                Self::Lcha {
                    lightness: lightness2,
                    chroma: chroma2,
                    hue: hue2,
                    alpha: alpha2,
                },
            ) => transition_4_tuple(
                (lightness, chroma, hue, alpha),
                (lightness2, chroma2, hue2, alpha2),
                speed,
                delta_seconds,
            ),
            (s, destination) => {
                //TODO convert self to the other color and then transition as normal
                *s = *destination;
                Some(*delta_seconds)
            }
        }
    }

    fn lerp_value(&self, rhs: &Self, s: f32) -> Self {
        match (self, rhs) {
            (
                Self::Rgba {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Self::Rgba {
                    red: red2,
                    green: green2,
                    blue: blue2,
                    alpha: alpha2,
                },
            ) => Self::Rgba {
                red: red.lerp_value(red2, s),
                green: green.lerp_value(green2, s),
                blue: blue.lerp_value(blue2, s),
                alpha: alpha.lerp_value(alpha2, s),
            },
            (
                Self::RgbaLinear {
                    red,
                    green,
                    blue,
                    alpha,
                },
                Self::RgbaLinear {
                    red: red2,
                    green: green2,
                    blue: blue2,
                    alpha: alpha2,
                },
            ) => Self::RgbaLinear {
                red: red.lerp_value(red2, s),
                green: green.lerp_value(green2, s),
                blue: blue.lerp_value(blue2, s),
                alpha: alpha.lerp_value(alpha2, s),
            },

            (
                Self::Hsla {
                    hue,
                    saturation,
                    lightness,
                    alpha,
                },
                Self::Hsla {
                    hue: hue2,
                    saturation: saturation2,
                    lightness: lightness2,
                    alpha: alpha2,
                },
            ) => Self::Hsla {
                lightness: lightness.lerp_value(lightness2, s),
                saturation: saturation.lerp_value(saturation2, s),
                hue: hue.lerp_value(hue2, s),
                alpha: alpha.lerp_value(alpha2, s),
            },
            (
                Self::Lcha {
                    lightness,
                    chroma,
                    hue,
                    alpha,
                },
                Self::Lcha {
                    lightness: lightness2,
                    chroma: chroma2,
                    hue: hue2,
                    alpha: alpha2,
                },
            ) => Self::Lcha {
                lightness: lightness.lerp_value(lightness2, s),
                chroma: chroma.lerp_value(chroma2, s),
                hue: hue.lerp_value(hue2, s),
                alpha: alpha.lerp_value(alpha2, s),
            },
            (lhs, rhs) => {
                //TODO convert self to the other color and then lerp as normal
                if s < 0.5 {
                    *lhs
                } else {
                    *rhs
                }
            }
        }
    }
}

fn transition_4_tuple(
    lhs: (&mut f32, &mut f32, &mut f32, &mut f32),
    rhs: (&f32, &f32, &f32, &f32),
    speed: &<f32 as Tweenable>::Speed,
    delta_seconds: &f32,
) -> Option<f32> {
    let mut remaining: Option<f32> = Some(*delta_seconds);

    for pair in [
        (lhs.0, rhs.0),
        (lhs.1, rhs.1),
        (lhs.2, rhs.2),
        (lhs.3, rhs.3),
    ] {
        if let Some(r) = pair.0.transition_towards(pair.1, speed, delta_seconds) {
            if remaining.is_some_and(|rem| rem > r) {
                remaining = Some(r);
            }
        } else {
            remaining = None;
        }
    }
    remaining
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

            fn transition_towards(&mut self, destination: &Self, speed: &Self::Speed, delta_seconds: &f32) -> Option<f32> {
                let ($($t,)*) = self;
                let ($($r,)*) = destination;
                let ($($s,)*) = speed;

                let mut remaining: Option<f32> = Some(*delta_seconds);
                $({
                    if let Some(r) = $t.transition_towards($r, $s, delta_seconds){
                        if remaining.is_some_and(|rem| rem > r){
                            remaining = Some(r);
                        }
                    }else{
                        remaining = None;
                    }

                })*
                remaining
            }

            fn lerp_value(&self, rhs: &Self, s: f32) -> Self {
                let ($($t,)*) = self;
                let ($($r,)*) = rhs;
                ($($t.lerp_value($r, s),)*)
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

#[cfg(test)]
mod tests {
    use bevy::{
        math::{Quat, Vec3},
        render::color::Color,
        transform::components::Transform,
    };

    use super::Tweenable;

    #[test]
    pub fn test_transition_tuple() {
        let mut value = (1.0, 2.0);

        let destination = (5.0, 5.0);

        let r1 = Tweenable::transition_towards(
            &mut value,
            &destination,
            &(3.0.into(), 1.0.into()),
            &1.0,
        );

        assert_eq!(r1, None);
        assert_eq!(value, (4.0, 3.0));
    }

    #[test]
    pub fn test_transition_transform() {
        let mut value = Transform {
            translation: Vec3 {
                x: 3.0,
                y: 4.0,
                z: 5.0,
            },
            rotation: Quat::from_axis_angle(Vec3::X, std::f32::consts::PI),
            scale: Vec3 {
                x: 3.0,
                y: 4.0,
                z: 5.0,
            },
        };

        let destination = Transform {
            translation: Vec3 {
                x: 5.0,
                y: 4.0,
                z: 3.0,
            },
            rotation: Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI),
            scale: Vec3 {
                x: 5.0,
                y: 4.0,
                z: 3.0,
            },
        };

        let r1 = Tweenable::transition_towards(
            &mut value,
            &destination,
            &(1.0.into(), 1.0.into(), 1.0.into()),
            &1.0,
        );

        assert_eq!(r1, None);

        let expected = Transform {
            translation: Vec3::new(3.707_106_8, 4.0, 4.292_893_4),
            rotation: Quat::from_xyzw(0.906_087_4, 0.423_090_5, 0.0, -5.810_021_3e-8),
            scale: Vec3::new(3.707_106_8, 4.0, 4.292_893_4),
        };

        assert_eq!(value, expected);
    }

    #[test]
    pub fn test_transition_color() {
        let mut value = Color::Rgba {
            red: 0.3,
            green: 0.4,
            blue: 0.5,
            alpha: 0.0,
        };

        let destination = Color::Rgba {
            red: 0.6,
            green: 0.8,
            blue: 1.0,
            alpha: 1.0,
        };

        let r1 = Tweenable::transition_towards(&mut value, &destination, &0.1.into(), &1.0);

        assert_eq!(r1, None);

        let expected = Color::Rgba {
            red: 0.4,
            green: 0.5,
            blue: 0.6,
            alpha: 0.1,
        };

        assert_eq!(value, expected);
    }
}
