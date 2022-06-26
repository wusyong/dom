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

    pub fn init<'s>(
        &self,
        scope: &mut HandleScope<'s, ()>,
        document: Local<'s, ObjectTemplate>,
        constructor: Local<'s, FunctionTemplate>,
    ) -> Option<Local<'s, FunctionTemplate>> {
        let template = FunctionTemplate::builder(
            |_: &mut HandleScope, _: FunctionCallbackArguments, _: ReturnValue| {},
        )
        .build(scope);

        template.set_class_name(String::new(scope, Self::NAME).unwrap());

        let instance = template.instance_template(scope);
        let name = String::new(scope, "document").unwrap();
        instance.set(name.into(), document.into());
        let name = String::new(scope, "Document").unwrap();
        instance.set(name.into(), constructor.into());

        Some(template)
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
        // Node template can get from Document itself
        let node_ft = document.as_node().template(scope).unwrap();
        let document_ft = document.template(scope, node_ft).unwrap();
        let document_it = document_ft.instance_template(scope);

        // Create Window
        let window = Window::new(document);
        let window_ft = window.init(scope, document_it, document_ft).unwrap();
        let window_it = window_ft.instance_template(scope);

        // Create Window Context
        let context = Context::new_from_template(scope, window_it);
        let scope = &mut ContextScope::new(scope, context);

        // Run Script
        let source = v8::String::new(scope, "this.document.m").unwrap();
        let script = v8::Script::compile(scope, source, None).unwrap();
        let r = script.run(scope).unwrap();

        let expected = Number::new(scope, 10.0);
        dbg!(r.strict_equals(expected.into()));
    }
}
