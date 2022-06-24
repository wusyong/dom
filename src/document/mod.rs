use gc::Trace;
use js::{
    object::{ConstructorBuilder, ObjectData},
    prelude::*,
};
use kuchiki::NodeRef;
use tap::{Conv, Pipe};

use crate::DOM;

/// The `Document` DOM object implementation.
///
/// The `Document` interface represents any web page loaded in the browser and serves as an entry point into the web page's content, which is the DOM tree.
///
/// More information:
///  - [DOM reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://dom.spec.whatwg.org/#interface-document
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/document
#[derive(Debug, Clone, Finalize)]
pub struct Document {
    node: NodeRef,
}

// Safety: NodeRef is already reference counted.
unsafe impl Trace for Document {
    unsafe_empty_trace!();
}

impl DOM for Document {
    const NAME: &'static str = "Document";

    /// Returns a new document.
    ///
    /// The `new Document()` constructor steps are to set this’s origin to the
    /// origin of current global object’s associated Document.
    ///
    /// More information:
    ///  - [DOM reference][spec]
    ///  - [MDN documentation][mdn]
    ///
    /// [spec]: https://dom.spec.whatwg.org/#dom-document-document
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Document/Document
    fn constructor(_: &JsValue, _: &[JsValue], _: &mut Context) -> JsResult<JsValue> {
        // We should get it from window's associated Document aka global object
        todo!()
    }

    fn init(context: &mut Context) -> Option<JsValue> {
        ConstructorBuilder::new(context, Self::constructor)
            .name(Self::NAME)
            .method(Self::say_hello, "say_hello", 0)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }

    fn js_object(&self, context: &mut Context) -> JsResult<JsObject> {
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

        Ok(JsObject::from_proto_and_data(
            prototype,
            ObjectData::native_object(Box::new(self.clone())),
        ))
    }
}

impl Document {
    pub fn new(node: NodeRef) -> Self {
        Document { node }
    }

    pub fn as_node(&self) -> &NodeRef {
        &self.node
    }

    pub fn say_hello(_: &JsValue, _: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        println!("Hello Document");
        Ok(JsValue::Undefined)
    }
}
