pub mod button_node;
pub mod carousel;
pub mod image_node;
pub mod text_node;
pub mod text2d_node;

pub mod prelude {
    pub use crate::widgets::button_node::*;
    pub use crate::widgets::carousel::*;
    pub use crate::widgets::image_node::*;
    pub use crate::widgets::text_node::*;
}

// pub(crate) fn get_or_load_asset<T: Asset>(path: &str, server: &AssetServer) -> Handle<T> {


//     server.load(path)

// }
