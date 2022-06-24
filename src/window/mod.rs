//! This module implements the global `Window` object.
//!
//! The `Window` interface represents a window containing a DOM document; the document property points to the DOM document loaded in that window.
//!
//! More information:
//!  - [DOM reference][spec]
//!  - [MDN documentation][mdn]
//!
//! [spec]: https://html.spec.whatwg.org/multipage/window-object.html#the-window-object
//! [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Window

pub mod wrapper;

use gc::Trace;
use js::{
    object::{ConstructorBuilder, ObjectData},
    prelude::JsObject,
    Context, JsResult, JsValue,
};
use kuchiki::traits::TendrilSink;
use tap::{Conv, Pipe};

use crate::{builtin::BuiltIn, Document};

/// DOM `Window` built-in implementation.
#[derive(Debug, Trace, Finalize)]
pub struct Window;

impl BuiltIn for Window {
    const NAME: &'static str = "Window";

    fn init(context: &mut Context) -> Option<JsValue> {
        ConstructorBuilder::new(context, Self::constructor)
            .name(Self::NAME)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }
}

impl Window {
    fn constructor(
        _new_target: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        // `Window` has no constructor.
        context.throw_type_error("Illegal constructor.")
    }

    fn new(context: &mut Context) -> JsResult<JsObject> {
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
        let document = Document::new(node, context)?;

        // TODO: Find better way to get prototype
        let prototype = context
            .global_object()
            .clone()
            .get("Window", context)?
            .as_object()
            .unwrap()
            .get("prototype", context)?
            .as_object()
            .unwrap()
            .clone();

        let window = JsObject::from_proto_and_data(prototype, ObjectData::global());
        window.set("document", document, false, context)?;
        Ok(window)
    }
}
