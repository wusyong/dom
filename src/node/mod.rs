use std::rc::Rc;

use kuchiki::NodeRef;
use v8::*;

use crate::Document;

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) inner: NodeRef,
}

impl Node {
    const NAME: &'static str = "Node";

    pub fn constructor<'s>(
        &self,
        scope: &mut HandleScope<'s, ()>,
    ) -> Option<Local<'s, FunctionTemplate>> {
        let template = FunctionTemplate::builder(
            // TODO
            |_: &mut HandleScope, _: FunctionCallbackArguments, _: ReturnValue| {},
        )
        .build(scope);

        template.set_class_name(String::new(scope, Self::NAME).unwrap());

        let instance = template.prototype_template(scope);
        let name = String::new(scope, "c").unwrap();
        let num = Number::new(scope, 10.0);
        instance.set(name.into(), num.into());

        let instance = template.instance_template(scope);
        let name = String::new(scope, "d").unwrap();
        let num = Number::new(scope, 10.0);
        instance.set(name.into(), num.into());

        Some(template)
    }
}

// /// Methods for JS Context.
// impl Node {
//     // FIXME: This is not the correct way to do this.
//     pub fn get_root_node(
//         this: &JsValue,
//         args: &[JsValue],
//         context: &mut Context,
//     ) -> JsResult<JsValue> {
//         match this
//             .as_object()
//             .unwrap()
//             .downcast_ref::<Document>()
//             .unwrap()
//             .as_node()
//             .as_raw()
//             .ancestors()
//             .last()
//         {
//             Some(node) => Ok(Node::new(node).js_object(context)?.into()),
//             None => Ok(JsValue::Null),
//         }
//     }
// }

/// Methods for Rust.
impl Node {
    pub fn new(inner: NodeRef) -> Self {
        Node { inner }
    }

    /// Get the parent node of this node.
    pub fn parent_node(&self) -> Option<NodeRef> {
        self.inner.parent()
    }
}
