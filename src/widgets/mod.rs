pub mod button_node;
pub mod carousel;
pub mod text_node;
pub mod either;
pub mod static_components;

pub mod prelude {
    pub use crate::widgets::button_node::*;
    pub use crate::widgets::carousel::*;
    pub use crate::widgets::text_node::*;
    pub use crate::widgets::either::*;
    pub use crate::widgets::static_components::*;
}
