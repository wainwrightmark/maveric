use std::marker::PhantomData;

pub trait Lens: std::fmt::Debug + Clone  + Send + Sync + 'static {
    type Object;
    type Value: 'static;
}

pub trait GetRefLens: Lens {
    fn get(object: &Self::Object) -> &Self::Value;
}

pub trait GetMutLens: GetRefLens {
    fn get_mut(object: &mut Self::Object) -> &mut Self::Value;
}

pub trait GetValueLens: Lens {
    fn get_value(object: &<Self as Lens>::Object) -> <Self as Lens>::Value;
}

// impl<V: Copy + 'static, L: GetRefLens<Value = V>> GetValueLens for L {
//     fn get_value(object: &<Self as Lens>::Object) -> <Self as Lens>::Value {
//         *L::get(object)
//     }
// }

pub trait SetValueLens: Lens {
    fn set(object: &mut <Self as Lens>::Object, value: <Self as Lens>::Value);
}

impl<L: GetMutLens> SetValueLens for L {
    fn set(object: &mut <Self as Lens>::Object, value: <Self as Lens>::Value) {
        *L::get_mut(object) = value
    }
}

// IdentityLens
#[derive(Debug, Copy, Eq)]
pub struct IdentityLens<T: std::fmt::Debug + Send + Sync + 'static> {
    phantom: PhantomData<T>,
}

impl<T: std::fmt::Debug + Send + Sync + 'static> GetRefLens for IdentityLens<T> {
    fn get(object: &Self::Object) -> &Self::Value {
        object
    }
}

impl<T: std::fmt::Debug + Clone + Send + Sync + 'static> GetValueLens for IdentityLens<T> {
    fn get_value(object: &<Self as Lens>::Object) -> <Self as Lens>::Value {
        object.clone()
    }
}

impl<T: std::fmt::Debug + Send + Sync + 'static> GetMutLens for IdentityLens<T> {
    fn get_mut(object: &mut Self::Object) -> &mut Self::Value {
        object
    }
}

impl<T: std::fmt::Debug + Send + Sync + 'static> Clone for IdentityLens<T> {
    fn clone(&self) -> Self {
        Self {
            phantom: self.phantom.clone(),
        }
    }
}

impl<T: std::fmt::Debug + Send + Sync + 'static> PartialEq for IdentityLens<T> {
    fn eq(&self, other: &Self) -> bool {
        self.phantom == other.phantom
    }
}

impl<T: std::fmt::Debug + Send + Sync + 'static> Lens for IdentityLens<T> {
    type Object = T;
    type Value = T;
}

// PRISMS

// Prism2

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct Prism2<L1: Lens, L2: Lens<Object = L1::Value>> {
    phantom: PhantomData<(L1, L2)>,
}

impl<L1: Lens, L2: Lens<Object = L1::Value>> Lens for Prism2<L1, L2> {
    type Object = L1::Object;
    type Value = L2::Value;
}

impl<L1: GetRefLens, L2: GetRefLens + Lens<Object = L1::Value>> GetRefLens for Prism2<L1, L2> {
    fn get(object: &Self::Object) -> &Self::Value {
        L2::get(L1::get(object))
    }
}

impl<L1: GetRefLens, L2: GetValueLens + Lens<Object = L1::Value>> GetValueLens for Prism2<L1, L2> {
    fn get_value(object: &<Self as Lens>::Object) -> <Self as Lens>::Value {
        L2::get_value(L1::get(object))
    }
}

impl<L1: GetMutLens, L2: Lens<Object = L1::Value> + GetMutLens> GetMutLens for Prism2<L1, L2> {
    fn get_mut(object: &mut Self::Object) -> &mut Self::Value {
        L2::get_mut(L1::get_mut(object))
    }
}

// Prism3

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct Prism3<L1: Lens, L2: Lens<Object = L1::Value>, L3: Lens<Object = L2::Value>> {
    phantom: PhantomData<(L1, L2, L3)>,
}

impl<L1: Lens, L2: Lens<Object = L1::Value>, L3: Lens<Object = L2::Value>> Lens
    for Prism3<L1, L2, L3>
{
    type Object = L1::Object;
    type Value = L3::Value;
}

impl<
        L1: GetRefLens,
        L2: GetRefLens + Lens<Object = L1::Value>,
        L3: GetRefLens + Lens<Object = L2::Value>,
    > GetRefLens for Prism3<L1, L2, L3>
{
    fn get(object: &Self::Object) -> &Self::Value {
        L3::get(L2::get(L1::get(object)))
    }
}

impl<
        L1: GetRefLens,
        L2: GetRefLens + Lens<Object = L1::Value>,
        L3: GetValueLens + Lens<Object = L2::Value>,
    > GetValueLens for Prism3<L1, L2, L3>
{
    fn get_value(object: &<Self as Lens>::Object) -> <Self as Lens>::Value {
        L3::get_value(L2::get(L1::get(object)))
    }
}

impl<
        L1: GetMutLens,
        L2: Lens<Object = L1::Value> + GetMutLens,
        L3: Lens<Object = L2::Value> + GetMutLens,
    > GetMutLens for Prism3<L1, L2, L3>
{
    fn get_mut(object: &mut Self::Object) -> &mut Self::Value {
        L3::get_mut(L2::get_mut(L1::get_mut(object)))
    }
}

// TUPLES

macro_rules! impl_lens {
    ($L0:ident, $($L:ident),*) => {
        impl<$L0 : Lens, $($L : Lens<Object = $L0::Object>),*> Lens for ($L0, $($L,)*) {
            type Object = $L0::Object;
            type Value = ($L0::Value, $($L::Value,)*);
        }
    };
}

macro_rules! impl_get_value_lens {
    ($L0:ident, $($L:ident),*) => {
        impl<$L0 : GetValueLens, $($L : GetValueLens + Lens<Object = $L0::Object>),*> GetValueLens for ($L0, $($L,)*) {
            fn get_value(object: &<Self as Lens>::Object) -> <Self as Lens>::Value {
                ($L0::get_value(object), $($L::get_value(object),)*)
            }
        }
    };
}

macro_rules! impl_set_lens {
    (($L0:ident, $l0:ident), $(($L:ident, $l:ident)),*) => {
        impl<$L0 : SetValueLens, $($L : SetValueLens + Lens<Object = $L0::Object>),*> SetValueLens for ($L0, $($L,)*) {
            fn set(object: &mut <Self as Lens>::Object, value: <Self as Lens>::Value) {
                let ($l0, $($l,)*) = value;


                $L0::set(object, $l0);
                $($L::set(object, $l);)*
            }
        }
    };
}

impl_lens!(L0, L1);
impl_lens!(L0, L1, L2);
impl_lens!(L0, L1, L2, L3);

impl_get_value_lens!(L0, L1);
impl_get_value_lens!(L0, L1, L2);
impl_get_value_lens!(L0, L1, L2, L3);

impl_set_lens!((L0, l0), (L1, l1));
impl_set_lens!((L0, l0), (L1, l1), (L2, l2));
impl_set_lens!((L0, l0), (L1, l1), (L2, l2), (L3, l3));
