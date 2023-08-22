pub use crate::prelude::*;
use bevy::ecs::system::EntityCommands;
pub use bevy::prelude::*;

pub(crate) fn create_recursive<'c, R: HierarchyRoot, N: HierarchyNode>(
    mut ec: EntityCommands,
    node: N,
    context: &<N::Context as NodeContext>::Wrapper<'c>,
    key: ChildKey,
    world: &World,
)-> Entity {
    let component_commands =
        SetComponentCommands::new(&node, None, context, SetEvent::Created, ec, world);
    let token = N::set_components(component_commands);
    let set_children_commands =
        SetChildrenCommands::new(&node, None, context, SetEvent::Created, token.commands, world);
    N::set_children::<R>(set_children_commands);

    let hnc = HierarchyNodeComponent::new(node);
    let hcc = HierarchyChildComponent::<R>::new::<N>(key.into());

    ec.insert((hnc, hcc));
    ec.id()
}

/// Recursively delete an entity. Returns the entity id if it is to linger.
#[must_use]
pub(crate) fn delete_recursive<'c, R: HierarchyRootChildren>(
    commands: &mut Commands,
    entity: Entity,
    world: &World,
) -> Option<Entity> {
    if let Some(x) = world.get::<ScheduledForDeletion>(entity) {
        return Some(entity);
    }

    let mut ec = commands.entity(entity);
    let mut cc = ConcreteComponentCommands::new(&mut ec, world);

    let dp: DeletionPolicy = world
        .get::<HierarchyChildComponent<R>>(entity)
        .map(|ac| ac.deleter.on_deleted(entity, &mut cc, world))
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

pub(crate) fn update_recursive<'c, R: HierarchyRoot, N: HierarchyNode>(
    commands: &mut Commands,
    entity: Entity,
    node: N,
    context: &<N::Context as NodeContext>::Wrapper<'c>,
    world: &World,
) {
    let mut ec = commands.entity(entity);
    let undeleted = if world.get::<ScheduledForDeletion>(entity).is_some() {
        ec.remove::<ScheduledForDeletion>();
        true
    } else {
        false
    };
    let previous = world
        .get::<HierarchyNodeComponent<N>>(entity)
        .map(|x| &x.node);

    let event = undeleted
        .then_some(SetEvent::Undeleted)
        .unwrap_or(SetEvent::Updated);

    //let concrete_commands = ConcreteComponentCommands::new(world, &mut ec);
    //let args = NodeArgs::new(&node, previous, context, event, concrete_commands);

    let set_component_commands =
        SetComponentCommands::new(&node, previous, context, event, ec, world);

    let token = N::set_components(set_component_commands);
    let set_children_commands =
        SetChildrenCommands::new(&node, previous, context, event, token.commands, token.world);

    N::set_children::<R>(set_children_commands);

    let node_changed = previous.map(|p| !p.eq(&node)).unwrap_or(true);

    if node_changed {
        ec.insert(HierarchyNodeComponent::<N> { node });
    }
}
