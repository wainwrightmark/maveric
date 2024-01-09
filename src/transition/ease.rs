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
    pub fn ease(&self, t: f32) -> f32 {
        match self {
            Ease::Linear => simple_easing::linear(t),
            Ease::BackIn => simple_easing::back_in(t),
            Ease::BackInOut => simple_easing::back_in_out(t),
            Ease::BackOut => simple_easing::back_out(t),
            Ease::BounceIn => simple_easing::bounce_in(t),
            Ease::BounceInOut => simple_easing::bounce_in_out(t),
            Ease::BounceOut => simple_easing::bounce_out(t),
            Ease::CircIn => simple_easing::circ_in(t),
            Ease::CircInOut => simple_easing::circ_in_out(t),
            Ease::CircOut => simple_easing::circ_out(t),
            Ease::CubicIn => simple_easing::cubic_in(t),
            Ease::CubicInOut => simple_easing::cubic_in_out(t),
            Ease::CubicOut => simple_easing::cubic_out(t),
            Ease::ElasticIn => simple_easing::elastic_in(t),
            Ease::ElasticInOut => simple_easing::elastic_in_out(t),
            Ease::ElasticOut => simple_easing::elastic_out(t),
            Ease::ExpoIn => simple_easing::expo_in(t),
            Ease::ExpoInOut => simple_easing::expo_in_out(t),
            Ease::ExpoOut => simple_easing::expo_out(t),
            Ease::QuadIn => simple_easing::quad_in(t),
            Ease::QuadInOut => simple_easing::quad_in_out(t),
            Ease::QuadOut => simple_easing::quad_out(t),
            Ease::QuartIn => simple_easing::quart_in(t),
            Ease::QuartInOut => simple_easing::quart_in_out(t),
            Ease::QuartOut => simple_easing::quart_out(t),
            Ease::QuintIn => simple_easing::quint_in(t),
            Ease::QuintInOut => simple_easing::quint_in_out(t),
            Ease::QuintOut => simple_easing::quint_out(t),
            Ease::Reverse => simple_easing::reverse(t),
            Ease::Roundtrip => simple_easing::roundtrip(t),
            Ease::SineIn => simple_easing::sine_in(t),
            Ease::SineInOut => simple_easing::sine_in_out(t),
            Ease::SineOut => simple_easing::sine_out(t),
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
