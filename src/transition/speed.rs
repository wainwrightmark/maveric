use std::{fmt::Display, time::Duration};

use super::tweenable::Tweenable;

pub trait Speed: std::fmt::Debug + Copy + Clone + PartialEq + Send + Sync + 'static {
    const ONE_PER_SECOND: Self;

    const INFINITE: Self;

    fn mul(self, rhs: f32) -> Self;
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct ScalarSpeed {
    pub amount_per_second: f32,
}

impl Display for ScalarSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.amount_per_second.fmt(f)?;
        f.write_str("/s")
    }
}

impl Speed for ScalarSpeed {
    const ONE_PER_SECOND: Self = Self {
        amount_per_second: 1.0,
    };

    const INFINITE: Self = Self {
        amount_per_second: f32::INFINITY,
    };

    fn mul(self, rhs: f32) -> Self {
        Self {
            amount_per_second: self.amount_per_second * rhs,
        }
    }
}

impl ScalarSpeed {
    pub fn new(amount_per_second: f32) -> Self {
        Self { amount_per_second }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct LinearSpeed {
    pub units_per_second: f32,
}

impl Display for LinearSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.units_per_second.fmt(f)?;
        f.write_str("/s")
    }
}

impl Speed for LinearSpeed {
    const ONE_PER_SECOND: Self = Self {
        units_per_second: 1.0,
    };

    const INFINITE: Self = Self {
        units_per_second: f32::INFINITY,
    };

    fn mul(self, rhs: f32) -> Self {
        Self {
            units_per_second: self.units_per_second * rhs,
        }
    }
}

impl LinearSpeed {
    pub fn new(units_per_second: f32) -> Self {
        Self { units_per_second }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct AngularSpeed {
    pub radians_per_second: f32,
}

impl Display for AngularSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.radians_per_second.fmt(f)?;
        f.write_str("/s")
    }
}

impl Speed for AngularSpeed {
    const ONE_PER_SECOND: Self = Self {
        radians_per_second: 1.0,
    };

    const INFINITE: Self = Self {
        radians_per_second: f32::INFINITY,
    };

    fn mul(self, rhs: f32) -> Self {
        Self {
            radians_per_second: self.radians_per_second * rhs,
        }
    }
}

impl AngularSpeed {
    pub fn new(radians_per_second: f32) -> Self {
        Self { radians_per_second }
    }
}

macro_rules! impl_speed {
    ($(($T:ident, $t:ident)),*) => {
        impl<$($T : Speed,)*> Speed for ($($T,)*) {

            const ONE_PER_SECOND: Self =($($T::ONE_PER_SECOND,)*);

            const INFINITE: Self =($($T::INFINITE,)*);



            fn mul(self, _rhs: f32) -> Self {
                let ($($t,)*)= self;
                ($($t.mul(_rhs),)*)
            }

        }
    };
}

impl_speed!();
impl_speed!((T0, t0));
impl_speed!((T0, t0), (T1, t1));
impl_speed!((T0, t0), (T1, t1), (T2, t2));
impl_speed!((T0, t0), (T1, t1), (T2, t2), (T3, t3));

pub fn calculate_speed<T: Tweenable>(t1: &T, t2: &T, duration: Duration) -> T::Speed {
    let time_to_run = t1
        .duration_to(t2, &<T::Speed as Speed>::ONE_PER_SECOND)
        .unwrap();

    let multiplier = time_to_run.as_secs_f32() / duration.as_secs_f32();

    <T::Speed as Speed>::ONE_PER_SECOND.mul(multiplier)
}
