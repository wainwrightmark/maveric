#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Ease {
    #[default]
    Linear,
    BackIn,
    BackInOut,
    BackOut,
    BounceIn,
    BounceInOut,
    BounceOut,
    CircIn,
    CircInOut,
    CircOut,
    CubicIn,
    CubicInOut,
    CubicOut,
    ElasticIn,
    ElasticInOut,
    ElasticOut,
    ExpoIn,
    ExpoInOut,
    ExpoOut,
    QuadIn,
    QuadInOut,
    QuadOut,
    QuartIn,
    QuartInOut,
    QuartOut,
    QuintIn,
    QuintInOut,
    QuintOut,
    Reverse,
    Roundtrip,
    SineIn,
    SineInOut,
    SineOut,
}

impl Ease {
    #[must_use]
    pub fn ease(&self, t: f32) -> f32 {
        match self {
            Self::Linear => simple_easing::linear(t),
            Self::BackIn => simple_easing::back_in(t),
            Self::BackInOut => simple_easing::back_in_out(t),
            Self::BackOut => simple_easing::back_out(t),
            Self::BounceIn => simple_easing::bounce_in(t),
            Self::BounceInOut => simple_easing::bounce_in_out(t),
            Self::BounceOut => simple_easing::bounce_out(t),
            Self::CircIn => simple_easing::circ_in(t),
            Self::CircInOut => simple_easing::circ_in_out(t),
            Self::CircOut => simple_easing::circ_out(t),
            Self::CubicIn => simple_easing::cubic_in(t),
            Self::CubicInOut => simple_easing::cubic_in_out(t),
            Self::CubicOut => simple_easing::cubic_out(t),
            Self::ElasticIn => simple_easing::elastic_in(t),
            Self::ElasticInOut => simple_easing::elastic_in_out(t),
            Self::ElasticOut => simple_easing::elastic_out(t),
            Self::ExpoIn => simple_easing::expo_in(t),
            Self::ExpoInOut => simple_easing::expo_in_out(t),
            Self::ExpoOut => simple_easing::expo_out(t),
            Self::QuadIn => simple_easing::quad_in(t),
            Self::QuadInOut => simple_easing::quad_in_out(t),
            Self::QuadOut => simple_easing::quad_out(t),
            Self::QuartIn => simple_easing::quart_in(t),
            Self::QuartInOut => simple_easing::quart_in_out(t),
            Self::QuartOut => simple_easing::quart_out(t),
            Self::QuintIn => simple_easing::quint_in(t),
            Self::QuintInOut => simple_easing::quint_in_out(t),
            Self::QuintOut => simple_easing::quint_out(t),
            Self::Reverse => simple_easing::reverse(t),
            Self::Roundtrip => simple_easing::roundtrip(t),
            Self::SineIn => simple_easing::sine_in(t),
            Self::SineInOut => simple_easing::sine_in_out(t),
            Self::SineOut => simple_easing::sine_out(t),
        }
    }
}

// pub trait Ease: Send + Sync  + std::fmt::Debug + 'static {
//     /// Takes `t` in range 0.0..=1.0
//     /// Returns a value on the transition from 0.0 to 1.0 (might go outside this range for a bounce effect)
//     fn ease(&self, t: f32) -> f32;
// }

// //TODO change this to an enum

// macro_rules! impl_ease {
//     ($name: ident, $input: ident, $calculate: expr) => {
//         #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
//         pub struct $name;

//         impl Ease for $name {
//             fn ease(&self, $input: f32) -> f32 {
//                 $calculate
//             }
//         }
//     };
// }

// impl_ease!(EaseLinear, t, t);

// impl_ease!(EaseInCirc, t, 1.0 - (1.0 - t.powi(2)).sqrt());
// impl_ease!(EaseOutCirc, t, (1.0 - (t - 1.0).powi(2)).sqrt());
// impl_ease!(
//     EaseInOutCirc,
//     t,
//     if t < 0.5 {
//         (1.0 - (1.0 - (2.0 * t).powi(2)).sqrt()) / 2.0
//     } else {
//         ((1.0 - (-2.0 * t + 2.0).powi(2)).sqrt() + 1.0) / 2.0
//     }
// );

// impl_ease!(EaseInCubic, t, t * t * t);
// impl_ease!(EaseOutCubic, t, 1.0 - (1.0 - t).powi(3));
// impl_ease!(
//     EaseInOutCubic,
//     t,
//     if t < 0.5 {
//         4.0 * t * t * t
//     } else {
//         1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
//     }
// );

// fn bounce_out(t: f32) -> f32 {
//     const N1: f32 = 7.5625;
//     const D1: f32 = 2.75;
//     if t < 1.0 / D1 {
//         return N1 * t * t;
//     } else if t < 2.0 / D1 {
//         return N1 * (t - 1.5 / D1).powi(2) + 0.75;
//     } else if t < 2.5 / D1 {
//         return N1 * (t - 2.25 / D1).powi(2) + 0.9375;
//     } else {
//         return N1 * (t - 2.625 / D1).powi(2) + 0.984375;
//     }
// }

// impl_ease!(EaseInBounce, t, 1.0 - bounce_out(1.0 - t));
// impl_ease!(EaseOutBounce, t, bounce_out(t));
// impl_ease!(
//     EaseInOutBounce,
//     t,
//     if t < 0.5 {
//         (1.0 - bounce_out(1.0 - 2.0 * t)) / 2.0
//     } else {
//         (1.0 + bounce_out(2.0 * t - 1.0)) / 2.0
//     }
// );
