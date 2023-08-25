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

pub mod node_context;
pub mod plugin;
pub mod root;
pub mod root_commands;
pub mod set_event;
pub mod transition;
pub mod set_children_commands;
pub mod set_components_commands;
pub mod allocator;
#[cfg(feature = "more_bevy")]
pub mod widgets;

pub mod prelude {
    pub use crate::child_commands::*;
    pub use crate::child_deletion_policy::*;
    pub use crate::child_key::*;
    pub use crate::child_tuple::*;
    pub use crate::component_commands::*;
    pub use crate::deleter::*;
    pub use crate::into_components::*;
    pub use crate::node::*;
    pub use crate::set_children_commands::*;
    pub use crate::set_components_commands::*;
    pub use crate::node_context::*;
    pub use crate::plugin::*;
    pub use crate::root::*;
    pub use crate::set_event::*;
    pub use crate::transition::prelude::*;
    #[cfg(feature = "more_bevy")]
    pub use crate::widgets::prelude::*;

    pub(crate) use crate::components::*;
    pub(crate) use crate::helpers::*;
    pub(crate) use crate::root_commands::*;
    pub(crate) use crate::allocator::*;

    pub use crate::impl_maveric_root;
}
