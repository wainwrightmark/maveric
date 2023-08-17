use bevy::{
    asset::Asset,
    prelude::{AssetServer, Handle},
};

#[cfg(feature = "more_bevy")]
pub mod button_node;
#[cfg(feature = "more_bevy")]
pub mod carousel;
#[cfg(feature = "more_bevy")]
pub mod text_node;

#[cfg(feature = "more_bevy")]
pub mod image_node;

pub mod prelude {

    #[cfg(feature = "more_bevy")]
    pub use crate::widgets::button_node::*;
    #[cfg(feature = "more_bevy")]
    pub use crate::widgets::carousel::*;
    #[cfg(feature = "more_bevy")]
    pub use crate::widgets::image_node::*;
    #[cfg(feature = "more_bevy")]
    pub use crate::widgets::text_node::*;
}

pub(crate) fn get_or_load_asset<T: Asset>(path: &str, server: &AssetServer) -> Handle<T> {
    let asset: Handle<T> = match server.get_load_state(path) {
        bevy::asset::LoadState::Loaded => server.get_handle(path),
        _ => server.load(path),
    };

    asset
}
