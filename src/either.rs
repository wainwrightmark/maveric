use crate::prelude::*;

macro_rules! impl_either {
    ($Either:ident, $T0:ident, $Case0:ident, $t0:ident, $(($T:ident, $C:ident, $t:ident)),*) =>
    {

        #[derive(Debug, Clone, PartialEq)]
        pub enum $Either<$T0: HierarchyNode, $($T: HierarchyNode<Context = $T0::Context>,)*> {
            $Case0($T0),
            $($C($T),)*
        }

        impl<$T0: HierarchyNode, $($T: HierarchyNode<Context = T0::Context>,)*> HierarchyNode for $Either<$T0, $($T,)*> {
            type Context = $T0::Context;

            fn set<'r>(
                &self,
                previous: Option<&Self>,
                context: &<Self::Context as NodeContext>::Wrapper<'r>,
                commands: &mut impl crate::prelude::NodeCommands,
                event: crate::prelude::SetComponentsEvent,
            ) {
                use $Either::*;
                match (self, previous) {
                    ($Case0(node), Some($Case0(prev))) => node.set(Some(prev), context, commands, event),
                    ($Case0(node),_)=> node.set(None, context, commands, event),
                    $(($C(node), Some($C(prev))) => node.set(Some(prev),context, commands, event),)*
                    $(($C(node), _) => node.set(None, context, commands, event),)*

                }
            }

            fn on_deleted<'r>(
                &self,
                commands: &mut impl crate::prelude::ComponentCommands,
            ) -> crate::prelude::DeletionPolicy {
                use $Either::*;
                match self {
                    $Case0(node) => node.on_deleted(commands),
                    $($C(node) => node.on_deleted(commands),)*

                }
            }

            //TODO children_type
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
