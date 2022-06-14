#[macro_use]
extern crate gc;

use js::{object::ObjectData, prelude::*, property::Attribute};
use kuchiki::traits::TendrilSink;

mod document;
pub use document::*;

pub struct Document {
    object: JsObject,
}

impl Document {
    pub fn new(ctx: &mut Context) -> Self {
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
        let document = document::Document { node };
        let object =
            JsObject::from_proto_and_data(None, ObjectData::native_object(Box::new(document)));
        object.set("hello", "Hello World", false, ctx).unwrap();

        ctx.register_global_property("document", object.clone(), Attribute::WRITABLE);
        Document { object }
    }

    pub fn say_hello(&self) {
        println!("{:?}", self.object);
    }
}

#[cfg(test)]
mod tests {
    use crate::Document;
    use js::Context;

    #[test]
    fn dom_document() {
        let mut ctx = Context::default();
        let document = Document::new(&mut ctx);
        ctx.eval("document.hello = 'Hello from JavaScript!';")
            .unwrap();
        document.say_hello();
    }
}
