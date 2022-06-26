use kuchiki::NodeRef;
use v8::*;

use crate::Node;

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
#[derive(Debug, Clone)]
pub struct Document {
    node: Node,
}

impl Document {
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
    pub fn template<'s>(
        &self,
        scope: &mut HandleScope<'s, ()>,
        node: Local<'s, FunctionTemplate>,
    ) -> Option<Local<'s, FunctionTemplate>> {
        let template = FunctionTemplate::builder(
            // TODO
            |_: &mut HandleScope, _: FunctionCallbackArguments, _: ReturnValue| {
                println!("Hello");
            },
        )
        .build(scope);

        template.set_class_name(String::new(scope, Self::NAME).unwrap());
        template.inherit(node);

        let instance = template.prototype_template(scope);
        let name = String::new(scope, "a").unwrap();
        let num = Number::new(scope, 10.0);
        instance.set(name.into(), num.into());

        let instance = template.instance_template(scope);
        let name = String::new(scope, "b").unwrap();
        let num = Number::new(scope, 10.0);
        instance.set(name.into(), num.into());

        Some(template)
    }
}

impl Document {
    pub fn new(root: NodeRef) -> Self {
        let node = Node::new(root);
        Document { node }
    }

    pub fn as_node(&self) -> &Node {
        &self.node
    }
}
