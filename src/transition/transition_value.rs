use super::super::prelude::*;
pub use super::prelude::*;
use bevy::prelude::*;

impl<'c, 'w, 's, 'a, 'world> ComponentCommands<'c, 'w, 's, 'a, 'world> {
    /// Inserts a transition to a particular destination
    pub fn transition_value<L: Lens + GetValueLens>(
        &mut self,
        default_initial_value: L::Value,
        destination: L::Value,
        speed: Option<<L::Value as Tweenable>::Speed>,
    ) where
        L::Value: Tweenable,
        L::Object: Clone + Component,
    {
        let Some(current_value) = self.get::<L::Object>().and_then(L::try_get_value) else {
            self.insert(Transition::<L> {
                step: TransitionStep::new_arc(default_initial_value, None, None),
            });
            return;
        };

        if let Some(previous_path) = self.get::<Transition<L>>() {
            if previous_path.step.destination.eq(&destination)
                && previous_path.step.speed == speed
                && previous_path.step.next.is_none()
            {
                return;
            }

            if current_value.eq(&destination) {
                self.remove::<Transition<L>>();
                return;
            }
        } else if current_value.eq(&destination) {
            return;
        }

        self.insert(Transition::<L> {
            step: TransitionStep::new_arc(destination, speed, None),
        });
    }
}
