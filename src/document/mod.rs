use gc::Trace;
use js::{
    object::{ConstructorBuilder, ObjectData},
    prelude::*,
};
use kuchiki::NodeRef;
use tap::{Conv, Pipe};

use crate::{Node, DOM};

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
    node: Node,
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
        // TODO: Find better way to get prototype
        let prototype = context
            .global_object()
            .clone()
            .get("Node", context)
            .unwrap()
            .as_object()
            .unwrap()
            .get("prototype", context)
            .unwrap()
            .as_object()
            .unwrap()
            .clone();
        ConstructorBuilder::new(context, Self::constructor)
            .name(Self::NAME)
            .inherit(prototype)
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
        let node = Node::new(node);
        Document { node }
    }

    pub fn as_node(&self) -> &Node {
        &self.node
    }
}
