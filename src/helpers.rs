pub use crate::prelude::*;
use bevy::ecs::system::EntityCommands;
pub use bevy::prelude::*;

pub(crate) fn create_recursive<'c, R: HierarchyRootChildren, N: HierarchyNode>(
    mut cec: &mut EntityCommands,
    node: N,
    context: &<N::Context as NodeContext>::Wrapper<'c>,
    key: ChildKey,
) {
    //info!("Creating Node {}", type_name::<N>());

    let mut child_commands = CreationCommands::<R>::new(&mut cec);

    node.set(
        None,
        &context,
        &mut child_commands,
        SetComponentsEvent::Created,
    );

    let hnc = HierarchyNodeComponent::new(node);
    let hcc = HierarchyChildComponent::<R>::new::<N>(key.into());

    cec.insert((hnc, hcc));
}

/// Recursively delete an entity. Returns the entity id if it is to linger.
#[must_use]
pub(crate) fn delete_recursive<'c, R: HierarchyRootChildren>(
    commands: &mut Commands,
    entity_ref: EntityRef,
    //parent_context: &<<P as HasContext>::Context as NodeContext>::Wrapper<'c>,
) -> Option<Entity> {
    if entity_ref.contains::<ScheduledForDeletion>() {
        return Some(entity_ref.id());
    }

    let mut ec = commands.entity(entity_ref.id());
    let mut cc = ConcreteComponentCommands::new(entity_ref, &mut ec);

    let dp: DeletionPolicy = entity_ref
        .get::<HierarchyChildComponent<R>>()
        .map(|ac| ac.deleter.on_deleted(entity_ref, &mut cc))
        .unwrap_or(DeletionPolicy::DeleteImmediately);

    match dp {
        DeletionPolicy::DeleteImmediately => {
            ec.despawn_recursive();
            return None;
        }
        DeletionPolicy::Linger(duration) => {
            ec.insert(ScheduledForDeletion {
                timer: Timer::new(duration, TimerMode::Once),
            });

            return Some(ec.id());
        }
    }
}

pub(crate) fn update_recursive<'c, R: HierarchyRootChildren, N: HierarchyNode>(
    commands: &mut Commands,
    entity_ref: EntityRef,
    node: N,
    context: &<N::Context as NodeContext>::Wrapper<'c>,
    world: &World,
) {
    let mut ec = commands.entity(entity_ref.id());
    let undeleted = if entity_ref.contains::<ScheduledForDeletion>() {
        ec.remove::<ScheduledForDeletion>();
        true
    } else {
        false
    };

    let old_node = entity_ref
        .get::<HierarchyNodeComponent<N>>()
        .map(|x| &x.node);

    let node_changed = !old_node.is_some_and(|oa| node.eq(oa));

    let children = entity_ref.get::<Children>();

    let hot = undeleted || <N::Context as NodeContext>::has_changed(context) || node_changed;
    if !hot {
        return;
    }

    let event = if undeleted {
        SetComponentsEvent::Undeleted
    } else {
        SetComponentsEvent::Updated
    };

    match N::CHILDREN_TYPE {
        ChildrenType::Ordered => {
            let mut children_commands =
                OrderedChildCommands::<R>::new(&mut ec, entity_ref, children, world);

            (&node).set(old_node, &context, &mut children_commands, event);
            children_commands.finish();
        }
        ChildrenType::Unordered => {
            let mut children_commands =
                UnorderedChildCommands::<R>::new(&mut ec, entity_ref, children, world);

            (&node).set(old_node, &context, &mut children_commands, event);
            children_commands.finish();
        }
    }

    if node_changed {
        ec.insert(HierarchyNodeComponent::<N> { node });
    }
}
