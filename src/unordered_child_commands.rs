use std::{any::type_name, marker::PhantomData, rc::Rc};

use crate::prelude::*;
use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    utils::hashbrown::HashMap,
};

pub(crate) struct UnorderedChildCommands<
    'w,
    's,
    'a,
    'b,
    'w1,
    'w_e,
    'd,
    'r,
    R: HierarchyRoot,
    NParent: AncestorAspect,
> {
    entity_ref: EntityRef<'w_e>,
    ec: &'b mut EntityCommands<'w, 's, 'a>,
    context: &'d <NParent::Context as NodeContext>::Wrapper<'r>,

    remaining_old_entities: HashMap<ChildKey, EntityRef<'w1>>,
    all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, HierarchyChildComponent<R>)>>,
    phantom: PhantomData<R>,
}

impl<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R: HierarchyRoot, NParent: AncestorAspect>
    ChildCommands<NParent>
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R, NParent>
{
    fn add_child<'c, NChild: HierarchyNode>(
        &mut self,
        key: impl Into<ChildKey>,
        child: NChild,
    ) where
        NParent: HasChild<NChild>,
    {
        let child_context = <NParent as HasChild<NChild>>::convert_context(self.context);
        //let context_changed = <NChild::Context as NodeContext>::has_changed(context);
        let key = key.into();

        match self.remaining_old_entities.remove(&key) {
            Some(entity_ref) => {
                //check if this node has changed

                if entity_ref.contains::<HierarchyNodeComponent<NChild>>() {
                    update_recursive::<R, NChild>(
                        &mut self.ec.commands(),
                        entity_ref.clone(),
                        child,
                        child_context,
                        self.all_child_nodes.clone(),
                    );
                } else {
                    warn!(
                        "Child with key '{key}' has had node type changed to {}",
                        type_name::<NChild>()
                    );
                    // The node type has changed - delete this entity and readd
                    self.ec
                        .commands()
                        .entity(entity_ref.id())
                        .despawn_recursive();

                    self.ec.with_children(|cb| {
                        let mut cec = cb.spawn_empty();
                        create_recursive::<R, NParent, NChild>(
                            &mut cec,
                            child,
                            &child_context,
                            key,
                        );
                    });
                }
            }
            None => {
                self.ec.with_children(|cb| {
                    let mut cec = cb.spawn_empty();
                    create_recursive::<R, NParent, NChild>(
                        &mut cec,
                        child,
                        &child_context,
                        key,
                    );
                });
            }
        }
    }
}

impl<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R: HierarchyRoot, NParent: AncestorAspect>
    UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R, NParent>
{
    pub(crate) fn new(
        ec: &'b mut EntityCommands<'w, 's, 'a>,
        entity_ref: EntityRef<'w_e>,
        children: Option<&Children>,
        context: &'d <NParent::Context as NodeContext>::Wrapper<'r>,
        all_child_nodes: Rc<HashMap<Entity, (EntityRef<'w1>, HierarchyChildComponent<R>)>>,
    ) -> Self {
        //let tree = tree.clone();
        match children {
            Some(children) => {
                let remaining_old_entities: HashMap<ChildKey, EntityRef<'w1>> = children
                    .iter()
                    .flat_map(|x| match all_child_nodes.get(x) {
                        Some((er, child)) => Some((child.key, er.clone())),
                        None => {
                            //new_entities.push(*x);
                            None
                        }
                    })
                    .collect();

                Self {
                    ec,
                    entity_ref,
                    remaining_old_entities,
                    all_child_nodes,
                    phantom: PhantomData,
                    context,
                }
            }
            None => Self {
                ec,
                entity_ref,
                remaining_old_entities: Default::default(),
                all_child_nodes,
                phantom: PhantomData,
                context,
            },
        }
    }

    pub fn finish(self) {
        let ec = self.ec;

        //remove all remaining old entities
        for (_key, entity_ref) in self.remaining_old_entities {
            delete_recursive::<NParent>(ec.commands(), entity_ref, self.context);
        }
    }
}

impl<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R: HierarchyRoot, NParent: AncestorAspect> CommandsBase
    for UnorderedChildCommands<'w, 's, 'a, 'b, 'w1, 'w_e, 'd, 'r, R, NParent>
{
    fn get<T: Component>(&self) -> Option<&T> {
        self.entity_ref.get()
    }
}
