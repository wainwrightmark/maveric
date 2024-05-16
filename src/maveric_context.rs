use bevy::ecs::system::ReadOnlySystemParam;

use crate::{has_changed::HasChanged, has_item_changed::HasItemChanged};

pub trait MavericContext: ReadOnlySystemParam + HasItemChanged + HasChanged {}

impl<R: ReadOnlySystemParam + HasItemChanged + HasChanged> MavericContext for R {}
