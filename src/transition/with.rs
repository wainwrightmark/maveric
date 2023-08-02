// use std::marker::PhantomData;
// use std::time::Duration;

// use bevy::prelude::*;
// use bevy::utils::HashSet;

// use crate::prelude::*;
// use crate::transition::prelude::*;

// use super::speed::{calculate_speed, Speed};

// pub trait DeletionPathMaker<L: Lens + GetValueLens>: PartialEq + Send + Sync + 'static
// where
//     L::Value: Tweenable,
//     L::Object: Clone + PartialEq + Component,
// {
//     fn get_step(
//         &self,
//         previous: &L::Value,
//         sibling_keys: &HashSet<ChildKey>,
//     ) -> Option<TransitionStep<L>>;
// }

// #[derive(Debug, Clone)]
// pub struct DurationDeletionPathMaker<L: Lens + GetValueLens>
// where
//     L::Value: Tweenable,
//     L::Object: Clone + PartialEq + Component,
// {
//     duration: Duration,
//     destination: L::Value,
// }

// impl<L: Lens + GetValueLens> PartialEq for DurationDeletionPathMaker<L>
// where
//     L::Value: Tweenable,
//     L::Object: Clone + PartialEq + Component,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.duration == other.duration && self.destination.approx_eq(&other.destination)
//     }
// }

// impl<L: Lens + GetValueLens> DeletionPathMaker<L> for DurationDeletionPathMaker<L>
// where
//     L::Value: Tweenable,
//     L::Object: Clone + PartialEq + Component,
// {
//     fn get_step(
//         &self,
//         previous: &<L as Lens>::Value,
//         _sibling_keys: &HashSet<ChildKey>,
//     ) -> Option<TransitionStep<L>> {
//         let out_speed = calculate_speed(previous, &self.destination, self.duration);

//         Some(TransitionStep::new(self.destination.clone(), out_speed, None).into())
//     }
// }

// impl<L: Lens + GetValueLens> DurationDeletionPathMaker<L>
// where
//     L::Value: Tweenable,
//     L::Object: Clone + PartialEq + Component,
// {
//     fn new(duration: Duration, destination: L::Value) -> Self {
//         Self {
//             duration,
//             destination,
//         }
//     }
// }

// impl<L: Lens + GetValueLens> DeletionPathMaker<L> for ()
// where
//     L::Value: Tweenable,
//     L::Object: Clone + PartialEq + Component,
// {
//     fn get_step(
//         &self,
//         previous: &<L as Lens>::Value,
//         sibling_keys: &HashSet<ChildKey>,
//     ) -> Option<TransitionStep<L>> {
//         None
//     }
// }

// impl<L: Lens + GetValueLens> DeletionPathMaker<L> for TransitionStep<L>
// where
//     L::Value: Tweenable,
//     L::Object: Clone + PartialEq + Component,
// {
//     fn get_step(
//         &self,
//         previous: &<L as Lens>::Value,
//         sibling_keys: &HashSet<ChildKey>,
//     ) -> Option<TransitionStep<L>> {
//         Some(self.clone())
//     }
// }

// pub trait CanHaveTransition: HierarchyNode + Sized {
//     fn with_transition_in<L: Lens + GetValueLens>(
//         self,
//         initial: L::Value,
//         destination: L::Value,
//         duration: Duration,
//     ) -> WithTransition<Self, L, ()>
//     where
//         L::Value: Tweenable,
//         L::Object: Clone + PartialEq + Component,
//     {
//         let in_speed = calculate_speed(&initial, &destination, duration);
//         let real_step = TransitionStep::new(destination, in_speed, None);

//         let first_step = TransitionStep::<L>::new(
//             initial,
//             <<L::Value as Tweenable>::Speed as Speed>::INFINITE,
//             Some(Box::new(real_step)),
//         );

//         self.with_transition(first_step, ())
//     }

//     fn with_transition_in_out<L: Lens + GetValueLens>(
//         self,
//         initial: L::Value,
//         destination: L::Value,
//         out_destination: L::Value,
//         in_duration: Duration,
//         out_duration: Duration,
//     ) -> WithTransition<Self, L, DurationDeletionPathMaker<L>>
//     where
//         L::Value: Tweenable,
//         L::Object: Clone + PartialEq + Component,
//     {
//         let in_speed = calculate_speed(&initial, &destination, in_duration);
//         let real_step = TransitionStep::new(destination, in_speed, None);

