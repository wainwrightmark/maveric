// use std::marker::PhantomData;

// use crate::prelude::*;
// use bevy::{ecs::system::EntityCommands, prelude::*};

// pub(crate) struct CreationCommands<'w, 's, 'a, R: HierarchyRoot> {
//     ec: EntityCommands<'w, 's, 'a>,
//     phantom: PhantomData<R>,
// }

// impl<'w, 's, 'a, 'b, R: HierarchyRoot> ChildCommands for CreationCommands<'w, 's, 'a, R> {
//     fn add_child<NChild: HierarchyNode>(
//         &mut self,
//         key: impl Into<ChildKey>,
//         child: NChild,
//         context: &<NChild::Context as NodeContext>::Wrapper<'_>,
//     ) {
//         self.ec.with_children(|cb| {
//             let key = key.into();
//             let mut cec = cb.spawn_empty();
//             create_recursive::<R, NChild>(cec, child, &context, key);
//         });
//     }
// }

// impl<'w, 's, 'a, 'b, R: HierarchyRoot> CreationCommands<'w, 's, 'a, R> {
//     pub(crate) fn new(ec: EntityCommands<'w, 's, 'a>) -> Self {
//         Self {
//             ec,
//             phantom: PhantomData,
//         }
//     }
// }

// impl<'w, 's, 'a, 'b, R: HierarchyRoot> ComponentCommands for CreationCommands<'w, 's, 'a, R> {
//     fn insert<T: Bundle>(&mut self, bundle: T) {
//         self.ec.insert(bundle);
//     }

//     fn remove<T: Bundle>(&mut self) {}

//     fn get<T: Component>(&self) -> Option<&T> {
//         None
//     }
// }
