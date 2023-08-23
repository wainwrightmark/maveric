use crate::prelude::*;
use std::{marker::PhantomData, time::Duration};

pub struct Carousel<Child: MavericNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> {
    current_page: u32,
    get_child: F,
    transition_duration: Duration,
    phantom: PhantomData<Child>,
}

impl<Child: MavericNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> Carousel<Child, F> {
    pub fn new(current_page: u32, get_child: F, transition_duration: Duration) -> Self {
        Self {
            current_page,
            get_child,
            transition_duration,
            phantom: PhantomData,
        }
    }
}

impl<Child: MavericNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> PartialEq
    for Carousel<Child, F>
{
    fn eq(&self, other: &Self) -> bool {
        self.current_page == other.current_page
            && self.phantom == other.phantom
            && self.transition_duration == other.transition_duration
    }
}

impl<Child: MavericNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> MavericNode
    for Carousel<Child, F>
{
    type Context = <Child as MavericNode>::Context;

    fn set<R: MavericRoot>(
        data: NodeData<Self, Self::Context, R, true>,
        commands: &mut NodeCommands,
    ) {
        data.clone().ignore_args().ignore_context().insert(
            commands,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        );

        data.ordered_children_with_args_and_context_advanced(commands,|node,previous,context, _,commands|{
        const CENTER: f32 = 50.0;
        const PAGE_WIDTH: f32 = 200.0;
        const LEFT: f32 = CENTER - PAGE_WIDTH;
        const RIGHT: f32 = CENTER + PAGE_WIDTH;

        let Some(center_page) = (node.get_child)(node.current_page) else {return;};
        let mut center_page_initial = CENTER;

        'previous: {
            if let Some(Self {
                current_page: previous_page_number,
                ..
            }) = previous
            {
                let (current_position, previous_position) =
                    match previous_page_number.cmp(&node.current_page) {
                        std::cmp::Ordering::Less => (RIGHT, LEFT),
                        std::cmp::Ordering::Equal => {
                            break 'previous;
                        }
                        std::cmp::Ordering::Greater => (LEFT, RIGHT),
                    };

                center_page_initial = current_position;

                let Some(previous_page) = (node.get_child)(*previous_page_number) else {break 'previous;};

                let previous_page = previous_page.with_transition_in::<StyleLeftLens>(
                    Val::Percent(CENTER),
                    Val::Percent(previous_position),
                    node.transition_duration,
                );

                commands.add_child(*previous_page_number, previous_page, context);
            }
        }

        let center_page = center_page.with_transition_in::<StyleLeftLens>(
            Val::Percent(center_page_initial),
            Val::Percent(CENTER),
            node.transition_duration,
        );

        commands.add_child(node.current_page, center_page, context);
        });
    }
}
