pub use crate::prelude::*;
use bevy::ecs::system::EntityCommands;
pub use bevy::prelude::*;

pub(crate) fn create_recursive<R: MavericRoot, N: MavericNode>(
    mut entity_commands: EntityCommands,
    node: N,
    context: &<N::Context as NodeContext>::Wrapper<'_>,
    key: ChildKey,
    world: &World,
    alloc: & bumpalo::Bump,
) -> Entity {
    let component_commands = SetComponentCommands::<N, N::Context>::new(
        NodeArgs::new(context, SetEvent::Created, &node, None),
        world,
        &mut entity_commands,
    );

    N::set_components(component_commands);

    let children_commands = SetChildrenCommands::<N, N::Context, R>::new(
        NodeArgs::new(context, SetEvent::Created, &node, None),
        world,
        &mut entity_commands,
        alloc,
    );

    N::set_children(children_commands);
    node.on_created(context, world, &mut entity_commands);
    let node_component = MavericNodeComponent::new(node);
    let child_component = MavericChildComponent::<R>::new::<N>(key);

    entity_commands.insert((node_component, child_component));



    entity_commands.id()
}

/// Recursively delete an entity. Returns the entity id if it is to linger.
#[must_use]
pub(crate) fn delete_recursive<R: MavericRootChildren>(
    commands: &mut Commands,
    entity: Entity,
    world: &World,
) -> Option<Entity> {
    if world.get::<ScheduledForDeletion>(entity).is_some() {
        return Some(entity);
    }

    let mut ec = commands.entity(entity);

    let mut cc = ComponentCommands::new(&mut ec, world, SetEvent::Updated);

    let dp: DeletionPolicy = world
        .get::<MavericChildComponent<R>>(entity)
        .map_or(DeletionPolicy::DeleteImmediately, |ac| {
            ac.deleter.on_deleted(entity, &mut cc, world)
        });

    match dp {
        DeletionPolicy::DeleteImmediately => {
            ec.despawn_recursive();
            None
        }
        DeletionPolicy::Linger(duration) => {
            cc.insert(ScheduledForDeletion {
                remaining: duration
            });

            Some(ec.id())
        }
    }
}

pub(crate) fn update_recursive<R: MavericRoot, N: MavericNode>(
    commands: &mut Commands,
    entity: Entity,
    node: N,
    context: &<N::Context as NodeContext>::Wrapper<'_>,
    world: &World,
    alloc: & bumpalo::Bump,
) {
    let mut ec = commands.entity(entity);
    let undeleted = if world.get::<ScheduledForDeletion>(entity).is_some() {
        ec.remove::<ScheduledForDeletion>();
        //info!("Node Undeleted");
        true
    } else {
        false
    };
    let previous = world
        .get::<MavericNodeComponent<N>>(entity)
        .map(|x| &x.node);

    let event = if undeleted {
        SetEvent::Undeleted
    } else {
        SetEvent::Updated
    };

    let component_commands = SetComponentCommands::<N, N::Context>::new(
        NodeArgs::new(context, event, &node, previous),
        world,
        &mut ec,
    );

    N::set_components(component_commands);

    let children_commands = SetChildrenCommands::<N, N::Context, R>::new(
        NodeArgs::new(context, event, &node, previous),
        world,
        &mut ec,
        alloc,
    );
    N::set_children(children_commands);

    let node_changed = previous.map_or(true, |p| !p.eq(&node));

    if node_changed {
        if let Some(previous) = previous {
            node.on_changed(previous, context, world, &mut ec);
        } else {
            node.on_created(context, world, &mut ec);
        }
        ec.insert(MavericNodeComponent::<N> { node });
    }
}
