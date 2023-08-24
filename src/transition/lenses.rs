use crate::transition::prelude::*;
use bevy::prelude::*;

use super::speed::{AngularSpeed, LinearSpeed};

#[macro_export]
macro_rules! define_lens {
    ($L:ident, $O:ident, $V:ident, $p:ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $L;

        impl Lens for $L {
            type Object = $O;
            type Value = $V;
        }

        impl GetRefLens for $L {
            fn try_get_ref(object: &Self::Object) -> Option<&Self::Value> {
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

#[macro_export]
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

pub type TransformRotationXLens = Prism2<TransformRotationLens, QuatXLens>;
pub type TransformRotationYLens = Prism2<TransformRotationLens, QuatYLens>;
pub type TransformRotationZLens = Prism2<TransformRotationLens, QuatZLens>;

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
