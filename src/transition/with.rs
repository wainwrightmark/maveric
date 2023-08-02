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
    fn get_path(
        &self,
        previous: &L::Value,
        sibling_keys: &HashSet<ChildKey>,
    ) -> Option<TransitionPath<L>>;
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
    fn get_path(
        &self,
        previous: &<L as Lens>::Value,
        _sibling_keys: &HashSet<ChildKey>,
    ) -> Option<TransitionPath<L>> {
        let out_speed = calculate_speed(previous, &self.destination, self.duration);

        Some(TransitionStep::new(self.destination.clone(), out_speed).into())
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
    fn get_path(
        &self,
        _previous: &L::Value,
        _sibling_keys: &HashSet<ChildKey>,
    ) -> Option<TransitionPath<L>> {
        None
    }
}

impl<L: Lens + GetValueLens> DeletionPathMaker<L> for TransitionPath<L>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    fn get_path(
        &self,
        _previous: &L::Value,
        _sibling_keys: &HashSet<ChildKey>,
    ) -> Option<TransitionPath<L>> {
        Some(self.clone())
    }
}

impl<L: Lens + GetValueLens> DeletionPathMaker<L> for TransitionStep<L>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    fn get_path(
        &self,
        _previous: &L::Value,
        _sibling_keys: &HashSet<ChildKey>,
    ) -> Option<TransitionPath<L>> {
        Some(self.clone().into())
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
        let first_step =
            TransitionStep::<L>::new(initial, <<L::Value as Tweenable>::Speed as Speed>::INFINITE);
        let path = TransitionPath {
            steps: vec![first_step, TransitionStep::new(destination, in_speed)],
        };

        self.with_transition(path, ())
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
        let first_step =
            TransitionStep::<L>::new(initial, <<L::Value as Tweenable>::Speed as Speed>::INFINITE);
        let path = TransitionPath {
            steps: vec![first_step, TransitionStep::new(destination, in_speed)],
        };

        self.with_transition(
            path,
            DurationDeletionPathMaker::new(out_duration, out_destination),
        )
    }

    fn with_transition<L: Lens + GetValueLens, P: DeletionPathMaker<L>>(
        self,
        path: TransitionPath<L>,
        deletion: P,
    ) -> WithTransition<Self, L, P>
    where
        L::Value: Tweenable,
        L::Object: Clone + PartialEq + Component,
    {
        WithTransition {
            node: self,
            path,
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
    pub path: TransitionPath<L>,
    pub deletion: P,
}

impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> PartialEq
    for WithTransition<N, L, P>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
            && self.path == other.path
            && self.deletion == other.deletion
    }
}

impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> HierarchyNode
    for WithTransition<N, L, P>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    type Context = N::Context;

    fn set_components<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        event: SetComponentsEvent,
    ) {
        self.node.set_components(context, commands, event);

        match event {
            SetComponentsEvent::Created => {
                commands.insert(TransitionPathComponent {
                    path: self.path.clone(),
                    index: 0,
                });
            }
            SetComponentsEvent::Updated => {
                if let Some(previous_path) = commands.get::<TransitionPathComponent<L>>() {
                    if previous_path.path != self.path {
                        //info!("New path found");
                        commands.insert(TransitionPathComponent {
                            path: self.path.clone(),
                            index: 0,
                        });
                    }
                }
            }
            SetComponentsEvent::Undeleted => {
                let new_path_index: Option<usize> =
                    if let Some(suspended_path) = commands.get::<SuspendedPathComponent<L>>() {
                        let i = suspended_path
                            .index
                            .min(self.path.steps.len().saturating_sub(1));

                        //let step = &self.path.steps[i];
                        //info!("Restoring suspended path index {i} len {l} step {step:?}", l = self.path.steps.len());
                        commands.remove::<SuspendedPathComponent<L>>();
                        Some(i)
                    } else {
                        //info!("No preexisting path found");
                        Some(0)
                    };

                if let Some(index) = new_path_index {
                    commands.insert(TransitionPathComponent {
                        path: self.path.clone(),
                        index,
                    });
                }
            }
        }
    }

    fn set_children<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    ) {
        self.node.set_children(context, commands);
    }

    fn on_deleted(
        &self,
        component_commands: &mut impl ComponentCommands,
        new_sibling_keys: &HashSet<ChildKey>,
    ) -> DeletionPolicy {
        let base = self.node.on_deleted(component_commands, new_sibling_keys);

        let Some(component) = component_commands
            .get::<L::Object>() else {return base;};

        let previous = &<L as GetValueLens>::get_value(component);

        let Some(deletion_path) = self.deletion.get_path(previous, new_sibling_keys) else{return  base;};

        let duration = deletion_path
            .remaining_duration(previous)
            .unwrap_or_default();

        let duration = match base {
            DeletionPolicy::DeleteImmediately => duration,
            DeletionPolicy::Linger(d) => duration.max(d),
        };
        let current_path = component_commands.get::<TransitionPathComponent<L>>();

        if let Some(current_path) = current_path {
            component_commands.insert(SuspendedPathComponent::<L> {
                index: current_path.index,
                phantom: PhantomData,
            })
        }

        component_commands.insert(TransitionPathComponent {
            path: deletion_path.clone(),
            index: 0,
        });

        DeletionPolicy::Linger(duration)
    }
}
