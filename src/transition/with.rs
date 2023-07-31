use std::marker::PhantomData;
use std::time::Duration;

use bevy::prelude::*;

use crate::prelude::*;
use crate::transition::prelude::*;

use super::speed::calculate_speed;

pub trait CanHaveTransition: HierarchyNode + Sized {
    fn with_transition_in<L: Lens + GetValueLens>(
        self,
        initial: L::Object,
        destination: L::Value,
        duration: Duration,
    )-> WithTransition<Self, L> where
        L::Value: Tweenable,
        L::Object: Clone + PartialEq + Component,
    {
        let initial_value = L::get_value(&initial);
        let speed = calculate_speed(&initial_value, &destination, duration);

        self.with_transition(
            initial,
            TransitionStep::new(destination, speed).into(),
            None,
        )
    }

    fn with_transition_in_out<L: Lens + GetValueLens>(
        self,
        initial: L::Object,
        destination: L::Value,
        out_destination: L::Value,
        in_duration: Duration,
        out_duration: Duration,
    )-> WithTransition<Self, L> where
        L::Value: Tweenable,
        L::Object: Clone + PartialEq + Component,
    {
        let initial_value = L::get_value(&initial);
        let in_speed = calculate_speed(&initial_value, &destination, in_duration);

        let out_speed = calculate_speed(&destination, &out_destination, out_duration);

        self.with_transition(
            initial,
            TransitionStep::new(destination, in_speed).into(),
            Some(TransitionStep::new(out_destination, out_speed).into()),
        )
    }

    // fn with_entry_transition<L: Lens>(
    //     self,
    //     initial: L::Object,
    //     path:  impl Into<TransitionPath<L>>,
    // ) -> WithTransition<Self, L>
    // where
    //     L::Value: Tweenable,
    //     L::Object: Clone + PartialEq + Component{
    //         self.with_transition(initial, path.into(), None)
    //     }

    // fn with_both_transitions<L: Lens>(
    //     self,
    //     initial: L::Object,
    //     path: impl Into<TransitionPath<L>>,
    //     deletion_path:  impl Into<TransitionPath<L>>,
    // ) -> WithTransition<Self, L>
    // where
    //     L::Value: Tweenable,
    //     L::Object: Clone + PartialEq + Component{
    //         self.with_transition(initial, path.into(), Some(deletion_path.into()))
    //     }

    fn with_transition<L: Lens>(
        self,
        initial: L::Object,
        path: TransitionPath<L>,
        deletion_path: Option<TransitionPath<L>>,
    ) -> WithTransition<Self, L>
    where
        L::Value: Tweenable,
        L::Object: Clone + PartialEq + Component,
    {
        WithTransition {
            node: self,
            initial,
            path,
            deletion_path,
        }
    }
}

impl<N: HierarchyNode> CanHaveTransition for N {}

/// This required the animation plugin

pub struct WithTransition<N: HierarchyNode, L: Lens>
where
    L::Value: Tweenable,
    L::Object: Clone + PartialEq + Component,
{
    pub node: N,
    pub initial: L::Object,
    pub path: TransitionPath<L>,
    pub deletion_path: Option<TransitionPath<L>>,
}

impl<
        N: HierarchyNode,
        L: Lens<Value = V, Object = C>,
        V: Tweenable,
        C: Clone + PartialEq + Component,
    > PartialEq for WithTransition<N, L>
{
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
            && self.initial == other.initial
            && self.path == other.path
            && self.deletion_path == other.deletion_path
    }
}

impl<
        N: HierarchyNode,
        C: Component + Clone + PartialEq,
        V: Tweenable,
        L: Lens<Object = C, Value = V> + GetValueLens,
    > HierarchyNode for WithTransition<N, L>
{
    type Context<'c> = N::Context<'c>;

    fn get_components<'c>(
        &self,
        context: &Self::Context<'c>,
        component_commands: &mut impl ComponentCommands,
    ) {
        self.node.get_components(context, component_commands);

        if let Some(previous) = component_commands.get::<C>() {
            component_commands.insert(previous.clone()); //prevent this being overwritten by node::get_components
        } else {
            component_commands.insert(self.initial.clone());
        }

        let new_path_index: Option<usize> = if let Some(suspended_path) =
            component_commands.get::<SuspendedPathComponent<L>>()
        {
            let i = suspended_path
                .index
                .min(self.path.steps.len().saturating_sub(1));

            //info!("Restoring suspended path index {i}");
            component_commands.remove::<SuspendedPathComponent<L>>();
            Some(i)
        } else if let Some(previous_path) = component_commands.get::<TransitionPathComponent<L>>() {
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

    fn get_children<'c>(
        &self,
        context: &Self::Context<'c>,
        child_commands: &mut impl ChildCommands,
    ) {
        self.node.get_children(context, child_commands)
    }

    fn on_deleted(&self, component_commands: &mut impl ComponentCommands) -> DeletionPolicy {
        let base = self.node.on_deleted(component_commands);

        let Some(deletion_path) = &self.deletion_path else{return  base;};

        let transform = component_commands.get::<C>().unwrap_or(&self.initial);
        let duration = deletion_path
            .remaining_duration(&L::get_value(transform))
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
