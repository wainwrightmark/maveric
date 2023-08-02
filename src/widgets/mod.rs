pub mod text_node;
pub mod button_node;
pub mod carousel;

pub mod prelude{
    pub use crate::widgets::text_node::*;
    pub use crate::widgets::button_node::*;
    pub use crate::widgets::carousel::*;
}