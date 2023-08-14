use std::rc::Rc;

pub use crate::prelude::*;
pub use bevy::prelude::*;
use bevy::{ecs::system::EntityCommands, utils::HashMap};

pub(crate) fn create_recursive<'c, R: HierarchyRoot, N: HierarchyNode>(
    mut cec: &mut EntityCommands,
    node: N,
    context: &<<N as HasContext>::Context as NodeContext>::Wrapper<'c>,
    key: ChildKey,
) {
    //info!("Creating Node {}", type_name::<N>());

    let children_context = N::children_context(context);
    let component_context = N::components_context(context);

    let mut child_commands = CreationCommands::<R>::new(&mut cec);

    let children_args = N::as_children_aspect(&node);
    let component_args = N::as_component_aspect(&node);

    <N::ComponentsAspect as ComponentsAspect>::set_components(
        component_args,
        None,
        &component_context,
        &mut child_commands,
        SetComponentsEvent::Created,
    );
    <N::ChildrenAspect as ChildrenAspect>::set_children(
        children_args,
        None,
        &children_context,
        &mut child_commands,
    );

    let hnc = HierarchyNodeComponent::new(node);
    let hcc = HierarchyChildComponent::<R>::new::<N>(key.into());

    cec.insert((hnc, hcc));
}

pub(crate) fn delete_recursive<'c, R: HierarchyRoot>(
    commands: &mut Commands,
    entity_ref: EntityRef,
    //parent_context: &<<P as HasContext>::Context as NodeContext>::Wrapper<'c>,
) {
    if entity_ref.contains::<ScheduledForDeletion>() {
        return;
    }

    let mut ec = commands.entity(entity_ref.id());
    let mut cc = ConcreteComponentCommands::new(entity_ref, &mut ec);

    let dp: DeletionPolicy = if let Some(ac) = entity_ref.get::<HierarchyChildComponent<R>>() {
        ac.deleter.on_deleted(entity_ref, &mut cc)
    } else {
        DeletionPolicy::DeleteImmediately
    };

    match dp {
        DeletionPolicy::DeleteImmediately => ec.despawn_recursive(),
        DeletionPolicy::Linger(duration) => {
            ec.insert(ScheduledForDeletion {
                timer: Timer::new(duration, TimerMode::Once),
            });
        }
    }
}

pub(crate) fn update_recursive<'c, R: HierarchyRoot, N: HierarchyNode>(
    commands: &mut Commands,
    entity_ref: EntityRef,
    node: N,
    context: &<<N as HasContext>::Context as NodeContext>::Wrapper<'c>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef, HierarchyChildComponent<R>)>>,
) {
    let mut ec = commands.entity(entity_ref.id());
    let undeleted = if entity_ref.contains::<ScheduledForDeletion>() {
        ec.remove::<ScheduledForDeletion>();
        true
    } else {
        false
    };

    let old_args = entity_ref
        .get::<HierarchyNodeComponent<N>>()
        .map(|x| &x.node);

    let args_changed = !old_args.is_some_and(|oa| node.eq(oa));

    let children = entity_ref.get::<Children>();

    let component_context = N::components_context(context);
    let component_args = N::as_component_aspect(&node);
    let children_args = N::as_children_aspect(&node);
    let children_context = N::children_context(context);

    // info!(
    //     "Update recursive {n} args_changed {args_changed}",
    //     n = std::any::type_name::<N>()
    // );

    let old_component_args = old_args.map(|x| N::as_component_aspect(&x));

    let components_hot = undeleted
        || <<N::ComponentsAspect as HasContext>::Context as NodeContext>::has_changed(
            component_context,
        )
        || (args_changed
            && !old_component_args.is_some_and(|oa| oa == component_args));

    if components_hot {
        //info!("Components hot {}", std::any::type_name::<N>());
        let mut component_commands = ConcreteComponentCommands::new(entity_ref, &mut ec);

        <N::ComponentsAspect as ComponentsAspect>::set_components(
            component_args,
            old_component_args,
            &component_context,
            &mut component_commands,
            if undeleted {
                SetComponentsEvent::Undeleted
            } else {
                SetComponentsEvent::Updated
            },
        );
    }

    let old_children_args = old_args.map(|x| N::as_children_aspect(&x));

    let children_hot =
        <<N::ChildrenAspect as HasContext>::Context as NodeContext>::has_changed(children_context)
            || (args_changed
                && !old_children_args.is_some_and(|oa| oa == children_args));

    if children_hot {
        //info!("Children hot {}", std::any::type_name::<N>());
        let mut children_commands =
            UnorderedChildCommands::<R>::new(&mut ec, children, all_child_nodes.clone());

        <N::ChildrenAspect as ChildrenAspect>::set_children(
            children_args,
            old_children_args,
            &children_context,
            &mut children_commands,
        );
        children_commands.finish();
    }

    if args_changed {
        ec.insert(HierarchyNodeComponent::<N> { node });
    }
}
