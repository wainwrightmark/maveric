use std::{marker::PhantomData, sync::Arc, time::Duration};

use super::{
    ease::Ease,
    lens::{GetValueLens, Lens, SetValueLens},
    prelude::Tweenable,
    speed::calculate_speed,
    step::Transition,
};

pub trait TransitionBuilderCanBuild<L: Lens + GetValueLens + SetValueLens>:
    Send + Sync + 'static
where
    L::Value: Tweenable,
{
    fn build(self) -> Transition<L>;
}

pub trait TransitionBuilderTrait<L: Lens + GetValueLens + SetValueLens>:
    Send + Sync + 'static
where
    L::Value: Tweenable,
{
    fn build_with_next(&self, next: Transition<L>) -> Transition<L>;
}

pub trait TransitionBuilderWithValue<L: Lens + GetValueLens + SetValueLens>:
    TransitionBuilderTrait<L> + Send + Sync + 'static
where
    L::Value: Tweenable,
{
    /// The value after this transition completes
    fn get_value(&self) -> &L::Value;
}

pub trait TransitionBuilderCanThen<L: Lens + GetValueLens + SetValueLens>:
    Sized + TransitionBuilderTrait<L>
where
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
        ease: Ease,
    ) -> TransitionBuilderEase<L, Self> {
        TransitionBuilderEase {
            previous: self,
            destination,
            speed,
            ease,
        }
    }

    fn then_ease_with_duration(
        self,
        destination: <L as Lens>::Value,
        duration: Duration,
        ease: Ease,
    ) -> TransitionBuilderEase<L, Self>
    where
        Self: TransitionBuilderWithValue<L>,
    {
        let current = self.get_value();
        let speed = calculate_speed(current, &destination, duration);

        self.then_ease(destination, speed, ease)
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

    fn then_tween_with_duration(
        self,
        destination: <L as Lens>::Value,
        duration: Duration,
    ) -> TransitionBuilderTween<L, Self>
    where
        Self: TransitionBuilderWithValue<L>,
    {
        let current = self.get_value();
        let speed = calculate_speed(current, &destination, duration);

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
    L::Value: Tweenable,
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransitionBuilder<L: Lens>(PhantomData<L>)
where
    L::Value: Tweenable;

impl<L: Lens + GetValueLens + SetValueLens> Default for TransitionBuilder<L>
where
    L::Value: Tweenable,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<L: Lens + GetValueLens + SetValueLens> TransitionBuilderTrait<L> for TransitionBuilder<L>
where
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
    L::Value: Tweenable,
{
    previous: Previous,
    value: L::Value,
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderTrait<L>>
    TransitionBuilderWithValue<L> for TransitionBuilderSetValue<L, Previous>
where
    L::Value: Tweenable,
{
    fn get_value(&self) -> &<L as Lens>::Value {
        &self.value
    }
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderTrait<L>>
    TransitionBuilderTrait<L> for TransitionBuilderSetValue<L, Previous>
where
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
    L::Value: Tweenable,
{
    previous: Previous,
    destination: L::Value,
    speed: <<L as Lens>::Value as Tweenable>::Speed,
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderCanThen<L>>
    TransitionBuilderWithValue<L> for TransitionBuilderTween<L, Previous>
where
    L::Value: Tweenable,
{
    fn get_value(&self) -> &L::Value {
        &self.destination
    }
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderCanThen<L>>
    TransitionBuilderTrait<L> for TransitionBuilderTween<L, Previous>
where
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
    L::Value: Tweenable,
{
    previous: Previous,
    destination: L::Value,
    speed: <<L as Lens>::Value as Tweenable>::Speed,
    ease: Ease,
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderCanThen<L>>
    TransitionBuilderWithValue<L> for TransitionBuilderEase<L, Previous>
where
    L::Value: Tweenable,
{
    fn get_value(&self) -> &L::Value {
        &self.destination
    }
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderCanThen<L>>
    TransitionBuilderTrait<L> for TransitionBuilderEase<L, Previous>
where
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
    L::Value: Tweenable,
{
    previous: Previous,
    duration: Duration,
    phantom: PhantomData<L>,
}

impl<
        L: Lens + GetValueLens + SetValueLens,
        Previous: TransitionBuilderCanThen<L> + TransitionBuilderWithValue<L>,
    > TransitionBuilderWithValue<L> for TransitionBuilderWait<L, Previous>
where
    L::Value: Tweenable,
{
    fn get_value(&self) -> &L::Value {
        self.previous.get_value()
    }
}

impl<L: Lens + GetValueLens + SetValueLens, Previous: TransitionBuilderCanThen<L>>
    TransitionBuilderTrait<L> for TransitionBuilderWait<L, Previous>
where
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

#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    use bevy::{math::Vec3, transform::components::Transform};

    use crate::widgets::prelude::TransformTranslationLens;

    use super::{TransitionBuilder, TransitionBuilderCanBuild, TransitionBuilderCanThen};

    #[test]
    pub fn test_transition_builder() {
        let mut transition: crate::widgets::prelude::Transition<TransformTranslationLens> =
            TransitionBuilder::<TransformTranslationLens>::default()
                .then_wait(Duration::from_secs(2))
                .then_set_value(Vec3::ONE)
                .then_wait(Duration::from_secs(2))
                .then_tween_with_duration(Vec3::splat(3.0), Duration::from_secs(2))
                .build();

        let expected_values: Vec<Vec3> = vec![0, 0, 1, 1, 1, 2, 3]
            .into_iter()
            .map(|x| x as f32 * Vec3::ONE)
            .collect();

        let mut actual_values: Vec<Vec3> = vec![];

        let mut transform = Transform::from_translation(Vec3::ZERO);
        for _ in 0..5 {
            actual_values.push(transform.translation);

            let should_delete = transition.step(&mut transform, Duration::from_secs(1));
            assert!(!should_delete);
        }

        actual_values.push(transform.translation);

        let should_delete = transition.step(&mut transform, Duration::from_secs(1));
        assert!(should_delete);

        actual_values.push(transform.translation);

        assert_eq!(expected_values, actual_values);
    }
}
