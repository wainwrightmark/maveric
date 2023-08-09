use std::rc::Rc;

use bevy::{
    ecs::system::{EntityCommands, StaticSystemParam},
    prelude::*,
    utils::hashbrown::HashMap,
};
use prelude::*;

pub mod child_deletion_policy;
pub mod child_key;
pub mod commands;
pub mod components;
pub mod desired_transform;
pub mod hierarchy_node;
pub mod hierarchy_root;
pub mod has_child;
pub mod node_context;
pub mod transition;
pub mod widgets;

pub mod concrete_component_commands;
pub mod creation_commands;
pub mod plugin;
pub mod root_commands;
pub mod unordered_child_commands;
pub mod helpers;

pub mod prelude {
    pub use crate::commands::*;
    pub(crate) use crate::components::*;

    pub use crate::child_deletion_policy::*;
    pub use crate::child_key::*;
    pub use crate::desired_transform::*;
    pub use crate::hierarchy_node::*;
    pub use crate::hierarchy_root::*;
    pub use crate::has_child::*;
    pub use crate::node_context::*;
    pub use crate::transition::prelude::*;
    pub use crate::widgets::prelude::*;

    pub (crate) use crate::concrete_component_commands::*;
    pub (crate) use crate::creation_commands::*;
    pub (crate) use crate::root_commands::*;
    pub (crate) use crate::unordered_child_commands::*;

    pub use crate::plugin::*;

    pub (crate) use crate::helpers::*;
}
