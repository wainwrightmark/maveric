use std::{any::type_name, marker::PhantomData, rc::Rc};

use crate::{prelude::*, DeletionPolicy};
use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    utils::{hashbrown::HashMap, HashSet},
};

pub(crate) struct RootCommands<'w, 's, 'b, 'w1, 'd, 'r, R: HierarchyRoot> {
    commands: &'b mut Commands<'w, 's>,
    remaining_old_entities: HashMap<ChildKey, (EntityRef<'w1>, HierarchyChildComponent<R>)>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, HierarchyChildComponent<R>)>>,
    context: &'d <R::Context as NodeContext>::Wrapper<'r>,
}

impl<'w, 's, 'b, 'w1, 'd, 'r, R: HierarchyRoot> RootCommands<'w, 's, 'b, 'w1, 'd, 'r, R> {
    pub(crate) fn new(
        commands: &'b mut Commands<'w, 's>,
        context: &'d <R::Context as NodeContext>::Wrapper<'r>,
        all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, HierarchyChildComponent<R>)>>,
        query: Query<Entity, (Without<Parent>, With<HierarchyChildComponent<R>>)>,
    ) -> Self {
        let remaining_old_entities: HashMap<
            ChildKey,
            (EntityRef<'w1>, HierarchyChildComponent<R>),
        > = query
            .into_iter()
            .flat_map(|x| match all_child_nodes.get(&x) {
                Some((er, child)) => Some((child.key, (er.clone(), child.clone()))),
                None => {
                    //new_entities.push(*x);
                    None
                }
            })
            .collect();

        Self {
            commands,
            context,
            remaining_old_entities,
            all_child_nodes,
        }
    }

    pub(crate) fn finish(self) {
        //remove all remaining old entities
        for (_key, (er, child)) in self.remaining_old_entities {
            let mut child_ec = self.commands.entity(er.id());
            // todo linger

            child_ec.despawn_recursive();
        }
    }
}

impl<'w, 's, 'b, 'w1, 'd, 'r, R: HierarchyRoot> CommandsBase
    for RootCommands<'w, 's, 'b, 'w1, 'd, 'r, R>
{
    fn get<T: Component>(&self) -> Option<&T> {
        None
    }
}

impl<'w, 's, 'b, 'w1, 'd, 'r, R: HierarchyRoot> ChildCommands<R>
    for RootCommands<'w, 's, 'b, 'w1, 'd, 'r, R>
{
    fn add_child<'c, NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child_args: <NChild as NodeBase>::Args,
    ) where
        R: HasChild<NChild>,
    {
        let child_context = <R as HasChild<NChild>>::convert_context(self.context);

        let key = key.into();

        match self.remaining_old_entities.remove(&key) {
            Some((entity_ref, _)) => {
                if entity_ref.contains::<HierarchyNodeComponent<NChild>>() {
                    update_recursive::<R, NChild>(
                        &mut self.commands,
                        entity_ref.clone(),
                        child_args,
                        child_context,
                        self.all_child_nodes.clone(),
                    );
                } else {
                    warn!(
                        "Child with key '{key}' has had node type changed to {}",
                        type_name::<NChild>()
                    );
                    // The node type has changed - delete this entity and readd
                    self.commands.entity(entity_ref.id()).despawn_recursive();

                    let mut cec = self
                        .commands
                        .spawn(HierarchyChildComponent::<R>::new::<NChild>(key));
                    create_recursive::<R, NChild>(&mut cec, child_args, child_context);
                }
            }
            None => {
                let mut cec = self
                    .commands
                    .spawn(HierarchyChildComponent::<R>::new::<NChild>(key));
                create_recursive::<R, NChild>(&mut cec, child_args, &child_context);
            }
        }
    }
}
