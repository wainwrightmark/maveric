use crate::widgets::prelude::{ChildrenAspect, ComponentsAspect, HasContext, NodeContext};

macro_rules! impl_either {
    ($Either:ident, $T0:ident, $Case0:ident, $t0:ident, $(($T:ident, $C:ident, $t:ident)),*) =>
    {

        #[derive(Debug, Clone, PartialEq)]
        pub enum $Either<$T0: HasContext, $($T: HasContext<Context = $T0::Context>,)*> {
            $Case0($T0),
            $($C($T),)*
        }

        impl<$T0: HasContext, $($T: HasContext<Context = T0::Context>,)*> HasContext for $Either<$T0, $($T,)*> {
            type Context = $T0::Context;
        }

        impl<$T0: HasContext, $($T: HasContext<Context = T0::Context>,)*> ComponentsAspect for $Either<$T0, $($T,)*>
        where
            $T0: ComponentsAspect,
            $($T: ComponentsAspect,)*
        {
            fn set_components<'r>(
                &self,
                context: &<Self::Context as NodeContext>::Wrapper<'r>,
                commands: &mut impl crate::widgets::prelude::ComponentCommands,
                event: crate::widgets::prelude::SetComponentsEvent,
            ) {
                use $Either::*;
                match self {
                    $Case0($t0) => $t0.set_components(context, commands, event),
                    $($C($t) => $t.set_components(context, commands, event),)*

                }
            }

            fn on_deleted<'r>(
                &self,
                commands: &mut impl crate::widgets::prelude::ComponentCommands,
            ) -> crate::widgets::prelude::DeletionPolicy {
                use $Either::*;
                match self {
                    $Case0($t0) => $t0.on_deleted(commands),
                    $($C($t) => $t.on_deleted(commands),)*

                }
            }
        }

        impl<$T0: HasContext, $($T: HasContext<Context = T0::Context>,)*> ChildrenAspect for $Either<$T0, $($T,)*>
        where
            $T0: ChildrenAspect,
            $($T: ChildrenAspect,)*
        {
            fn set_children<'r>(
                &self,
                context: &<Self::Context as NodeContext>::Wrapper<'r>,
                commands: &mut impl crate::widgets::prelude::ChildCommands,
            ) {
                use $Either::*;
                match self {
                    $Case0($t0) => $t0.set_children(context, commands),
                    $($C($t) => $t.set_children(context, commands),)*

                }
            }
        }
    }
}

impl_either!(Either2, T0, Case0, t0, (T1, Case1, t1));
impl_either!(Either3, T0, Case0, t0, (T1, Case1, t1), (T2, Case2, t2));
impl_either!(
    Either4,
    T0,
    Case0,
    t0,
    (T1, Case1, t1),
    (T2, Case2, t2),
    (T3, Case3, t3)
);
