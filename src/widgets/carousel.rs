// use std::{marker::PhantomData, time::Duration};

// use crate::prelude::*;

// pub struct Carousel<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> {
//     current_page: u32,
//     get_child: F,
//     transition_duration: Duration,
//     phantom: PhantomData<Child>,
// }

// #[derive(Debug, Component)]
// struct CarouselComponent {
//     page: u32,
// }

// impl<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> Carousel<Child, F> {
//     pub fn new(current_page: u32, get_child: F, transition_duration: Duration) -> Self {
//         Self {
//             current_page,
//             get_child,
//             transition_duration,
//             phantom: PhantomData,
//         }
//     }
// }

// impl<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> PartialEq
//     for Carousel<Child, F>
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.current_page == other.current_page
//             && self.phantom == other.phantom
//             && self.transition_duration == other.transition_duration
//     }
// }

// impl<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> HierarchyNode
//     for Carousel<Child, F>
// {
//     type Context = <Child as HierarchyNode>::Context;

//     fn set_components<'r>(
//         &self,
//         _context: &<Self::Context as NodeContext>::Wrapper<'r>,
//         commands: &mut impl ComponentCommands,
//         event: SetComponentsEvent,
//     ) {
//         if event == SetComponentsEvent::Created {
//             commands.insert(NodeBundle {
//                 style: Style {
//                     width: Val::Percent(100.0),
//                     height: Val::Percent(100.0),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             });
//         }

//         commands.insert(CarouselComponent {
//             page: self.current_page,
//         });
//     }

//     fn set_children<'r>(
//         &self,
//         context: &<Self::Context as NodeContext>::Wrapper<'r>,
//         commands: &mut impl ChildCommands,
//     ) {
//         let child = (self.get_child)(self.current_page);

//         if let Some(child) = child {
//             let initial_left = match commands.get::<CarouselComponent>() {
//                 Some(CarouselComponent { page }) => match page.cmp(&self.current_page) {
//                     std::cmp::Ordering::Less => Val::Percent(00.0),
//                     std::cmp::Ordering::Equal => Val::Percent(50.0),
//                     std::cmp::Ordering::Greater => Val::Percent(100.0),
//                 },
//                 _ => Val::Percent(0.0),
//             };

//             let child = child.with_transition_in_out::<StyleLeftLens>(
//                 initial_left,
//                 Val::Percent(50.0),
//                 Val::Percent(100.0),
//                 self.transition_duration,
//                 self.transition_duration,
//             );

//             commands.add_child(self.current_page, context, child);
//         }
//     }
// }
