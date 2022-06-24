use js::prelude::*;

use crate::{wrapper, Document};

/// DOM [`Window`][window] Rust object.
///
/// [document]: https://developer.mozilla.org/en-US/docs/Web/API/Window
#[derive(Debug, Clone, Trace, Finalize)]
pub struct Window {
    inner: JsObject,
}

impl Window {
    pub fn new(context: &mut Context) -> Self {
        Self {
            inner: super::Window::new(context).unwrap(),
        }
    }

    /// Get `Document` rust wrapper object.
    pub fn document(&self, context: &mut Context) -> JsResult<wrapper::Document> {
        let value = self.inner.get("document", context)?;
        let object = value.as_object().unwrap();
        let inner = object.downcast_ref::<Document>().unwrap().clone();
        Ok(wrapper::Document { inner })
    }
}

#[cfg(test)]
mod tests {
    use super::Window;
    use js::{property::Attribute, Context};

    #[test]
    fn dom_document() {
        let mut ctx = Context::default();
        crate::init(&mut ctx);
        let window = Window::new(&mut ctx);
        ctx.register_global_property("window", window.inner.clone(), Attribute::WRITABLE);
        dbg!(ctx.eval("window.document").unwrap());
        let document = window.document(&mut ctx);
        // ctx.eval("let x = new Document(); x.say_hello()");
    }
}
