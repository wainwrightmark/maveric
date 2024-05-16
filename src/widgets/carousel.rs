use crate::prelude::*;
use crate::transition::ui_lenses::StyleLeftLens;
use std::{marker::PhantomData, time::Duration};

/// You must register the `StyleLeftLens` transition for this to work
pub struct Carousel<Child: MavericNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> {
    current_page: u32,
    get_child: F,
    transition_duration: Duration,
    ease: Ease,
    phantom: PhantomData<Child>,
}

impl<Child: MavericNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> Carousel<Child, F> {
    pub const fn new(
        current_page: u32,
        get_child: F,
        transition_duration: Duration,
        ease: Ease,
    ) -> Self {
        Self {
            current_page,
            get_child,
            transition_duration,
            ease,
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
            && self.ease == other.ease
    }
}

impl<Child: MavericNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> MavericNode
    for Carousel<Child, F>
{
    type Context<'w, 's> = <Child as MavericNode>::Context<'w, 's>;

    fn set_components(commands: SetComponentCommands<Self, Self::Context<'_, '_>>) {
        commands.ignore_node().ignore_context().insert(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        });
    }

    fn set_children<R: MavericRoot>(commands: SetChildrenCommands<Self, Self::Context<'_, '_>, R>) {
        commands.ordered(|args, commands| {
            const CENTER: f32 = 50.0;
            const PAGE_WIDTH: f32 = 200.0;
            const LEFT: f32 = CENTER - PAGE_WIDTH;
            const RIGHT: f32 = CENTER + PAGE_WIDTH;

            let NodeArgs {
                context,
                event: _event,
                node,
                previous,
            } = args;

            let Some(center_page) = (node.get_child)(node.current_page) else {
                return;
            };
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

                    let Some(previous_page) = (node.get_child)(*previous_page_number) else {
                        break 'previous;
                    };

                    let previous_page = previous_page.with_transition_in::<StyleLeftLens>(
                        Val::Percent(CENTER),
                        Val::Percent(previous_position),
                        node.transition_duration,
                        Some(Ease::CubicIn),
                    );

                    commands.add_child(*previous_page_number, previous_page, context);
                }
            }

            let center_page = center_page.with_transition_in::<StyleLeftLens>(
                Val::Percent(center_page_initial),
                Val::Percent(CENTER),
                node.transition_duration,
                Some(node.ease),
            );

            commands.add_child(node.current_page, center_page, context);
        });
    }
}
