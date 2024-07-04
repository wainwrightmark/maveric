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
        delta_seconds: f32,
    ) -> Option<f32>;

    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[must_use]
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
        delta_seconds: f32,
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

macro_rules! impl_tweenable_vec {
    ($ty: ty) => {
        impl Tweenable for $ty {
            type Speed = LinearSpeed;

            fn transition_towards(
                &mut self,
                destination: &Self,
                speed: &Self::Speed,
                delta_seconds: f32,
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
    };
}

impl_tweenable_vec!(Vec2);
impl_tweenable_vec!(Vec3);
impl_tweenable_vec!(Vec4);

impl Tweenable for Quat {
    type Speed = AngularSpeed;

    fn transition_towards(
        &mut self,
        destination: &Self,
        speed: &Self::Speed,
        delta_seconds: f32,
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
        delta_seconds: f32,
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
        delta_seconds: f32,
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
                Some(delta_seconds)
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

macro_rules! impl_tweenable_for_color {
    ($type: ty) => {
        impl Tweenable for $type {
            type Speed = ScalarSpeed;
            fn duration_to(
                &self,
                rhs: &Self,
                speed: &Self::Speed,
            ) -> Result<Duration, TryFromFloatSecsError> {
                self.to_f32_array().duration_to(&rhs.to_f32_array(), speed)
            }
            fn transition_towards(
                &mut self,
                destination: &Self,
                speed: &Self::Speed,
                delta_seconds: f32,
            ) -> Option<f32> {
                let mut self_as_vec4 = self.to_f32_array();
                let r: Option<f32> = self_as_vec4.transition_towards(
                    &destination.to_f32_array(),
                    speed,
                    delta_seconds,
                );
                *self = Self::from_f32_array(self_as_vec4);
                r
            }
            fn lerp_value(&self, rhs: &Self, s: f32) -> Self {
                let r = self.to_f32_array().lerp_value(&rhs.to_f32_array(), s);
                Self::from_f32_array(r)
            }
        }
    };
}

//todo implement for all colors, use a different color space as default

impl_tweenable_for_color!(LinearRgba);
impl_tweenable_for_color!(Srgba);
impl_tweenable_for_color!(Hsla);
impl_tweenable_for_color!(Hsva);
impl_tweenable_for_color!(Hwba);
impl_tweenable_for_color!(Laba);
impl_tweenable_for_color!(Lcha);
impl_tweenable_for_color!(Oklaba);
impl_tweenable_for_color!(Oklcha);
impl_tweenable_for_color!(Xyza);

impl Tweenable for Color {
    type Speed = ScalarSpeed;

    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError> {
        match rhs {
            Color::Srgba(c) => Srgba::from(*self).duration_to(c, speed),
            Color::LinearRgba(c) => LinearRgba::from(*self).duration_to(c, speed),
            Color::Hsla(c) => Hsla::from(*self).duration_to(c, speed),
            Color::Hsva(c) => Hsva::from(*self).duration_to(c, speed),
            Color::Hwba(c) => Hwba::from(*self).duration_to(c, speed),
            Color::Laba(c) => Laba::from(*self).duration_to(c, speed),
            Color::Lcha(c) => Lcha::from(*self).duration_to(c, speed),
            Color::Oklaba(c) => Oklaba::from(*self).duration_to(c, speed),
            Color::Oklcha(c) => Oklcha::from(*self).duration_to(c, speed),
            Color::Xyza(c) => Xyza::from(*self).duration_to(c, speed),
        }
    }

    fn transition_towards(
        &mut self,
        destination: &Self,
        speed: &Self::Speed,
        delta_seconds: f32,
    ) -> Option<f32> {
        match destination {
            Color::Srgba(dest) => {
                if let Color::Srgba(lhs) = self {
                    lhs.transition_towards(&dest, speed, delta_seconds)
                } else {
                    let mut s = Srgba::from(*self);
                    let r = s.transition_towards(&dest, speed, delta_seconds);
                    *self = s.into();
                    r
                }
            }
            Color::LinearRgba(dest) => {
                if let Color::LinearRgba(lhs) = self {
                    lhs.transition_towards(&dest, speed, delta_seconds)
                } else {
                    let mut s = LinearRgba::from(*self);
                    let r = s.transition_towards(&dest, speed, delta_seconds);
                    *self = s.into();
                    r
                }
            }
            Color::Hsla(dest) => {
                if let Color::Hsla(lhs) = self {
                    lhs.transition_towards(&dest, speed, delta_seconds)
                } else {
                    let mut s = Hsla::from(*self);
                    let r = s.transition_towards(&dest, speed, delta_seconds);
                    *self = s.into();
                    r
                }
            }
            Color::Hsva(dest) => {
                if let Color::Hsva(lhs) = self {
                    lhs.transition_towards(&dest, speed, delta_seconds)
                } else {
                    let mut s = Hsva::from(*self);
                    let r = s.transition_towards(&dest, speed, delta_seconds);
                    *self = s.into();
                    r
                }
            }
            Color::Hwba(dest) => {
                if let Color::Hwba(lhs) = self {
                    lhs.transition_towards(&dest, speed, delta_seconds)
                } else {
                    let mut s = Hwba::from(*self);
                    let r = s.transition_towards(&dest, speed, delta_seconds);
                    *self = s.into();
                    r
                }
            }
            Color::Laba(dest) => {
                if let Color::Laba(lhs) = self {
                    lhs.transition_towards(&dest, speed, delta_seconds)
                } else {
                    let mut s = Laba::from(*self);
                    let r = s.transition_towards(&dest, speed, delta_seconds);
                    *self = s.into();
                    r
                }
            }
            Color::Lcha(dest) => {
                if let Color::Lcha(lhs) = self {
                    lhs.transition_towards(&dest, speed, delta_seconds)
                } else {
                    let mut s = Lcha::from(*self);
                    let r = s.transition_towards(&dest, speed, delta_seconds);
                    *self = s.into();
                    r
                }
            }
            Color::Oklaba(dest) => {
                if let Color::Oklaba(lhs) = self {
                    lhs.transition_towards(&dest, speed, delta_seconds)
                } else {
                    let mut s = Oklaba::from(*self);
                    let r = s.transition_towards(&dest, speed, delta_seconds);
                    *self = s.into();
                    r
                }
            }
            Color::Oklcha(dest) => {
                if let Color::Oklcha(lhs) = self {
                    lhs.transition_towards(&dest, speed, delta_seconds)
                } else {
                    let mut s = Oklcha::from(*self);
                    let r = s.transition_towards(&dest, speed, delta_seconds);
                    *self = s.into();
                    r
                }
            }
            Color::Xyza(dest) => {
                if let Color::Xyza(lhs) = self {
                    lhs.transition_towards(&dest, speed, delta_seconds)
                } else {
                    let mut s = Xyza::from(*self);
                    let r = s.transition_towards(&dest, speed, delta_seconds);
                    *self = s.into();
                    r
                }
            }
        }
    }

    fn lerp_value(&self, rhs: &Self, s: f32) -> Self {
        match rhs {
            Color::Srgba(c) => Srgba::from(*self).lerp_value(c, s).into(),
            Color::LinearRgba(c) => LinearRgba::from(*self).lerp_value(c, s).into(),
            Color::Hsla(c) => Hsla::from(*self).lerp_value(c, s).into(),
            Color::Hsva(c) => Hsva::from(*self).lerp_value(c, s).into(),
            Color::Hwba(c) => Hwba::from(*self).lerp_value(c, s).into(),
            Color::Laba(c) => Laba::from(*self).lerp_value(c, s).into(),
            Color::Lcha(c) => Lcha::from(*self).lerp_value(c, s).into(),
            Color::Oklaba(c) => Oklaba::from(*self).lerp_value(c, s).into(),
            Color::Oklcha(c) => Oklcha::from(*self).lerp_value(c, s).into(),
            Color::Xyza(c) => Xyza::from(*self).lerp_value(c, s).into(),
        }
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

            fn transition_towards(&mut self, destination: &Self, speed: &Self::Speed, delta_seconds: f32) -> Option<f32> {
                let ($($t,)*) = self;
                let ($($r,)*) = destination;
                let ($($s,)*) = speed;

                let mut remaining: Option<f32> = Some(delta_seconds);
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

impl<const N: usize, T: Tweenable> Tweenable for [T; N] {
    type Speed = T::Speed;

    fn duration_to(
        &self,
        rhs: &Self,
        speed: &Self::Speed,
    ) -> Result<Duration, TryFromFloatSecsError> {
        let mut current = Duration::ZERO;

        for (l, r) in self.into_iter().zip(rhs.into_iter()) {
            let d = l.duration_to(r, speed)?;

            if d > current {
                current = d;
            }
        }
        Ok(current)
    }

    fn transition_towards(
        &mut self,
        destination: &Self,
        speed: &Self::Speed,
        delta_seconds: f32,
    ) -> Option<f32> {
        let mut remaining = Some(delta_seconds);

        for (l, r) in self.into_iter().zip(destination.into_iter()) {
            match l.transition_towards(r, speed, delta_seconds) {
                Some(new_rem) => {
                    if let Some(rem1) = remaining {
                        if new_rem < rem1 {
                            remaining = Some(new_rem);
                        }
                    }
                }
                None => remaining = None,
            }
        }

        remaining
    }

    fn lerp_value(&self, rhs: &Self, s: f32) -> Self {
        let mut l = self.clone();

        for i in 0..N {
            l[i] = l[i].lerp_value(&rhs[i], s);
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use bevy::{
        color::*,
        math::{Quat, Vec3},
        transform::components::Transform,
    };

    use super::Tweenable;

    #[test]
    pub fn test_transition_tuple() {
        let mut value = (1.0, 2.0);

        let destination = (5.0, 5.0);

        let r1 =
            Tweenable::transition_towards(&mut value, &destination, &(3.0.into(), 1.0.into()), 1.0);

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
            1.0,
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
        let mut value = Color::LinearRgba(LinearRgba {
            red: 0.3,
            green: 0.4,
            blue: 0.5,
            alpha: 0.0,
        });

        let destination = Color::LinearRgba(LinearRgba {
            red: 0.6,
            green: 0.8,
            blue: 1.0,
            alpha: 1.0,
        });

        let r1 = Tweenable::transition_towards(&mut value, &destination, &0.1.into(), 1.0);

        assert_eq!(r1, None);

        let expected = Color::LinearRgba(LinearRgba {
            red: 0.4,
            green: 0.5,
            blue: 0.6,
            alpha: 0.1,
        });

        assert_eq!(value, expected);
    }
}
