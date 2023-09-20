use crate::transition::prelude::*;
use bevy::prelude::*;

pub trait CanRegisterTransition {
    fn register_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Component,
        L::Value: Tweenable;
}

impl CanRegisterTransition for App {
    fn register_transition<L: Lens + GetValueLens + SetValueLens>(&mut self) -> &mut Self
    where
        L::Object: Component,
        L::Value: Tweenable,
    {
        self.add_systems(PreUpdate, step_transition::<L>)
    }
}

#[allow(clippy::needless_pass_by_value)]
fn step_transition<L: Lens + GetValueLens + SetValueLens>(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transition<L>, &mut L::Object)>,
) where
    L::Object: Component,
    L::Value: Tweenable,
{
    let delta_seconds = time.delta_seconds();

    for (entity, mut tp, mut t) in query.iter_mut() {
        let component = t.as_mut();

        let speed = {
            loop {
                if let Some(speed) = tp.step.speed {
                    break speed;
                }
                <L as SetValueLens>::try_set(component, tp.step.destination.clone());
                if !tp.try_go_to_next_step() {
                    commands.entity(entity).remove::<Transition<L>>();
                    return;
                }
            }
        };

        // info!(
        //     "Transition: {lens:?} {delta_seconds:?}",
        //     lens = std::any::type_name::<L>()
        // );

        let from = L::try_get_value(component);

        let Some(from) = from else {
            return;
        };

        let new_value =
            Tweenable::transition_towards(&from, &tp.step.destination, &speed, &delta_seconds);

        //info!("Transition from {from:?} to {new_value:?}");

        if tp.step.destination.eq(&new_value) && !tp.try_go_to_next_step() {
            commands.entity(entity).remove::<Transition<L>>();
        }

        <L as SetValueLens>::try_set(component, new_value);
    }
}
