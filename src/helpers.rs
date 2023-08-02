use std::rc::Rc;

pub use crate::prelude::*;
pub use bevy::prelude::*;
use bevy::{ecs::system::EntityCommands, utils::HashMap};

pub(crate) fn create_recursive<'c, R: HierarchyRoot, P: HasChild<N>, N: HierarchyNode>(
    mut cec: &mut EntityCommands,
    args: <N as NodeBase>::Args,
    context: &<<N as NodeBase>::Context as NodeContext>::Wrapper<'c>,
    key: ChildKey,
) {
    //info!("Creating Node {}", type_name::<N>());

    let ancestor_context = N::ancestor_context(context);
    let component_context = N::components_context(context);

    let c_context_ref = <<N::ComponentsAspect as NodeBase>::Context as NodeContext>::from_wrapper(
        component_context,
    );
    let a_context_ref =
        <<N::AncestorAspect as NodeBase>::Context as NodeContext>::from_wrapper(ancestor_context);

    let mut child_commands =
        CreationCommands::<R, N::AncestorAspect>::new(&mut cec, ancestor_context);

    let ancestor_args = N::ancestor_args(&args);
    let component_args = N::component_args(&args);

    <N::ComponentsAspect as ComponentsAspect>::set_components(
        component_args,
        &c_context_ref,
        &mut child_commands,
        SetComponentsEvent::Created,
    );
    <N::AncestorAspect as AncestorAspect>::set_children(
        ancestor_args,
        &a_context_ref,
        &mut child_commands,
    );

    let hnc = HierarchyNodeComponent::<N> { args };
    let hcc = HierarchyChildComponent::<R>::new::<N>(key.into());
    let ac = AncestorComponent::<P>::new::<N>();

    cec.insert((hnc, hcc, ac));
}

pub(crate) fn delete_recursive<'c, P: AncestorAspect>(
    commands: &mut Commands,
    entity_ref: EntityRef,
    parent_context: &<<P as NodeBase>::Context as NodeContext>::Wrapper<'c>,
) {
    if entity_ref.contains::<ScheduledForDeletion>() {
        return;
    }

    let mut ec = commands.entity(entity_ref.id());
    let mut cc = ConcreteComponentCommands::new(entity_ref, &mut ec);

    let dp: DeletionPolicy = if let Some(ac) = entity_ref.get::<AncestorComponent<P>>() {
        ac.deleter.on_deleted(entity_ref, &mut cc, parent_context)
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
    args: <N as NodeBase>::Args,
    context: &<<N as NodeBase>::Context as NodeContext>::Wrapper<'c>,
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
        .map(|x| &x.args);

    let args_changed = match old_args {
        Some(a) => args.eq(a),
        None => true,
    };

    let children = entity_ref.get::<Children>();

    let component_context = N::components_context(context);
    let component_args = N::component_args(&args);
    let ancestor_args = N::ancestor_args(&args);
    let ancestor_context = N::ancestor_context(context);

    let components_hot = undeleted
        || <<N::ComponentsAspect as NodeBase>::Context as NodeContext>::has_changed(
            component_context,
        )
        || (args_changed && old_args.is_some_and(|oa| N::component_args(&oa) == component_args));

    if components_hot {
        let mut component_commands = ConcreteComponentCommands::new(entity_ref, &mut ec);

        let c_context_ref =
            <<N::ComponentsAspect as NodeBase>::Context as NodeContext>::from_wrapper(
                component_context,
            );

        <N::ComponentsAspect as ComponentsAspect>::set_components(
            component_args,
            &c_context_ref,
            &mut component_commands,
            if undeleted {
                SetComponentsEvent::Undeleted
            } else {
                SetComponentsEvent::Updated
            },
        );
    }

    let ancestors_hot =
        <<N::AncestorAspect as NodeBase>::Context as NodeContext>::has_changed(ancestor_context)
            || (args_changed && old_args.is_some_and(|oa| N::ancestor_args(&oa) == ancestor_args));

    if ancestors_hot {
        let mut ancestor_commands = UnorderedChildCommands::<R, N::AncestorAspect>::new(
            &mut ec,
            entity_ref,
            children,
            ancestor_context,
            all_child_nodes.clone(),
        );

        let a_context_ref = <<N::AncestorAspect as NodeBase>::Context as NodeContext>::from_wrapper(
            ancestor_context,
        );

        <N::AncestorAspect as AncestorAspect>::set_children(
            ancestor_args,
            &a_context_ref,
            &mut ancestor_commands,
        );
        ancestor_commands.finish();
    }

    if args_changed {
        ec.insert(HierarchyNodeComponent::<N> { args });
    }
}
