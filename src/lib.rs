#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::option_if_let_else)]

pub mod child_commands;
pub mod child_deletion_policy;
pub mod child_key;
pub mod child_tuple;
pub mod component_commands;
pub mod components;
pub mod deleter;
pub mod helpers;
pub mod into_components;
pub mod node;

pub mod allocator;
pub mod node_args;
pub mod node_context;
pub mod plugin;
pub mod root;
pub mod root_commands;
pub mod set_children_commands;
pub mod set_components_commands;
pub mod set_event;
pub mod transition;
pub mod with_bundle;
#[cfg(any(feature = "widgets", test))]
pub mod widgets;
pub mod scheduled_for_deletion;
pub mod scheduled_change;

pub mod prelude {
    pub use crate::child_commands::*;
    pub use crate::child_deletion_policy::*;
    pub use crate::child_key::*;
    pub use crate::child_tuple::*;
    pub use crate::component_commands::*;

    pub use crate::deleter::*;
    pub use crate::into_components::*;
    pub use crate::node::*;
    pub use crate::node_args::*;
    pub use crate::node_context::*;
    pub use crate::plugin::*;
    pub use crate::root::*;
    pub use crate::set_children_commands::*;
    pub use crate::set_components_commands::*;
    pub use crate::set_event::*;
    pub use crate::scheduled_for_deletion::*;
    pub use crate::scheduled_change::*;
    pub use crate::transition::prelude::*;
    pub use crate::with_bundle;

    #[cfg(any(feature = "widgets", test))]
    pub use crate::widgets::prelude::*;

    pub (crate) use crate::components::*;
    pub(crate) use crate::allocator::*;
    pub(crate) use crate::helpers::*;
    pub(crate) use crate::root_commands::*;

    #[cfg(any(feature = "derive", test))]
    pub use maveric_macro::{MavericContext, MavericRoot};
}
