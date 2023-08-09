use std::marker::PhantomData;

use crate::prelude::*;


pub trait HasChild<NChild: HierarchyNode>: ChildrenAspect {
    fn convert_context<'a, 'r>(
        context: &'a <Self::Context as NodeContext>::Wrapper<'r>,
    ) -> &'a <<NChild as NodeBase>::Context as NodeContext>::Wrapper<'r>;

    const DELETER: &'static dyn ChildDeleter<Self> = &NodeDeleter::<Self, NChild>::new();
}

pub trait ChildDeleter<NParent: ChildrenAspect>: Send + Sync + 'static {
    fn on_deleted<'r>(
        &self,
        entity_ref: EntityRef,
        commands: &mut ConcreteComponentCommands,
        parent_context: &<NParent::Context as NodeContext>::Wrapper<'r>,
    ) -> DeletionPolicy;
}

#[macro_export]
macro_rules! impl_has_child {
    ($NParent:ty, $NChild: ty, $context: ident, $from_context:expr  ) => {
        impl HasChild<$NChild> for $NParent {
            fn convert_context<'a, 'r>(
                $context: &'a <Self::Context as NodeContext>::Wrapper<'r>,
            ) -> &'a <<$NChild as NodeBase>::Context as NodeContext>::Wrapper<'r> {
                $from_context
            }
        }
    };
}

// impl<NParent: ChildrenAspect, NChild: HierarchyNode + NodeBase<Context = NoContext>>
//     HasChild<NChild> for NParent
// {
//     fn convert_context<'a, 'r>(
//         context: &'a <Self::Context as NodeContext>::Wrapper<'r>,
//     ) -> &'a <<NChild as NodeBase>::Context as NodeContext>::Wrapper<'r> {
//         &()
//     }
// }

// impl<Context: NodeContext, NParent: ChildrenAspect + NodeBase<Context = Context>, NChild: HierarchyNode + NodeBase<Context = Context>>
//     HasChild<NChild> for NParent
// {
//     fn convert_context<'a, 'r>(
//         context: &'a <Self::Context as NodeContext>::Wrapper<'r>,
//     ) -> &'a <<NChild as NodeBase>::Context as NodeContext>::Wrapper<'r> {
//         context
//     }
// }


#[derive(Debug)]
struct NodeDeleter<NParent: ChildrenAspect + HasChild<NChild>, NChild: HierarchyNode> {
    phantom: PhantomData<(NParent, NChild)>,
}

impl<NParent: ChildrenAspect + HasChild<NChild>, NChild: HierarchyNode>
    NodeDeleter<NParent, NChild>
{
    const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<NParent: ChildrenAspect + HasChild<NChild>, NChild: HierarchyNode> ChildDeleter<NParent>
    for NodeDeleter<NParent, NChild>
{
    fn on_deleted<'r>(
        &self,
        entity_ref: EntityRef,
        commands: &mut ConcreteComponentCommands,
        parent_context: &<<NParent>::Context as NodeContext>::Wrapper<'r>,
    ) -> DeletionPolicy {
        if let Some(hierarchy_node_component) = entity_ref.get::<HierarchyNodeComponent<NChild>>() {
            let child_context = NParent::convert_context(parent_context);
            let component_context = NChild::components_context(child_context);

            let previous_args = NChild::as_component_aspect(&hierarchy_node_component.node);
            <NChild::ComponentsAspect>::on_deleted(previous_args, &component_context, commands)
        } else {
            warn!(
                "Deleted entity of type {t} did not have HierarchyNodeComponent",
                t = std::any::type_name::<NChild>()
            );
            DeletionPolicy::DeleteImmediately
        }
    }
}