use js::prelude::*;

/// DOM [`Document`][document] Rust object.
///
/// [document]: https://developer.mozilla.org/docs/Web/API/document
#[derive(Debug, Clone, Trace, Finalize)]
pub struct Document {
    inner: JsObject,
}
