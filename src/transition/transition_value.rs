use super::super::prelude::*;
pub use super::prelude::*;
use bevy::prelude::*;

impl<'c, 'a, 'world> ComponentCommands<'c, 'a, 'world> {
    /// Inserts a transition to a particular value
    /// Returns the value that the property should be set to
    pub fn transition_value<L: Lens + GetValueLens + SetValueLens>(
        &mut self,
        destination: L::Value,
        speed: <L::Value as Tweenable>::Speed,
        ease: Option<Ease>,
    ) -> L::Value
    where
        L::Value: Tweenable,
        L::Object: Component,
    {
        let Some(current_value) = self.get::<L::Object>().and_then(L::try_get_value) else {
            return destination;
        };

        if let Some(previous_path) = self.get::<Transition<L>>() {
            if previous_path
                .destination()
                .is_some_and(|d| d == &destination)
            {
                return current_value; //same destination - no need to change anything
            }

            // if let Transition::TweenValue {
            //     destination: old_to,
            //     speed: old_speed,
            //     next: old_next,
            // } = previous_path
            // {
            //     if old_to.eq(&destination) && old_speed.eq(&speed) && old_next.is_none() {
            //         return current_value;
            //     }
            // }

            if current_value.eq(&destination) {
                self.remove::<Transition<L>>();
                return current_value;
            }
        } else if current_value.eq(&destination) {
            return current_value;
        }

        let new_transition = match ease {
            Some(ease) => Transition::<L>::ThenEase {
                destination,
                speed,
                next: None,
                ease,
            },
            None => Transition::<L>::TweenValue {
                destination,
                speed,
                next: None,
            },
        };

        self.insert(new_transition);

        current_value
    }
}
