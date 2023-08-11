use crate::widgets::prelude::ComponentCommands;

pub use super::prelude::*;
use bevy::prelude::*;

pub trait TransitionComponentCommands: ComponentCommands {
    /// Inserts a transition
    fn transition_value<L: Lens + GetValueLens>(
        &mut self,
        default_initial_value: L::Value,
        destination: L::Value,
        speed: Option<<L::Value as Tweenable>::Speed>,
    ) -> L::Value
    where
        L::Value: Tweenable,
        L::Object: Clone + Component,
    {
         let current_value = match self.get::<L::Object>(){
            Some(o) => L::try_get_value(o).unwrap_or(default_initial_value),
            None => default_initial_value,
        };

        if let Some(previous_path) = self.get::<Transition<L>>(){
            if previous_path.step.destination.approx_eq(&destination) && previous_path.step.speed == speed && previous_path.step.next.is_none(){
                return current_value; //previous path is the same - do not replace
            }

            if current_value.approx_eq(&destination){
                self.remove::<Transition<L>>();
                return current_value;
            }
        }
        else {
            if current_value.approx_eq(&destination){
                return current_value;
            }
        }

        self.insert(Transition::<L>{
            step: TransitionStep::new_arc(destination, speed, None)
        });

         current_value
    }
}

impl<CC: ComponentCommands> TransitionComponentCommands for CC {}
