use bevy::prelude::*;

pub trait HasDetectChanges {
    fn has_changed(&self)->bool;
}

impl<'c, R:Resource> HasDetectChanges for Res<'c, R>{
    fn has_changed(&self)->bool {
        self.is_changed()
    }
}

macro_rules! impl_state_tree_args {
    ($(($T:ident, $t:ident)),*) => {
        impl<$($T : DetectChanges),*> HasDetectChanges for ($($T,)*)  {


        fn has_changed(
            &self,
        ) -> bool {
            let &($($t,)*) = &self;
            false $(|| $t.is_changed())*
        }
        }
    }
}

bevy::utils::all_tuples!(impl_state_tree_args, 0, 15, T, t);