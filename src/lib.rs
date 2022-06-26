#![doc = include_str!("../README.md")]

pub mod prelude;
pub use prelude::init;

////////////////
// DOM objects
////////////////
mod document;
pub use document::Document;
mod node;
pub use node::Node;
mod window;
pub use window::Window;
