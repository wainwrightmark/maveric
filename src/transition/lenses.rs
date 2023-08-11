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
            fn try_get_ref(object: &Self::Object) -> Option<&Self::Value>  {
                Some(&object.$p)
            }
        }

        impl GetValueLens for $L {
            fn try_get_value(object: &Self::Object) -> Option<Self::Value> {
                Some(object.$p)
            }
        }

        impl GetMutLens for $L {
            fn try_get_mut(object: &mut Self::Object) -> Option<&mut Self::Value> {
                Some(&mut object.$p)
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
            fn try_get_ref(object: &Self::Object) -> Option<&Self::Value> {
                Some(&object.0)
            }
        }

        impl GetValueLens for $L {
            fn try_get_value(object: &Self::Object) -> Option<Self::Value> {
                Some(object.0)
            }
        }

        impl GetMutLens for $L {
            fn try_get_mut(object: &mut Self::Object) -> Option<&mut Self::Value> {
                Some(&mut object.0)
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

pub type TextColorLens<const SECTION: usize> = Prism2<TextStyleLens<SECTION>, TextStyleColorLens>;

#[derive(Debug, Clone)]
pub struct TextStyleLens<const SECTION: usize>;

impl<const SECTION: usize> Lens for TextStyleLens<SECTION>{
    type Object = Text;
    type Value = TextStyle;
}

impl<const SECTION: usize> GetRefLens for TextStyleLens<SECTION>{
    fn try_get_ref(object: &Self::Object) -> Option<&Self::Value> {
        object.sections.get(SECTION).map(|x|&x.style)
    }
}

impl<const SECTION: usize> GetMutLens for TextStyleLens<SECTION>{
    fn try_get_mut(object: &mut Self::Object) -> Option<&mut Self::Value> {
        object.sections.get_mut(SECTION).map(|x|&mut x.style)
    }
}


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
