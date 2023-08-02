use std::{marker::PhantomData, time::Duration};

use crate::prelude::*;

pub struct Carousel<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> {
    current_page: u32,
    get_child: F,
    transition_duration: Duration,
    phantom: PhantomData<Child>,
}

impl<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> Carousel<Child, F> {
    pub fn new(current_page: u32, get_child: F, transition_duration: Duration) -> Self {
        Self {
            current_page,
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

impl<Child: HierarchyNode, F: Send + Sync + 'static + Fn(u32) -> Option<Child>> HierarchyNode
    for Carousel<Child, F>
{
    type Context = <Child as HierarchyNode>::Context;

    fn update<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl UpdateCommands,
    ) {
        commands.insert(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        });

        let child = (self.get_child)(self.current_page);

        if let Some(child) = child {
            //child.update(context, commands);

            let child = child.with_transition_in_out::<StyleLeftLens>(
                Val::Percent(00.0),
                Val::Percent(50.0),
                Val::Percent(100.0),
                self.transition_duration,
                self.transition_duration,
            );

            commands.add_child(self.current_page, context, child);
        }
    }
}
