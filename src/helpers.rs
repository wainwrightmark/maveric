pub use crate::prelude::*;
use bevy::ecs::system::EntityCommands;
pub use bevy::prelude::*;

pub(crate) fn create_recursive<R: MavericRoot, N: MavericNode>(
    mut ec: EntityCommands,
    node: N,
    context: &<N::Context as NodeContext>::Wrapper<'_>,
    key: ChildKey,
    world: &World,
) -> Entity {
    let component_commands = NodeCommands::<N, N::Context, R, false>::new(
        &node,
        None,
        context,
        SetEvent::Created,
        world,
        &mut ec,
    );

    N::set_components(component_commands);

    let children_commands = NodeCommands::<N, N::Context, R, true>::new(
        &node,
        None,
        context,
        SetEvent::Created,
        world,
        &mut ec,
    );

    N::set_children(children_commands);

    let hnc = MavericNodeComponent::new(node);
    let hcc = MavericChildComponent::<R>::new::<N>(key);

    ec.insert((hnc, hcc));
    ec.id()
}

/// Recursively delete an entity. Returns the entity id if it is to linger.
#[must_use]
pub(crate) fn delete_recursive<R: RootChildren>(
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
        .map(|ac| ac.deleter.on_deleted(entity, &mut cc, world))
        .unwrap_or(DeletionPolicy::DeleteImmediately);

    match dp {
        DeletionPolicy::DeleteImmediately => {
            ec.despawn_recursive();
            None
        }
        DeletionPolicy::Linger(duration) => {
            cc.insert(ScheduledForDeletion {
                timer: Timer::new(duration, TimerMode::Once),
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

    let component_commands = NodeCommands::<N, N::Context, R, false>::new(
        &node, previous, context, event, world, &mut ec,
    );

    N::set_components(component_commands);

    let children_commands = NodeCommands::<N, N::Context, R, true>::new(
        &node, previous, context, event, world, &mut ec,
    );
    N::set_children(children_commands);

    let node_changed = previous.map(|p| !p.eq(&node)).unwrap_or(true);

    if node_changed {
        ec.insert(MavericNodeComponent::<N> { node });
    }
}
