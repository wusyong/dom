#![doc = include_str!("../README.md")]

#[macro_use]
extern crate gc;

pub mod prelude;
pub use js;
pub use prelude::{init, DOM};

////////////////
// DOM objects
////////////////
mod document;
pub use document::Document;
mod window;
pub use window::Window;
