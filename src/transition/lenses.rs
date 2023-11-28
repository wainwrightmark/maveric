use std::marker::PhantomData;

use super::speed::{AngularSpeed, LinearSpeed};
use crate::transition::prelude::*;
use bevy::prelude::*;

#[macro_export]
macro_rules! define_lens {
    ($L:ident, $O:ident, $V:ident, $p:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $L;

        impl $crate::transition::prelude::Lens for $L {
            type Object = $O;
            type Value = $V;
        }

        impl $crate::transition::prelude::GetRefLens for $L {
            fn try_get_ref(object: &Self::Object) -> Option<&Self::Value> {
                Some(&object.$p)
            }
        }

        impl $crate::transition::prelude::GetValueLens for $L {
            fn try_get_value(object: &Self::Object) -> Option<Self::Value> {
                Some(object.$p)
            }
        }

        impl $crate::transition::prelude::GetMutLens for $L {
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

        impl $crate::transition::prelude::Lens for $L {
            type Object = $O;
            type Value = $V;
        }

        impl $crate::transition::prelude::GetRefLens for $L {
            fn try_get_ref(object: &Self::Object) -> Option<&Self::Value> {
                Some(&object.0)
            }
        }

        impl $crate::transition::prelude::GetValueLens for $L {
            fn try_get_value(object: &Self::Object) -> Option<Self::Value> {
                Some(object.0)
            }
        }

        impl $crate::transition::prelude::GetMutLens for $L {
            fn try_get_mut(object: &mut Self::Object) -> Option<&mut Self::Value> {
                Some(&mut object.0)
            }
        }
    };
}

define_lens!(TransformTranslationLens, Transform, Vec3, translation);
define_lens!(TransformRotationLens, Transform, Quat, rotation);
define_lens!(TransformScaleLens, Transform, Vec3, scale);

const EULER_ROT: EulerRot = EulerRot::YXZ;

#[derive(Debug, Clone, PartialEq)]
pub struct QuatXLens;

impl Lens for QuatXLens {
    type Object = Quat;
    type Value = f32;
}

impl GetValueLens for QuatXLens {
    fn try_get_value(object: &Self::Object) -> Option<Self::Value> {
        let (_y, x, _z) = object.to_euler(EULER_ROT);
        Some(x)
    }
}

impl SetValueLens for QuatXLens {
    fn try_set(object: &mut <Self as Lens>::Object, value: <Self as Lens>::Value) {
        let (y, _x, z) = object.to_euler(EULER_ROT);
        let new_quat = Quat::from_euler(EULER_ROT, y, value, z);
        *object = new_quat
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuatYLens;

impl Lens for QuatYLens {
    type Object = Quat;
    type Value = f32;
}

impl GetValueLens for QuatYLens {
    fn try_get_value(object: &Self::Object) -> Option<Self::Value> {
        let (y, _x, _z) = object.to_euler(EULER_ROT);
        Some(y)
    }
}

impl SetValueLens for QuatYLens {
    fn try_set(object: &mut <Self as Lens>::Object, value: <Self as Lens>::Value) {
        let (_y, x, z) = object.to_euler(EULER_ROT);
        let new_quat = Quat::from_euler(EULER_ROT, value, x, z);
        *object = new_quat
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuatZLens;

impl Lens for QuatZLens {
    type Object = Quat;
    type Value = f32;
}

impl GetValueLens for QuatZLens {
    fn try_get_value(object: &Self::Object) -> Option<Self::Value> {
        let (_y, _x, z) = object.to_euler(EULER_ROT);
        Some(z)
    }
}

impl SetValueLens for QuatZLens {
    fn try_set(object: &mut <Self as Lens>::Object, value: <Self as Lens>::Value) {
        let (y, x, _) = object.to_euler(EULER_ROT);
        let new_quat = Quat::from_euler(EULER_ROT, y, x, value);
        *object = new_quat
    }
}

define_lens!(Vec3XLens, Vec3, f32, x);
define_lens!(Vec3YLens, Vec3, f32, y);
define_lens!(Vec3ZLens, Vec3, f32, z);

pub type TransformRotationXLens = Prism2<TransformRotationLens, QuatXLens>;
pub type TransformRotationYLens = Prism2<TransformRotationLens, QuatYLens>;
pub type TransformRotationZLens = Prism2<TransformRotationLens, QuatZLens>;

impl SetValueLens for TransformRotationXLens {
    fn try_set(object: &mut <Self as Lens>::Object, value: <Self as Lens>::Value) {
        QuatXLens::try_set(&mut object.rotation, value)
    }
}

impl SetValueLens for TransformRotationYLens {
    fn try_set(object: &mut <Self as Lens>::Object, value: <Self as Lens>::Value) {
        QuatYLens::try_set(&mut object.rotation, value)
    }
}

impl SetValueLens for TransformRotationZLens {
    fn try_set(object: &mut <Self as Lens>::Object, value: <Self as Lens>::Value) {
        QuatZLens::try_set(&mut object.rotation, value)
    }
}

#[must_use]
pub const fn transform_speed(
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

#[derive(Debug, PartialEq, Clone)]
pub struct ElementAtLens<
    const INDEX: usize,
    const ARRAY_LEN: usize,
    Element: PartialEq + Clone + Copy + std::fmt::Debug + Send + Sync + 'static,
>(PhantomData<Element>);

impl<
        const INDEX: usize,
        const ARRAY_LEN: usize,
        Element: PartialEq + Clone + Copy + std::fmt::Debug + Send + Sync + 'static,
    > GetRefLens for ElementAtLens<INDEX, ARRAY_LEN, Element>
{
    fn try_get_ref(object: &Self::Object) -> Option<&Self::Value> {
        object.get(INDEX)
    }
}

impl<
        const INDEX: usize,
        const ARRAY_LEN: usize,
        Element: PartialEq + Clone + Copy + std::fmt::Debug + Send + Sync + 'static,
    > GetMutLens for ElementAtLens<INDEX, ARRAY_LEN, Element>
{
    fn try_get_mut(object: &mut Self::Object) -> Option<&mut Self::Value> {
        object.get_mut(INDEX)
    }
}

impl<
        const INDEX: usize,
        const ARRAY_LEN: usize,
        Element: PartialEq + Clone + Copy + std::fmt::Debug + Send + Sync + 'static,
    > GetValueLens for ElementAtLens<INDEX, ARRAY_LEN, Element>
{
    fn try_get_value(object: &<Self as Lens>::Object) -> Option<<Self as Lens>::Value> {
        object.get(INDEX).map(|x| *x)
    }
}

impl<
        const INDEX: usize,
        const ARRAY_LEN: usize,
        Element: PartialEq + Clone + Copy + std::fmt::Debug + Send + Sync + 'static,
    > Lens for ElementAtLens<INDEX, ARRAY_LEN, Element>
{
    type Object = [Element; ARRAY_LEN];

    type Value = Element;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use bevy::prelude::Quat;

    #[test]
    pub fn test_rotation_lens() {
        let mut quat = Quat::from_rotation_x(1.0);

        assert_eq!(QuatZLens::try_get_value(&quat), Some(0.0));

        QuatZLens::try_set(&mut quat, 2.0);

        assert_eq!(QuatZLens::try_get_value(&quat), Some(2.0000002));

        assert_eq!(QuatXLens::try_get_value(&quat), Some(1.0000001));

        QuatYLens::try_set(&mut quat, 3.0);

        assert_eq!(QuatZLens::try_get_value(&quat), Some(2.0));

        assert_eq!(QuatXLens::try_get_value(&quat), Some(1.0000001));
        assert_eq!(QuatYLens::try_get_value(&quat), Some(3.0));
    }

    #[test]
    pub fn test_element_at_lens() {
        type L0 = ElementAtLens::<0,2, f64>;
        type L1 = ElementAtLens::<1,2, f64>;
        let mut array = [0.0,1.0];

        assert_eq!(L0::try_get_value(&array), Some(0.0));
        assert_eq!(L1::try_get_value(&array), Some(1.0));

        L1::try_set(&mut array, 2.0);

        assert_eq!(L0::try_get_value(&array), Some(0.0)); //value should not have changed
        assert_eq!(L1::try_get_value(&array), Some(2.0));
    }
}
