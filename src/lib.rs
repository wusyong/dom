#[macro_use]
extern crate gc;

mod builtin;
pub use builtin::init;
mod document;
pub use document::Document;
mod window;
pub use window::Window;

pub mod wrapper {
    pub use super::document::wrapper::Document;
    pub use super::window::wrapper::Window;
}
