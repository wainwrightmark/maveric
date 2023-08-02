use std::marker::PhantomData;
use std::time::Duration;

use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::prelude::*;
use crate::transition::prelude::*;

use super::speed::{calculate_speed, Speed};

pub trait DeletionPathMaker<L: Lens + GetValueLens>: PartialEq + Send + Sync + 'static
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    fn get_step(
        &self,
        previous: &L::Value,
    ) -> Option<TransitionStep<L>>;
}

#[derive(Debug, Clone)]
pub struct DurationDeletionPathMaker<L: Lens + GetValueLens>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    duration: Duration,
    destination: L::Value,
}

impl<L: Lens + GetValueLens> PartialEq for DurationDeletionPathMaker<L>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration && self.destination.approx_eq(&other.destination)
    }
}

impl<L: Lens + GetValueLens> DeletionPathMaker<L> for DurationDeletionPathMaker<L>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    fn get_step(
        &self,
        previous: &<L as Lens>::Value,
    ) -> Option<TransitionStep<L>> {
        let out_speed = calculate_speed(previous, &self.destination, self.duration);

        Some(TransitionStep::new(self.destination.clone(), out_speed, None).into())
    }
}

impl<L: Lens + GetValueLens> DurationDeletionPathMaker<L>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    fn new(duration: Duration, destination: L::Value) -> Self {
        Self {
            duration,
            destination,
        }
    }
}

impl<L: Lens + GetValueLens> DeletionPathMaker<L> for ()
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    fn get_step(
        &self,
        previous: &<L as Lens>::Value,
    ) -> Option<TransitionStep<L>> {
        None
    }
}

impl<L: Lens + GetValueLens> DeletionPathMaker<L> for TransitionStep<L>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    fn get_step(
        &self,
        previous: &<L as Lens>::Value,
    ) -> Option<TransitionStep<L>> {
        Some(self.clone())
    }
}

pub trait CanHaveTransition: HierarchyNode + Sized {
    fn with_transition_in<L: Lens + GetValueLens>(
        self,
        initial: L::Value,
        destination: L::Value,
        duration: Duration,
    ) -> WithTransition<Self, L, ()>
    where
        L::Value: Tweenable,
        L::Object: Clone + PartialEq + Component,
    {
        let in_speed = calculate_speed(&initial, &destination, duration);
        let real_step = TransitionStep::new(destination, in_speed, None);

        let first_step = TransitionStep::<L>::new(
            initial,
            <<L::Value as Tweenable>::Speed as Speed>::INFINITE,
            Some(Box::new(real_step)),
        );

        self.with_transition(first_step, ())
    }

    fn with_transition_in_out<L: Lens + GetValueLens>(
        self,
        initial: L::Value,
        destination: L::Value,
        out_destination: L::Value,
        in_duration: Duration,
        out_duration: Duration,
    ) -> WithTransition<Self, L, DurationDeletionPathMaker<L>>
    where
        L::Value: Tweenable,
        L::Object: Clone + PartialEq + Component,
    {
        let in_speed = calculate_speed(&initial, &destination, in_duration);
        let real_step = TransitionStep::new(destination, in_speed, None);

        let first_step = TransitionStep::<L>::new(
            initial,
            <<L::Value as Tweenable>::Speed as Speed>::INFINITE,
            Some(Box::new(real_step)),
        );

        self.with_transition(
            first_step,
            DurationDeletionPathMaker::new(out_duration, out_destination),
        )
    }

    fn with_transition<L: Lens + GetValueLens, P: DeletionPathMaker<L>>(
        self,
        step: TransitionStep<L>,
        deletion: P,
    ) -> WithTransition<Self, L, P>
    where
        L::Value: Tweenable,
        L::Object: Clone + PartialEq + Component,
    {
        WithTransition {
            node: self,
            step,
            deletion,
        }
    }
}

impl<N: HierarchyNode> CanHaveTransition for N {}

/// This required the animation plugin

pub struct WithTransition<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    pub node: N,
    pub step: TransitionStep<L>,
    pub deletion: P,
}

impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> HierarchyNode
    for WithTransition<N, L, P>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    type ComponentsAspect = Self;

    type AncestorAspect = N::AncestorAspect;

    fn components_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::ComponentsAspect as NodeBase>::Context as NodeContext>::Wrapper<'r> {
        context
    }

    fn ancestor_context<'a, 'r>(
        context: &'a <<Self as NodeBase>::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<Self::AncestorAspect as NodeBase>::Context as NodeContext>::Wrapper<'r> {
        context
    }

    fn as_component_aspect<'a>(&'a self) -> &'a Self::ComponentsAspect {
        self
    }

    fn as_ancestor_aspect<'a>(&'a self) -> &'a Self::AncestorAspect {
        &self.node
    }
}

impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> NodeBase
    for WithTransition<N, L, P>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    type Context = N::Context;
}

impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> ComponentsAspect
    for WithTransition<N, L, P>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    fn set_components<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Ref<'r>,
        commands: &mut impl ComponentCommands,
        event: SetComponentsEvent,
    ) {
        let r = <<N::ComponentsAspect as NodeBase>::Context as NodeContext>::from_wrapper(N::components_context(context));
        self.node.as_component_aspect().set_components(, commands, event);

        match event {
            SetComponentsEvent::Created => {
                commands.insert(TransitionPathComponent {
                    step: self.step.clone(),
                });
            }
            SetComponentsEvent::Updated => {
                if let Some(previous_path) = commands.get::<TransitionPathComponent<L>>() {
                    if previous_path.step != self.step {
                        //info!("New path found");
                        commands.insert(TransitionPathComponent {
                            step: self.step.clone(),
                        });
                    }
                }
            }
            SetComponentsEvent::Undeleted => {
                commands.insert(TransitionPathComponent {
                    step: self.step.clone(),
                });

                // let new_path_index: Option<usize> =
                //     if let Some(suspended_path) = commands.get::<SuspendedPathComponent<L>>() {
                //         let i = suspended_path
                //             .index
                //             .min(self.step.steps.len().saturating_sub(1));

                //         //let step = &self.path.steps[i];
                //         //info!("Restoring suspended path index {i} len {l} step {step:?}", l = self.path.steps.len());
                //         commands.remove::<SuspendedPathComponent<L>>();
                //         Some(i)
                //     } else {
                //         //info!("No preexisting path found");
                //         Some(0)
                //     };

                // if let Some(index) = new_path_index {

                // }
            }
        }
    }

    fn on_deleted<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Ref<'r>,
        commands: &mut impl ComponentCommands,
    ) -> DeletionPolicy {
        let base = self.as_component_aspect().on_deleted(context, commands);

        let Some(component) = commands
                .get::<L::Object>() else {return base;};

        let previous = &<L as GetValueLens>::get_value(component);

        let Some(deletion_path) = self.deletion.get_step(previous, new_sibling_keys) else{return  base;};

        let duration = deletion_path
            .remaining_duration(previous)
            .unwrap_or_default();

        let duration = match base {
            DeletionPolicy::DeleteImmediately => duration,
            DeletionPolicy::Linger(d) => duration.max(d),
        };

        component_commands.insert(TransitionPathComponent {
            step: deletion_path.clone(),
        });

        DeletionPolicy::Linger(duration)
    }
}

impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> PartialEq
    for WithTransition<N, L, P>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node && self.step == other.step && self.deletion == other.deletion
    }
}
