// use std::{marker::PhantomData, ops::Add, time::Duration};

use crate::transition::prelude::*;
use bevy::prelude::*;

use super::speed::{AngularSpeed, LinearSpeed};

macro_rules! define_lens {
    ($L:ident, $O:ident, $V:ident, $p:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $L;

        impl Lens for $L {
            type Object = $O;
            type Value = $V;
        }

        impl GetRefLens for $L {
            fn get(object: &Self::Object) -> &Self::Value {
                &object.$p
            }
        }

        impl GetValueLens for $L {
            fn get_value(object: &Self::Object) -> Self::Value {
                object.$p
            }
        }

        impl GetMutLens for $L {
            fn get_mut(object: &mut Self::Object) -> &mut Self::Value {
                &mut object.$p
            }
        }
    };
}

macro_rules! define_lens_transparent {
    ($L:ident, $O:ident, $V:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $L;

        impl Lens for $L {
            type Object = $O;
            type Value = $V;
        }

        impl GetRefLens for $L {
            fn get(object: &Self::Object) -> &Self::Value {
                &object.0
            }
        }

        impl GetValueLens for $L {
            fn get_value(object: &Self::Object) -> Self::Value {
                object.0
            }
        }

        impl GetMutLens for $L {
            fn get_mut(object: &mut Self::Object) -> &mut Self::Value {
                &mut object.0
            }
        }
    };
}

define_lens!(TransformTranslationLens, Transform, Vec3, translation);
define_lens!(TransformRotationLens, Transform, Quat, rotation);
define_lens!(TransformScaleLens, Transform, Vec3, scale);
define_lens!(QuatXLens, Quat, f32, x);
define_lens!(QuatYLens, Quat, f32, y);
define_lens!(QuatZLens, Quat, f32, z);

define_lens!(Vec3XLens, Vec3, f32, x);
define_lens!(Vec3YLens, Vec3, f32, y);
define_lens!(Vec3ZLens, Vec3, f32, z);

define_lens!(StyleWidthLens, Style, Val, width);
define_lens!(StyleHeightLens, Style, Val, height);

define_lens!(StyleTopLens, Style, Val, top);
define_lens!(StyleBottomLens, Style, Val, bottom);
define_lens!(StyleLeftLens, Style, Val, left);
define_lens!(StyleRightLens, Style, Val, right);

define_lens_transparent!(BackgroundColorLens, BackgroundColor, Color);
define_lens_transparent!(BorderColorLens, BorderColor, Color);
define_lens!(TextStyleColorLens, TextStyle, Color, color);

// #[derive(Debug, Clone)]
// struct TextStyleLens;

// impl Lens for TextStyleLens{
//     type Object = Text;
//     type Value = TextStyle;
// }

// impl GetRefLens for TextStyleLens{
//     fn get(object: &Self::Object) -> &Self::Value {
//         object.sections.first().map(|x|x.style)
//     }
// }

pub fn transform_speed(
    translation_units_per_second: f32,
    radians_per_second: f32,
    scale_units_per_second: f32,
) -> (LinearSpeed, AngularSpeed, LinearSpeed) {
    (
        LinearSpeed::new(translation_units_per_second),
        AngularSpeed::new(radians_per_second),
        LinearSpeed::new(scale_units_per_second),
    )
}
