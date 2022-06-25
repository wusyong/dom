use gc::Trace;
use js::{object::ConstructorBuilder, prelude::*};
use kuchiki::traits::TendrilSink;
use tap::{Conv, Pipe};

use crate::{Document, DOM};

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
#[derive(Debug, Clone, Trace, Finalize)]
pub struct Window {
    document: Document,
}

impl DOM for Window {
    const NAME: &'static str = "Window";

    fn constructor(_: &JsValue, _: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        // `Window` has no constructor.
        context.throw_type_error("Illegal constructor.")
    }

    fn init(context: &mut Context) -> Option<JsValue> {
        ConstructorBuilder::new(context, Self::constructor)
            .name(Self::NAME)
            .constructor(false)
            .method(Self::say_hello, "say_hello", 0)
            .build()
            .conv::<JsValue>()
            .pipe(Some)
    }

    fn js_object(&self, context: &mut Context) -> JsResult<JsObject> {
        let document = self.document.js_object(context)?;

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

        let window = JsObject::empty();
        window.set_prototype(Some(prototype));
        window.set("document", document, false, context)?;
        Ok(window)
    }
}

impl Window {
    pub fn new(_todo: bool) -> Self {
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

        Window {
            document: Document::new(node),
        }
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn say_hello(_: &JsValue, _: &[JsValue], _context: &mut Context) -> JsResult<JsValue> {
        println!("Hello Window");
        Ok(JsValue::Undefined)
    }
}

#[cfg(test)]
mod tests {
    use js::Context;

    #[test]
    fn dom_reflection() {
        let context = &mut Context::default();
        let window = crate::init(context).unwrap();
        context.eval("window.document").unwrap();
        context.eval("window.say_hello();").unwrap();
        context.eval("window.document.getRootNode();").unwrap();
        let _document = window.document();
    }
}
