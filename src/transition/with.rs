use std::sync::Arc;
use std::time::Duration;

use bevy::prelude::*;

use crate::prelude::*;
use crate::transition::prelude::*;

use super::speed::calculate_speed;

pub trait CanHaveTransition: HierarchyNode + Sized {
    fn with_transition_in<L: Lens + GetValueLens>(
        self,
        initial_value: L::Value,
        destination: L::Value,
        duration: Duration,
    ) -> WithTransition<Self, L, ()>
    where
        L::Value: Tweenable,
        L::Object: Clone + Component,
    {
        let speed = calculate_speed(&initial_value, &destination, duration);
        let update_transition = TransitionStep::new_arc(destination, Some(speed), None);

        self.with_transition(initial_value, update_transition, ())
    }

    fn with_transition_in_out<L: Lens + GetValueLens>(
        self,
        initial_value: L::Value,
        destination: L::Value,
        out_destination: L::Value,
        in_duration: Duration,
        out_duration: Duration,
    ) -> WithTransition<Self, L, DurationDeletionPathMaker<L>>
    where
        L::Value: Tweenable,
        L::Object: Clone + Component,
    {
        let speed = calculate_speed(&initial_value, &destination, in_duration);
        let update_transition = TransitionStep::new_arc(destination, Some(speed), None);

        self.with_transition(
            initial_value,
            update_transition,
            DurationDeletionPathMaker::new(out_duration, out_destination),
        )
    }

    fn with_transition_to<L: Lens + GetValueLens>(
        self,
        destination: L::Value,
        speed: <L::Value as Tweenable>::Speed,
    ) -> WithTransition<Self, L, ()>
    where
        L::Value: Tweenable,
        L::Object: Clone + Component,
    {
        let update_transition = TransitionStep::new_arc(destination.clone(), Some(speed), None);

        self.with_transition(destination, update_transition, ())
    }

    fn with_transition<L: Lens + GetValueLens, P: DeletionPathMaker<L>>(
        self,
        initial_value: L::Value,
        update_transition: Arc<TransitionStep<L>>,
        deletion: P,
    ) -> WithTransition<Self, L, P>
    where
        L::Value: Tweenable,
        L::Object: Clone + Component,
    {
        WithTransition {
            node: self,
            initial_value,
            update_transition,
            deletion,
        }
    }
}

impl<N: HierarchyNode> CanHaveTransition for N {}

/// This requires the animation plugin

pub struct WithTransition<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    pub node: N,

    /// The initial value
    pub initial_value: L::Value,
    /// The transition that will be run when the node is updated or undeleted
    pub update_transition: Arc<TransitionStep<L>>,
    pub deletion: P,
}

impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> HasChildrenAspect
    for WithTransition<N, L, P>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    type ChildrenAspect = N::ChildrenAspect;

    fn children_context<'a, 'r>(
        context: &'a <<Self as HasContext>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ChildrenAspect as HasContext>::Context as NodeContext>::Wrapper<'r> {
        N::children_context(context)
    }

    fn as_children_aspect<'a>(&'a self) -> &'a Self::ChildrenAspect {
        &self.node.as_children_aspect()
    }
}

impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> HasContext
    for WithTransition<N, L, P>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    type Context = N::Context;
}

impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> ComponentsAspect
    for WithTransition<N, L, P>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    fn set_components<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        event: SetComponentsEvent,
    ) {
        self.node.as_component_aspect().set_components(
            N::components_context(context),
            commands,
            event,
        );

        match event {
            SetComponentsEvent::Created => {
                let in_transition = TransitionStep::new_arc(
                    self.initial_value.clone(),
                    None,
                    Some(self.update_transition.clone()),
                );

                commands.insert(Transition {
                    step: in_transition,
                });
            }
            SetComponentsEvent::Updated => {
                if let Some(previous_path) = commands.get::<Transition<L>>() {
                    if self.update_transition.contains(&previous_path.step) {
                        //info!("Same path found - no change");
                    } else {
                        //info!("New path found");
                        commands.insert(Transition {
                            step: self.update_transition.clone(),
                        });
                    }
                } else {
                    //info!("No path found");
                    commands.insert(Transition {
                        step: self.update_transition.clone(),
                    });
                }
            }
            SetComponentsEvent::Undeleted => {
                let step = if let Some(existing_value) = commands.get::<L::Object>() {
                    if let Some(destination) = L::try_get_value(existing_value) {
                        TransitionStep::<L>::new_arc(
                            destination,
                            None,
                            Some(self.update_transition.clone()),
                        )
                    } else {
                        self.update_transition.clone()
                    }
                } else {
                    self.update_transition.clone()
                };

                commands.insert(Transition { step });
            }
        }
    }

    fn on_deleted<'r>(&self, commands: &mut impl ComponentCommands) -> DeletionPolicy {
        let base = self.node.as_component_aspect().on_deleted(commands);

        let Some(component) = commands
                .get::<L::Object>() else {return base;};

        let previous = &<L as GetValueLens>::try_get_value(component);

        let Some(previous) = previous else {return  base;};

        let Some(deletion_path) = self.deletion.get_step(previous) else{return  base;};

        let duration = deletion_path
            .remaining_duration(previous)
            .unwrap_or_default();

        let duration = match base {
            DeletionPolicy::DeleteImmediately => duration,
            DeletionPolicy::Linger(d) => duration.max(d),
        };

        commands.insert(Transition {
            step: deletion_path,
        });

        DeletionPolicy::Linger(duration)
    }
}

impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> PartialEq
    for WithTransition<N, L, P>
where
    L::Value: Tweenable,
    L::Object: Clone + Component,
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
            && self.initial_value.approx_eq(&other.initial_value)
            && self.update_transition == self.update_transition
            && self.deletion == other.deletion
    }
}
