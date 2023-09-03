use super::super::prelude::*;
pub use super::prelude::*;
use bevy::prelude::*;

impl<'c, 'w, 's, 'a, 'world> ComponentCommands<'c, 'w, 's, 'a, 'world> {
    /// Inserts a transition to a particular
    pub fn transition_value<L: Lens + GetValueLens>(
        &mut self,
        default_initial_value: L::Value,
        destination: L::Value,
        speed: Option<<L::Value as Tweenable>::Speed>,
    )
        -> L::Value
     where
        L::Value: Tweenable + Clone,
        L::Object: Clone + Component,
    {
        let Some(current_value) = self.get::<L::Object>().and_then(L::try_get_value) else {
            self.insert(Transition::<L> {
                step: TransitionStep::new_arc(default_initial_value.clone(), None, None),
            });
            return default_initial_value;
        };

        if let Some(previous_path) = self.get::<Transition<L>>() {
            if previous_path.step.destination.eq(&destination)
                && previous_path.step.speed == speed
                && previous_path.step.next.is_none()
            {
                return current_value;
            }

            if current_value.eq(&destination) {
                self.remove::<Transition<L>>();
                return current_value;
            }
        } else if current_value.eq(&destination) {
            return current_value;
        }

        self.insert(Transition::<L> {
            step: TransitionStep::new_arc(destination, speed, None),
        });

        return current_value;
    }
}
