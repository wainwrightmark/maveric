pub use crate::prelude::*;
use bevy::ecs::system::EntityCommands;
pub use bevy::prelude::*;

pub(crate) fn create_recursive<'c, R: MavericRoot, N: MavericNode>(
    ec: EntityCommands,
    node: N,
    context: &<N::Context as NodeContext>::Wrapper<'c>,
    key: ChildKey,
    world: &World,
)-> Entity {

    let args = NodeData::<N, N::Context, R, true>::new(&node, None, context, SetEvent::Created);
    let mut commands = NodeCommands::new(ec, world);

    N::set(args, &mut commands);

    let hnc = MavericNodeComponent::new(node);
    let hcc = MavericChildComponent::<R>::new::<N>(key.into());

    commands.ec.insert((hnc, hcc));
    commands.ec.id()
}

/// Recursively delete an entity. Returns the entity id if it is to linger.
#[must_use]
pub(crate) fn delete_recursive<'c, R: RootChildren>(
    commands: &mut Commands,
    entity: Entity,
    world: &World,
) -> Option<Entity> {
    if let Some(_) = world.get::<ScheduledForDeletion>(entity) {
        return Some(entity);
    }

    let ec = commands.entity(entity);
    let mut nc = NodeCommands::new(ec, world);
    let mut cc = ComponentCommands::new(&mut nc, SetEvent::Updated);

    let dp: DeletionPolicy = world
        .get::<MavericChildComponent<R>>(entity)
        .map(|ac| ac.deleter.on_deleted(entity, &mut cc, world))
        .unwrap_or(DeletionPolicy::DeleteImmediately);

    match dp {
        DeletionPolicy::DeleteImmediately => {
            nc.ec.despawn_recursive();
            return None;
        }
        DeletionPolicy::Linger(duration) => {
            cc.insert(ScheduledForDeletion {
                timer: Timer::new(duration, TimerMode::Once),
            });

            return Some(nc.ec.id());
        }
    }
}

pub(crate) fn update_recursive<'c, R: MavericRoot, N: MavericNode>(
    commands: &mut Commands,
    entity: Entity,
    node: N,
    context: &<N::Context as NodeContext>::Wrapper<'c>,
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

    let event = undeleted
        .then_some(SetEvent::Undeleted)
        .unwrap_or(SetEvent::Updated);

    //let concrete_commands = ConcreteComponentCommands::new(world, &mut ec);
    //let args = NodeArgs::new(&node, previous, context, event, concrete_commands);

    let args =
        NodeData::<N, N::Context, R, true> ::new(&node, previous, context, event);
    let mut commands = NodeCommands::new(ec, world);

    N::set(args, &mut commands);

    let node_changed = previous.map(|p| !p.eq(&node)).unwrap_or(true);

    if node_changed {
        commands.ec.insert(MavericNodeComponent::<N> { node });
    }
}
