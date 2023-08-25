pub mod deletion_path_maker;
pub mod lens;
pub mod lenses;
pub mod plugin;
pub mod speed;
pub mod step;
pub mod transition_value;
pub mod tweenable;
pub mod with;

#[cfg(feature = "more_bevy")]
pub mod ui_lenses;

pub mod prelude {
    pub use crate::transition::deletion_path_maker::*;
    pub use crate::transition::lens::*;
    pub use crate::transition::lenses::*;
    pub use crate::transition::plugin::*;
    pub use crate::transition::step::*;
    pub use crate::transition::transition_value::*;
    pub use crate::transition::tweenable::*;
    pub use crate::transition::with::*;

    #[cfg(feature = "more_bevy")]
    pub use crate::transition::ui_lenses::*;
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::transition::speed::ScalarSpeed;

    use super::{speed::calculate_speed, prelude::Tweenable};

    #[test]
    pub fn test_calculate_speed() {
        let actual = calculate_speed::<f32>(&-1.0, &2.0, Duration::from_secs_f32(1.5));
        assert_eq!(actual, ScalarSpeed::new(2.0));
    }

    #[test]
    pub fn test_transition(){
        let transitioned = <f32 as Tweenable>::transition_towards(&-10.0, &10.0, &ScalarSpeed::new(20.0), &0.5);
        assert_eq!(transitioned, 0.0);
    }

    #[test]
    pub fn test_complete_transition(){
        let transitioned = <f32 as Tweenable>::transition_towards(&-1.0, &1.0, &ScalarSpeed::new(20.0), &0.5);
        assert_eq!(transitioned, 1.0);
    }
}
