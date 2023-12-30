use std::time::Duration;

use bevy::prelude::*;

use crate::prelude::*;
use crate::transition::prelude::*;

use super::speed::calculate_speed;

pub trait CanHaveTransition: MavericNode + Sized {

    /// Transition from `initial_value` to `destination` when the node is first created
    #[must_use]
    fn with_transition_in<L: Lens + GetValueLens + SetValueLens>(
        self,
        initial_value: L::Value,
        destination: L::Value,
        duration: Duration,
    ) -> WithTransition<Self, L, ()>
    where
        L::Value: Tweenable,
        L::Object: Component,
    {
        let speed = calculate_speed(&initial_value, &destination, duration);

        self.with_transition(
            initial_value,
            Transition::TweenValue {
                destination,
                speed,
                next: None,
            },
            (),
        )
    }
    /// Transition from `initial_value` to `destination` when the node is first created
    /// Transition to `out_destination` when the node is removed
    #[must_use]
    fn with_transition_in_out<L: Lens + GetValueLens + SetValueLens>(
        self,
        initial_value: L::Value,
        destination: L::Value,
        out_destination: L::Value,
        in_duration: Duration,
        out_duration: Duration,
    ) -> WithTransition<Self, L, DurationDeletionPathMaker<L>>
    where
        L::Value: Tweenable,
        L::Object: Component,
    {
        let speed = calculate_speed(&initial_value, &destination, in_duration);

        self.with_transition(
            initial_value,
            Transition::TweenValue {
                destination,
                speed,
                next: None,
            },
            DurationDeletionPathMaker::new(out_duration, out_destination),
        )
    }
    /// Transition to `destination` whenever it changes
    #[must_use]
    fn with_transition_to<L: Lens + GetValueLens + SetValueLens>(
        self,
        destination: L::Value,
        speed: <L::Value as Tweenable>::Speed,
    ) -> WithTransition<Self, L, ()>
    where
        L::Value: Tweenable,
        L::Object: Component,
    {
        self.with_transition(
            destination.clone(),
            Transition::TweenValue {
                destination,
                speed,
                next: None,
            },
            (),
        )
    }

    #[must_use]
    fn with_transition<L: Lens + GetValueLens + SetValueLens, P: DeletionPathMaker<L>>(
        self,
        initial_value: L::Value,
        update_transition: Transition<L>,
        deletion: P,
    ) -> WithTransition<Self, L, P>
    where
        L::Value: Tweenable,
        L::Object: Component,
    {
        WithTransition {
            node: self,
            transition: (initial_value, update_transition),
            deletion,
        }
    }
}

impl<N: MavericNode> CanHaveTransition for N {}

/// This requires the animation plugin

#[derive(Debug, Clone, PartialEq)]
pub struct WithTransition<
    N: MavericNode,
    L: Lens + GetValueLens + SetValueLens,
    P: DeletionPathMaker<L>,
> where
    L::Value: Tweenable,
    L::Object: Component,
{
    pub node: N,
    pub transition: (L::Value, Transition<L>),
    pub deletion: P,
}

// impl<N: MavericNode, L: Lens + GetValueLens + SetValueLens, P: DeletionPathMaker<L>> PartialEq
//     for WithTransition<N, L, P>
// where
//     L::Value: Tweenable,
//     L::Object: Component,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.node == other.node
//             && self.transition == other.transition
//             && self.deletion == other.deletion
//     }
// }

impl<N: MavericNode, L: Lens + GetValueLens + SetValueLens, P: DeletionPathMaker<L>> MavericNode
    for WithTransition<N, L, P>
where
    L::Value: Tweenable,
    L::Object: Component,
{
    type Context = N::Context;

    fn on_created(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        world: &World,
        entity_commands: &mut bevy::ecs::system::EntityCommands,
    ) {
        N::on_created(&self.node, context, world, entity_commands);
    }

    fn on_changed(
        &self,
        previous: &Self,
        context: &<Self::Context as NodeContext>::Wrapper<'_>,
        world: &World,
        entity_commands: &mut bevy::ecs::system::EntityCommands,
    ) {
        N::on_changed(&self.node, &previous.node, context, world, entity_commands);
    }

    fn set_components(mut commands: SetComponentCommands<Self, Self::Context>) {
        commands.scope(|commands| N::set_components(commands.map_node(|x| &x.node)));

        commands
            .map_node(|x| &x.transition)
            .ignore_context()
            .advanced(|args, commands| {
                let (initial_value, update_transition) = args.node;

                //info!("With! {args:?}");

                let transition = match args.event {
                    SetEvent::Created => {
                        let in_transition = Transition::SetValue {
                            value: initial_value.clone(),
                            next: Some(Box::new(update_transition.clone())),
                        };

                        Some(in_transition)
                    }
                    SetEvent::Updated => {
                        if args.is_hot() {
                            let transition = if let Some(previous_transition) =
                                commands.get::<Transition<L>>()
                            {
                                if previous_transition.same_destination(update_transition) {
                                    //info!("Same path found - no change");
                                    None
                                } else {
                                    //info!("New path found");
                                    Some(update_transition.clone())
                                }
                            } else {
                                //info!("No path found");
                                Some(update_transition.clone())
                            };

                            match transition {
                                Some(t) => Some(
                                    if let Some(current_value) =
                                        commands.get::<L::Object>().and_then(L::try_get_value)
                                    {
                                        Transition::SetValue {
                                            value: current_value,
                                            next: Some(Box::new(t)),
                                        }
                                    } else {
                                        t
                                    },
                                ),
                                None => None,
                            }
                        } else {
                            None
                        }
                    }
                    SetEvent::Undeleted => {
                        Some(if let Some(existing_value) = commands.get::<L::Object>() {
                            if let Some(destination) = L::try_get_value(existing_value) {
                                Transition::SetValue {
                                    value: destination,
                                    next: Some(Box::new(update_transition.clone())),
                                }
                            } else {
                                update_transition.clone()
                            }
                        } else {
                            update_transition.clone()
                        })
                    }
                };

                //info!("Transition {transition:?}");
                if let Some(transition) = transition {
                    commands.insert(transition);
                }
            });
    }

    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context, R>) {
        N::set_children(commands.map_args(|x| &x.node));
    }

    fn on_deleted<'r>(&self, commands: &mut ComponentCommands) -> DeletionPolicy {
        let base = self.node.on_deleted(commands);

        let Some(component) = commands.get::<L::Object>() else {
            return base;
        };

        let previous = &<L as GetValueLens>::try_get_value(component);

        let Some(previous) = previous else {
            return base;
        };

        let Some(deletion_path) = self.deletion.get_step(previous) else {
            return base;
        };

        let duration = deletion_path
            .remaining_duration(previous)
            .unwrap_or_default();

        let duration = match base {
            DeletionPolicy::DeleteImmediately => duration,
            DeletionPolicy::Linger(d) => duration.max(d),
        };

        commands.insert(deletion_path);

        DeletionPolicy::Linger(duration)
    }
}
