use gc::Trace;
use js::{
    object::{ConstructorBuilder, ObjectData},
    prelude::*,
};
use kuchiki::NodeRef;
use tap::{Conv, Pipe};

use crate::{Document, DOM};

#[derive(Debug, Clone, Finalize)]
pub struct Node {
    inner: NodeRef,
}

// Safety: NodeRef is already reference counted.
unsafe impl Trace for Node {
    unsafe_empty_trace!();
}

impl DOM for Node {
    const NAME: &'static str = "Node";

    fn constructor(_: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        // `Node` has no constructor.
        context.throw_type_error("Illegal constructor.")
    }

    fn init(context: &mut Context) -> Option<JsValue> {
        ConstructorBuilder::new(context, Self::constructor)
            .name(Self::NAME)
            .constructor(false)
            .method(Self::get_root_node, "getRootNode", 0)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }

    fn js_object(&self, context: &mut Context) -> JsResult<JsObject> {
        // TODO: Find better way to get prototype
        let prototype = context
            .global_object()
            .clone()
            .get("Node", context)?
            .as_object()
            .unwrap()
            .get("prototype", context)?
            .as_object()
            .unwrap()
            .clone();

        let node = JsObject::from_proto_and_data(
            prototype,
            ObjectData::native_object(Box::new(self.clone())),
        );
        Ok(node)
    }
}

/// Methods for JS Context.
impl Node {
    // FIXME: This is not the correct way to do this.
    pub fn get_root_node(
        this: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        match this
            .as_object()
            .unwrap()
            .downcast_ref::<Document>()
            .unwrap()
            .as_node()
            .as_raw()
            .ancestors()
            .last()
        {
            Some(node) => Ok(Node::new(node).js_object(context)?.into()),
            None => Ok(JsValue::Null),
        }
    }
}

/// Methods for Rust.
impl Node {
    pub fn new(inner: NodeRef) -> Self {
        Node { inner }
    }

    pub fn as_raw(&self) -> &NodeRef {
        &self.inner
    }

    /// Get the parent node of this node.
    pub fn parent_node(&self) -> Option<NodeRef> {
        self.inner.parent()
    }
}
