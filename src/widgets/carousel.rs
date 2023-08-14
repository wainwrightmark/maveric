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
        previous: Option<&Self>,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl ChildCommands,
    ) {
        const CENTER: f32 = 50.0;
        const PAGE_WIDTH: f32 = 200.0;
        const LEFT: f32 = CENTER - PAGE_WIDTH;
        const RIGHT: f32 = CENTER + PAGE_WIDTH;

        let Some(current_page) = (self.get_child)(self.current_page) else {return;};

        'previous: {
            if let Some(Self {
                current_page: previous_page_number,..
            }) = previous
            {
                let (current_position, previous_position) =
                    match previous_page_number.cmp(&self.current_page) {
                        std::cmp::Ordering::Less => (RIGHT, LEFT),
                        std::cmp::Ordering::Equal => {
                            break 'previous;
                        }
                        std::cmp::Ordering::Greater => (LEFT, RIGHT),
                    };

                let Some(previous_page) = (self.get_child)(*previous_page_number) else {break 'previous;};

                let previous_page = previous_page.with_transition_in::<StyleLeftLens>(
                    Val::Percent(CENTER),
                    Val::Percent(previous_position),
                    self.transition_duration,
                );

                let current_page = current_page.with_transition_in::<StyleLeftLens>(
                    Val::Percent(current_position),
                    Val::Percent(CENTER),
                    self.transition_duration,
                );

                commands.add_child(*previous_page_number, previous_page, context);
                commands.add_child(self.current_page, current_page, context);
                return;
            }
        }

        commands.add_child(self.current_page, current_page, context);
    }
}
