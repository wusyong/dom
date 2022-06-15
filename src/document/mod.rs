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
use js::{
    object::{ConstructorBuilder, ObjectData},
    prelude::*,
};
use kuchiki::{traits::TendrilSink, NodeRef};
use tap::{Conv, Pipe};

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
        ConstructorBuilder::new(context, Self::constructor)
            .name(Self::NAME)
            .method(Self::say_hello, "say_hello", 0)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }
}

impl Document {
    fn constructor(
        _new_target: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        let document = Self::fake_new(context)?;
        Ok(document.into())
    }

    fn fake_new(context: &mut Context) -> JsResult<JsObject> {
        let html = r"
            <DOCTYPE html>
            <html>
            <head></head>
            <body>
                <h1>Example</h1>
                <p class='foo'>Hello, world!</p>
                <p class='foo'>I love HTML</p>
            </body>
            </html>
        ";
        let node = kuchiki::parse_html().one(html);
        let document = Document { node };

        // TODO: Find better way to get prototype
        let prototype = context
            .global_object()
            .clone()
            .get("Document", context)?
            .as_object()
            .unwrap()
            .get("prototype", context)?
            .as_object()
            .unwrap()
            .clone();
        Ok(JsObject::from_proto_and_data(prototype, ObjectData::native_object(Box::new(document))))
    }

    pub fn say_hello(_: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        println!("Hello World from Document.");
        Ok(JsValue::Null)
    }
}
