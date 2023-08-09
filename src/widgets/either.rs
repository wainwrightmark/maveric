use crate::widgets::prelude::{
    ChildrenAspect, ComponentsAspect,  HasContext, NodeContext,
};

//TODO impl tuples

#[derive(Debug, Clone, PartialEq)]
/// A node that could either be
pub enum Either<T0: HasContext, T1: HasContext<Context = T0::Context>> {
    Case0(T0),
    Case1(T1),
}

impl<T0: HasContext, T1: HasContext<Context = T0::Context>> HasContext for Either<T0, T1> {
    type Context = T0::Context;
}

impl<T0: HasContext, T1: HasContext<Context = T0::Context>> ComponentsAspect for Either<T0, T1>
where
    T0: ComponentsAspect,
    T1: ComponentsAspect,
{
    fn set_components<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl crate::widgets::prelude::ComponentCommands,
        event: crate::widgets::prelude::SetComponentsEvent,
    ) {
        match self {
            Either::Case0(t0) => t0.set_components(context, commands, event),
            Either::Case1(t1) => t1.set_components(context, commands, event),
        }
    }

    fn on_deleted<'r>(
        &self,
        commands: &mut impl crate::widgets::prelude::ComponentCommands,
    ) -> crate::widgets::prelude::DeletionPolicy {
        match self {
            Either::Case0(t0) => t0.on_deleted(commands),
            Either::Case1(t1) => t1.on_deleted(commands),
        }
    }
}

impl<T0: HasContext, T1: HasContext<Context = T0::Context>> ChildrenAspect for Either<T0, T1>
where
    T0: ChildrenAspect,
    T1: ChildrenAspect,
{
    fn set_children<'r>(
        &self,
        context: &<Self::Context as NodeContext>::Wrapper<'r>,
        commands: &mut impl crate::widgets::prelude::ChildCommands,
    ) {
        match self {
            Either::Case0(t0) => t0.set_children(context, commands),
            Either::Case1(t1) => t1.set_children(context, commands),
        }
    }
}
