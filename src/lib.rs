#[macro_use]
extern crate gc;

mod builtin;
pub use builtin::init;
mod document;
pub use document::*;
