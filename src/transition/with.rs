use std::marker::PhantomData;

use crate::prelude::*;
use crate::transition::prelude::*;

/// This required the animation plugin
#[derive(PartialEq)]
pub struct WithTransition<N: HierarchyNode, V: ComponentVelocity> {
    pub node: N,
    pub initial: V::C,
    pub path: TransitionPath<V>,
    pub deletion_path: Option<TransitionPath<V>>,
}

impl<N: HierarchyNode, V: ComponentVelocity> HierarchyNode for WithTransition<N, V> {
    type Context<'c> = N::Context<'c>;

    fn get_components<'c>(
        &self,
        context: &Self::Context<'c>,
        component_commands: &mut impl ComponentCommands,
    ) {
        self.node.get_components(context, component_commands);

        if let Some(previous) = component_commands.get::<V::C>() {
            component_commands.insert(previous.clone()); //prevent this being overwritten by component_commands
        } else {
            component_commands.insert(self.initial.clone());
        }

        let new_path_index: Option<usize> = if let Some(suspended_path) =
            component_commands.get::<SuspendedPathComponent<V>>()
        {
            let i = suspended_path
                .index
                .min(self.path.steps.len().saturating_sub(1));

            //info!("Restoring suspended path index {i}");
            component_commands.remove::<SuspendedPathComponent<V>>();
            Some(i)
        } else if let Some(previous_path) = component_commands.get::<TransitionPathComponent<V>>() {
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

        let transform = component_commands.get::<V::C>().unwrap_or(&self.initial);
        let duration = deletion_path.remaining_duration(transform);

        let duration = match base {
            DeletionPolicy::DeleteImmediately => duration,
            DeletionPolicy::Linger(d) => duration.max(d),
        };
        let current_path = component_commands.get::<TransitionPathComponent<V>>();

        if let Some(current_path) = current_path {
            component_commands.insert(SuspendedPathComponent::<V> {
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
