use crate::prelude::*;
use std::{marker::PhantomData, time::Duration};

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
        #[derive(Debug, PartialEq)]
        enum Position {
            Prev,
            Current,
            Next,
        }

        const FAR_LEFT: Val = Val::Percent(0.0);
        const CENTER: Val = Val::Percent(50.0);
        const FAR_RIGHT: Val = Val::Percent(100.0);

        let left_speed =
            crate::transition::speed::calculate_speed(&FAR_LEFT, &CENTER, self.transition_duration);

        const ZERO_VEC: Vec3 = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        };

        let scale_speed = crate::transition::speed::calculate_speed(
            &ZERO_VEC,
            &Vec3::ONE,
            self.transition_duration,
        );

        let children = [
            (Position::Prev, FAR_LEFT),
            (Position::Current, CENTER),
            (Position::Next, FAR_RIGHT),
        ];

        for (position, left) in children {
            let index = match position {
                Position::Prev => self.current_page.checked_sub(1),
                Position::Current => Some(self.current_page),
                Position::Next => self.current_page.checked_add(1),
            };

            let Some(index) = index else{ continue;};

            let Some(child) = (self.get_child)(index) else {continue;};

            let current_scale = if position == Position::Current {
                Vec3::ONE
            } else {
                ZERO_VEC
            };

            let child = child
                .with_transition_to::<StyleLeftLens>(
                    left,
                    left_speed,
                )
                .with_transition_to::<TransformScaleLens>(
                    current_scale,scale_speed
                );

            commands.add_child(index, child, context);
        }
    }
}