//         let first_step = TransitionStep::<L>::new(
//             initial,
//             <<L::Value as Tweenable>::Speed as Speed>::INFINITE,
//             Some(Box::new(real_step)),
//         );

//         self.with_transition(
//             first_step,
//             DurationDeletionPathMaker::new(out_duration, out_destination),
//         )
//     }

//     fn with_transition<L: Lens + GetValueLens, P: DeletionPathMaker<L>>(
//         self,
//         step: TransitionStep<L>,
//         deletion: P,
//     ) -> WithTransition<Self, L, P>
//     where
//         L::Value: Tweenable,
//         L::Object: Clone + PartialEq + Component,
//     {
//         WithTransition {
//             node: self,
//             step,
//             deletion,
//         }
//     }
// }

// impl<N: HierarchyNode> CanHaveTransition for N {}

// /// This required the animation plugin

// pub struct WithTransition<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>>
// where
//     L::Value: Tweenable,
//     L::Object: Clone + PartialEq + Component,
// {
//     pub node: N,
//     pub step: TransitionStep<L>,
//     pub deletion: P,
// }

// impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> PartialEq
//     for WithTransition<N, L, P>
// where
//     L::Value: Tweenable,
//     L::Object: Clone + PartialEq + Component,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.node == other.node && self.step == other.step && self.deletion == other.deletion
//     }
// }

// impl<N: HierarchyNode, L: Lens + GetValueLens, P: DeletionPathMaker<L>> HierarchyNode
//     for WithTransition<N, L, P>
// where
//     L::Value: Tweenable,
//     L::Object: Clone + PartialEq + Component,
// {
//     type Context = N::Context;

//     fn set_components<'r>(
//         &self,
//         context: &<Self::Context as NodeContext>::Wrapper<'r>,
//         commands: &mut impl ComponentCommands,
//         event: SetComponentsEvent,
//     ) {
//         self.node.set_components(context, commands, event);

//         match event {
//             SetComponentsEvent::Created => {
//                 commands.insert(TransitionPathComponent {
//                     step: self.step.clone(),
//                 });
//             }
//             SetComponentsEvent::Updated => {
//                 if let Some(previous_path) = commands.get::<TransitionPathComponent<L>>() {
//                     if previous_path.step != self.step {
//                         //info!("New path found");
//                         commands.insert(TransitionPathComponent {
//                             step: self.step.clone(),
//                         });
//                     }
//                 }
//             }
//             SetComponentsEvent::Undeleted => {
//                 commands.insert(TransitionPathComponent {
//                     step: self.step.clone(),
//                 });

//                 // let new_path_index: Option<usize> =
//                 //     if let Some(suspended_path) = commands.get::<SuspendedPathComponent<L>>() {
//                 //         let i = suspended_path
//                 //             .index
//                 //             .min(self.step.steps.len().saturating_sub(1));

//                 //         //let step = &self.path.steps[i];
//                 //         //info!("Restoring suspended path index {i} len {l} step {step:?}", l = self.path.steps.len());
//                 //         commands.remove::<SuspendedPathComponent<L>>();
//                 //         Some(i)
//                 //     } else {
//                 //         //info!("No preexisting path found");
//                 //         Some(0)
//                 //     };

//                 // if let Some(index) = new_path_index {

//                 // }
//             }
//         }
//     }

//     fn set_children<'r>(
//         &self,
//         context: &<Self::Context as NodeContext>::Wrapper<'r>,
//         commands: &mut impl ChildCommands,
//     ) {
//         self.node.set_children(context, commands);
//     }

//     fn on_deleted(
//         &self,
//         component_commands: &mut impl ComponentCommands,
//         new_sibling_keys: &HashSet<ChildKey>,
//     ) -> DeletionPolicy {
//         let base = self.node.on_deleted(component_commands, new_sibling_keys);

//         let Some(component) = component_commands
//             .get::<L::Object>() else {return base;};

//         let previous = &<L as GetValueLens>::get_value(component);

//         let Some(deletion_path) = self.deletion.get_step(previous, new_sibling_keys) else{return  base;};

//         let duration = deletion_path
//             .remaining_duration(previous)
//             .unwrap_or_default();

//         let duration = match base {
//             DeletionPolicy::DeleteImmediately => duration,
//             DeletionPolicy::Linger(d) => duration.max(d),
//         };

//         component_commands.insert(TransitionPathComponent {
//             step: deletion_path.clone(),
//         });

//         DeletionPolicy::Linger(duration)
//     }
// }
