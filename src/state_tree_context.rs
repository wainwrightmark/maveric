use bevy::{
    ecs::system::SystemParam,
    prelude::*
};


// pub trait StateTreeContext<'world, 'state> : SystemParam<Item<'world, 'state> = Self> + DetectChanges{

//     fn is_changed(&self)-> bool;
// }






// /// State tree args is automatically implemented for all tuples of types which implement `Resource + Clone`.
// /// This includes `()` and 1-tuples like `(AssetServer,)`
// pub trait StateTreeContext: Clone + Send + Sync {
//     type Param<'world>: SystemParam;

//     fn clone_from_system_param<'world: 'a, 'a>(
//         item: &'a <Self::Param<'_> as SystemParam>::Item<'world, '_>,
//     ) -> Self;

//     fn is_changed<'world: 'a, 'a>(
//         item: &'a <Self::Param<'_> as SystemParam>::Item<'world, '_>,
//     ) -> bool;
// }

// macro_rules! impl_state_tree_args {
//     ($(($T:ident, $t:ident)),*) => {
//         impl<$($T : Resource + Clone),*> StateTreeContext for ($($T,)*)  {
//             type Param<'world> = ($(Res<'world, $T>,)*);


//         fn is_changed<'world: 'a, 'a>(
//             item: &'a <Self::Param<'_> as SystemParam>::Item<'world, '_>,
//         ) -> bool {
//             let &($($t,)*) = &item;
//             false $(|| $t.is_changed())*
//         }

//         fn clone_from_system_param<'world: 'a, 'a>(
//             item: &'a <Self::Param<'_> as SystemParam>::Item<'world, '_>,
//         ) -> Self {
//             let ($($t,)*) = item;
//             ($($T::clone($t),)*)
//         }
//         }
//     }
// }

// bevy::utils::all_tuples!(impl_state_tree_args, 0, 15, T, t);
