use v8::*;

use crate::Document;

/// The `Window` DOM object implementation.
///
/// The `Window` interface represents a window containing a DOM document; the document property points to the DOM document loaded in that window.
///
/// More information:
///  - [DOM reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://html.spec.whatwg.org/multipage/window-object.html#the-window-object
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Window
#[derive(Debug, Clone)]
pub struct Window {
    document: Document,
}

impl Window {
    const NAME: &'static str = "Window";

    pub fn constructor<'s>(
        &self,
        scope: &mut HandleScope<'s, ()>,
    ) -> Option<Local<'s, ObjectTemplate>> {
        let template = FunctionTemplate::builder(
            |_: &mut HandleScope, _: FunctionCallbackArguments, _: ReturnValue| {},
        )
        .build(scope);

        template.set_class_name(String::new(scope, Self::NAME)?);
        let instance_t = template.instance_template(scope);
        Some(instance_t)
    }

    pub fn init_global<'s>(&self, scope: &mut HandleScope<'s>) -> Option<Local<'s, Function>> {
        // Get global property which should be `window` itself
        let global = scope.get_current_context().global(scope);
        let document = &self.document;
        let node = document.as_node();

        // Set the root of DOM tree to a scope slot
        scope.set_slot(node.inner.clone());

        // Set `Node` function
        // Node template can get from Document itself
        let node_ft = node.constructor(scope)?;
        let node_f = node_ft.get_function(scope)?;
        let name = String::new(scope, "Node")?;
        global.set(scope, name.into(), node_f.into());

        // Set `Document` function
        let document_ft = document.constructor(scope, node_ft)?;
        let document_f = document_ft.get_function(scope)?;
        let name = String::new(scope, "Document")?;
        global.set(scope, name.into(), document_f.into());

        // Set `document` instance
        let document_i = document_f.new_instance(scope, &[])?;
        let name = String::new(scope, "document")?;
        global.set(scope, name.into(), document_i.into());
        Some(document_f)
    }
}

impl Window {
    pub fn new(document: Document) -> Self {
        Window { document }
    }
}

#[cfg(test)]
mod tests {
    use kuchiki::traits::TendrilSink;
    use v8::*;

    use crate::{Document, Window};

    #[test]
    fn dom_reflection() {
        crate::init();
        // Create Isolate
        let isolate = &mut Isolate::new(Default::default());
        let scope = &mut HandleScope::new(isolate);

        // Create Document
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
        let root = kuchiki::parse_html().one(html);
        let document = Document::new(root);

        // Create Window
        let window = Window::new(document);
        let templ = window.constructor(scope).unwrap();

        // Create Window Context
        let context = Context::new_from_template(scope, templ);
        let scope = &mut ContextScope::new(scope, context);
        window.init_global(scope);

        // Run Script
        let source = v8::String::new(scope, "let x = new Document(); x.bb").unwrap();
        let script = v8::Script::compile(scope, source, None).unwrap();
        let r = script.run(scope).unwrap();

        let expected = Number::new(scope, 10.0);
        dbg!(r.strict_equals(expected.into()));
    }
}
