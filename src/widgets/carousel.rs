use crate::prelude::*;
use std::{marker::PhantomData, time::Duration};

pub struct Carousel<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> {
    current_page: u32,
    total_pages: u32,
    get_child: F,
    transition_duration: Duration,
    phantom: PhantomData<Child>,
}

impl<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> Carousel<Child, F> {
    pub fn new(
        current_page: u32,
        total_pages: u32,
        get_child: F,
        transition_duration: Duration,
    ) -> Self {
        Self {
            current_page,
            total_pages,
            get_child,
            transition_duration,
            phantom: PhantomData,
        }
    }
}

impl<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> PartialEq
    for Carousel<Child, F>
{
    fn eq(&self, other: &Self) -> bool {
        self.current_page == other.current_page
            && self.phantom == other.phantom
            && self.transition_duration == other.transition_duration
    }
}

impl<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> HasContext
    for Carousel<Child, F>
{
    type Context = <Child as HasContext>::Context;
}

impl<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> ComponentsAspect
    for Carousel<Child, F>
{
    fn set_components<'r>(
        &self,
        _context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ComponentCommands,
        event: SetComponentsEvent,
    ) {
        if event == SetComponentsEvent::Created {
            commands.insert(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

impl<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> ChildrenAspect
    for Carousel<Child, F>
{
    fn set_children<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    ) {
        const CENTER: f32 = 50.0;

        const PAGE_WIDTH: f32 = 200.0;

        let left_speed = crate::transition::speed::calculate_speed(
            &0.0f32,
            &PAGE_WIDTH,
            self.transition_duration,
        );

        for index in 0..self.total_pages {
            let Some(child) = (self.get_child)(index) else {continue;};

            let percentage = CENTER + (((index as f32) - (self.current_page as f32)) * PAGE_WIDTH);


            let child = child.with_transition_to::<StyleLeftLens>(Val::Percent(percentage), left_speed);

            commands.add_child(index, child, context);
        }
    }
}
