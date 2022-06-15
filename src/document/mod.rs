//! This module implements the global `Document` object.
//!
//! The `Document` interface represents any web page loaded in the browser and serves as an entry point into the web page's content, which is the DOM tree.
//!
//! More information:
//!  - [DOM reference][spec]
//!  - [MDN documentation][mdn]
//!
//! [spec]: https://dom.spec.whatwg.org/#interface-document
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/document

pub mod wrapper;

use gc::Trace;
use js::prelude::*;
use kuchiki::NodeRef;

use crate::builtin::BuiltIn;

/// DOM `Document` built-in implementation.
#[derive(Debug, Finalize)]
pub struct Document {
    pub node: NodeRef,
}

// Safety: NodeRef is already reference counted.
unsafe impl Trace for Document {
    unsafe_empty_trace!();
}

impl BuiltIn for Document {
    const NAME: &'static str = "Document";

    fn init(context: &mut Context) -> Option<JsValue> {
        todo!()
    }
}

impl Document {
    fn constructor(
        new_target: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        todo!()
    }
}
