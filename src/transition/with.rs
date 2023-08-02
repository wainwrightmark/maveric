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

    // // fn with_entry_transition<L: Lens>(
    // //     self,
    // //     initial: L::Object,
    // //     path:  impl Into<TransitionPath<L>>,
    // // ) -> WithTransition<Self, L>
    // // where
    // //     L::Value: Tweenable,
    // //     L::Object: Clone + PartialEq + Component{
    // //         self.with_transition(initial, path.into(), None)
    // //     }

    // // fn with_both_transitions<L: Lens>(
    // //     self,
    // //     initial: L::Object,
    // //     path: impl Into<TransitionPath<L>>,
    // //     deletion_path:  impl Into<TransitionPath<L>>,
    // // ) -> WithTransition<Self, L>
    // // where
    // //     L::Value: Tweenable,
    // //     L::Object: Clone + PartialEq + Component{
    // //         self.with_transition(initial, path.into(), Some(deletion_path.into()))
    // //     }

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
    //pub initial: L::Object,
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
            //&& self.initial == other.initial
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

    fn update<'c>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'c>,
        component_commands: &mut impl UpdateCommands,
    ) {
        self.node.update(context, component_commands);

        // if let Some(previous) = component_commands.get::<L::Object>() {
        //     component_commands.insert(previous.clone()); //prevent this being overwritten by node::get_components
        // } else {
        //     component_commands.insert(self.initial.clone());
        // }

        let new_path_index: Option<usize> =
            if let Some(previous_path) = component_commands.get::<TransitionPathComponent<L>>() {
                if previous_path.path != self.path {
                    //info!("New path found");
                    Some(0)
                } else {
                    //info!("Same path found");
                    None
                }
            } else {
                //info!("No preexisting path found");
                Some(0)
            };

        if let Some(index) = new_path_index {
            component_commands.insert(TransitionPathComponent {
                path: self.path.clone(),
                index,
            });
        }
    }

    fn on_undeleted<'c>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'c>,
        commands: &mut impl ComponentCommands,
    ) {
        self.node.on_undeleted(context, commands);

        let new_path_index: Option<usize> =
            if let Some(suspended_path) = commands.get::<SuspendedPathComponent<L>>() {
                let i = suspended_path
                    .index
                    .min(self.path.steps.len().saturating_sub(1));

                //info!("Restoring suspended path index {i}");
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
