use std::{marker::PhantomData, sync::Arc, time::Duration};

use bevy::ecs::component::Component;

use super::{
    lens::{GetValueLens, Lens, SetValueLens},
    prelude::Tweenable,
    step::Transition, ease::Ease,
};

pub trait TransitionBuilderCanBuild<L: Lens + GetValueLens + SetValueLens>:
    Send + Sync + 'static
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn build(self) -> Transition<L>;
}

pub trait TransitionBuilderTrait<L: Lens + GetValueLens + SetValueLens>:
    Send + Sync + 'static
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn build_with_next(&self, next: Transition<L>) -> Transition<L>;
}

pub trait TransitionBuilderCanThen<L: Lens + GetValueLens + SetValueLens>:
    Sized + TransitionBuilderTrait<L>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn then_set_value(self, value: <L as Lens>::Value) -> TransitionBuilderSetValue<L, Self> {
        TransitionBuilderSetValue {
            previous: self,
            value,
        }
    }

    fn then_ease(
        self,
        destination: <L as Lens>::Value,
        speed: <<L as Lens>::Value as Tweenable>::Speed,
        ease: &'static dyn Ease,
    ) -> TransitionBuilderEase<L, Self> {
        TransitionBuilderEase {
            previous: self,
            destination,
            speed,
            ease,
        }
    }

    fn then_tween(
        self,
        destination: <L as Lens>::Value,
        speed: <<L as Lens>::Value as Tweenable>::Speed,
    ) -> TransitionBuilderTween<L, Self> {
        TransitionBuilderTween {
            previous: self,
            destination,
            speed,
        }
    }

    fn then_wait(self, duration: Duration) -> TransitionBuilderWait<L, Self> {
        TransitionBuilderWait {
            previous: self,
            duration,
            phantom: PhantomData,
        }
    }

    fn build_loop(self) -> Transition<L> {
        Transition::Loop(Arc::new(self))
    }
}

impl<L: Lens + GetValueLens + SetValueLens, T: Sized + TransitionBuilderTrait<L>>
    TransitionBuilderCanThen<L> for T
where
    L::Object: Component,
    L::Value: Tweenable,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionBuilder<L: Lens>(PhantomData<L>)
where
    L::Object: Component,
    L::Value: Tweenable;

impl<L: Lens + GetValueLens + SetValueLens> Default for TransitionBuilder<L>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<L: Lens + GetValueLens + SetValueLens> TransitionBuilderTrait<L> for TransitionBuilder<L>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn build_with_next(&self, next: Transition<L>) -> Transition<L> {
        next
    }
}

pub struct TransitionBuilderSetValue<
    L: Lens + GetValueLens + SetValueLens,
    Previous: TransitionBuilderTrait<L>,
> where
    L::Object: Component,
    L::Value: Tweenable,
{
    previous: Previous,
    value: L::Value,
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderTrait<L>>
    TransitionBuilderTrait<L> for TransitionBuilderSetValue<L, Previous>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn build_with_next(&self, next: Transition<L>) -> Transition<L> {
        let t = Transition::SetValue {
            value: self.value.clone(),
            next: Some(Box::new(next)),
        };
        self.previous.build_with_next(t)
    }
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderTrait<L>>
    TransitionBuilderCanBuild<L> for TransitionBuilderSetValue<L, Previous>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn build(self) -> Transition<L> {
        self.previous.build_with_next(Transition::SetValue {
            value: self.value,
            next: None,
        })
    }
}

pub struct TransitionBuilderTween<
    L: Lens + GetValueLens + SetValueLens,
    Previous: TransitionBuilderCanThen<L>,
> where
    L::Object: Component,
    L::Value: Tweenable,
{
    previous: Previous,
    destination: L::Value,
    speed: <<L as Lens>::Value as Tweenable>::Speed,
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderCanThen<L>>
    TransitionBuilderTrait<L> for TransitionBuilderTween<L, Previous>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn build_with_next(&self, next: Transition<L>) -> Transition<L> {
        let this = Transition::TweenValue {
            destination: self.destination.clone(),
            speed: self.speed,
            next: Some(Box::new(next)),
        };
        self.previous.build_with_next(this)
    }
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderCanThen<L>>
    TransitionBuilderCanBuild<L> for TransitionBuilderTween<L, Previous>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn build(self) -> Transition<L> {
        self.previous.build_with_next(Transition::TweenValue {
            destination: self.destination,
            speed: self.speed,
            next: None,
        })
    }
}

pub struct TransitionBuilderEase<
    L: Lens + GetValueLens + SetValueLens,
    Previous: TransitionBuilderCanThen<L>,
> where
    L::Object: Component,
    L::Value: Tweenable,
{
    previous: Previous,
    destination: L::Value,
    speed: <<L as Lens>::Value as Tweenable>::Speed,
    ease: &'static dyn Ease,
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderCanThen<L>>
    TransitionBuilderTrait<L> for TransitionBuilderEase<L, Previous>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn build_with_next(&self, next: Transition<L>) -> Transition<L> {
        let this = Transition::ThenEase {
            destination: self.destination.clone(),
            speed: self.speed,
            next: Some(Box::new(next)),
            ease: self.ease,
        };
        self.previous.build_with_next(this)
    }
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderCanThen<L>>
    TransitionBuilderCanBuild<L> for TransitionBuilderEase<L, Previous>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn build(self) -> Transition<L> {
        self.previous.build_with_next(Transition::ThenEase {
            destination: self.destination,
            speed: self.speed,
            next: None,
            ease: self.ease,
        })
    }
}

pub struct TransitionBuilderWait<
    L: Lens + GetValueLens + SetValueLens,
    Previous: TransitionBuilderCanThen<L>,
> where
    L::Object: Component,
    L::Value: Tweenable,
{
    previous: Previous,
    duration: Duration,
    phantom: PhantomData<L>,
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderCanThen<L>>
    TransitionBuilderTrait<L> for TransitionBuilderWait<L, Previous>
where
    L::Object: Component,
    L::Value: Tweenable,
{
    fn build_with_next(&self, next: Transition<L>) -> Transition<L> {
        let this = Transition::Wait {
            remaining: self.duration,
            next: Some(Box::new(next)),
        };
        self.previous.build_with_next(this)
    }
}
