pub trait Ease: Send + Sync  + std::fmt::Debug + 'static {
    /// Takes `t` in range 0.0..=1.0
    /// Returns a value on the transition from 0.0 to 1.0 (might go outside this range for a bounce effect)
    fn ease(&self, t: f32) -> f32;
}

//TODO change this to an enum

macro_rules! impl_ease {
    ($name: ident, $input: ident, $calculate: expr) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name;

        impl Ease for $name {
            fn ease(&self, $input: f32) -> f32 {
                $calculate
            }
        }
    };
}

impl_ease!(EaseLinear, t, t);

impl_ease!(EaseInCirc, t, 1.0 - (1.0 - t.powi(2)).sqrt());
impl_ease!(EaseOutCirc, t, (1.0 - (t - 1.0).powi(2)).sqrt());
impl_ease!(
    EaseInOutCirc,
    t,
    if t < 0.5 {
        (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
    } else {
        ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
    }
);

impl_ease!(EaseInCubic, t, t * t * t);
impl_ease!(EaseOutCubic, t, 1.0 - (1.0 - t).powi(3));
impl_ease!(
    EaseInOutCubic,
    t,
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
);

fn bounce_out(t: f32) -> f32 {
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;
    if t < 1.0 / D1 {
        return N1 * t * t;
    } else if t < 2.0 / D1 {
        return N1 * (t - 1.5 / D1).powi(2) + 0.75;
    } else if t < 2.5 / D1 {
        return N1 * (t - 2.25 / D1).powi(2) + 0.9375;
    } else {
        return N1 * (t - 2.625 / D1).powi(2) + 0.984375;
    }
}

impl_ease!(EaseInBounce, t, 1.0 - bounce_out(1.0 - t));
impl_ease!(EaseOutBounce, t, bounce_out(t));
impl_ease!(
    EaseInOutBounce,
    t,
    if t < 0.5 {
        (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
    }
);
