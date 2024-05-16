#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::option_if_let_else)]

pub mod cached;
pub mod child_commands;
pub mod child_deletion_policy;
pub mod child_key;
pub mod child_tuple;
pub mod component_commands;
pub mod components;
pub mod deleter;
pub mod has_changed;
pub mod has_item_changed;
pub mod helpers;
pub mod into_components;
pub mod node;
pub mod with_previous;

pub mod maveric_context;
pub mod node_args;
pub mod plugin;
pub mod root;
pub mod root_commands;
pub mod scheduled_change;
pub mod scheduled_for_deletion;
pub mod set_children_commands;
pub mod set_components_commands;
pub mod set_event;
pub mod transition;

#[cfg(feature = "tracing")]
pub mod tracing;
#[cfg(any(feature = "widgets", test))]
pub mod widgets;
pub mod with_bundle;

pub mod prelude {
    pub use crate::child_commands::*;
    pub use crate::child_deletion_policy::*;
    pub use crate::child_key::*;
    pub use crate::child_tuple::*;
    pub use crate::component_commands::*;

    pub use crate::deleter::*;
    pub use crate::into_components::*;
    pub use crate::maveric_context::*;
    pub use crate::node::*;
    pub use crate::node_args::*;
    pub use crate::plugin::*;
    pub use crate::root::*;
    pub use crate::scheduled_change::*;
    pub use crate::scheduled_for_deletion::*;
    pub use crate::set_children_commands::*;
    pub use crate::set_components_commands::*;
    pub use crate::set_event::*;
    pub use crate::transition::prelude::*;
    pub use crate::with_bundle;

    #[cfg(any(feature = "widgets", test))]
    pub use crate::widgets::prelude::*;

    pub(crate) use crate::components::*;

    pub(crate) use crate::helpers::*;
    pub(crate) use crate::root_commands::*;

    #[cfg(any(feature = "derive", test))]
    pub use maveric_macro::HasChanged;

    #[cfg(feature = "bumpalo")]
    pub(crate) type Allocator = bumpalo::Bump;
    #[cfg(not(feature = "bumpalo"))]
    pub(crate) type Allocator = allocator_api2::alloc::Global;

    pub(crate) fn reset_allocator(allocator: &mut Allocator) {
        #[cfg(feature = "bumpalo")]
        {
            allocator.reset();
        }
    }
}
