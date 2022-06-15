use js::prelude::*;

/// DOM [`Document`][document] Rust object.
///
/// [document]: https://developer.mozilla.org/docs/Web/API/document
#[derive(Debug, Clone, Trace, Finalize)]
pub struct Document {
    inner: JsObject,
}

impl Document {
    pub fn new(context: &mut Context) -> Self {
        Self {
            inner: super::Document::fake_new(context).unwrap(),
        }
    }

    pub fn say_hello(&self, context: &mut Context) {
        super::Document::say_hello(&self.inner.clone().into(), &[], context).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Document;
    use js::{property::Attribute, Context};

    #[test]
    fn dom_document() {
        let mut ctx = Context::default();
        crate::init(&mut ctx);
        let document = Document::new(&mut ctx);
        ctx.register_global_property("document", document.inner.clone(), Attribute::WRITABLE);
        ctx.eval("document.say_hello()");
        ctx.eval("let x = new Document(); x.say_hello()");
    }
}
