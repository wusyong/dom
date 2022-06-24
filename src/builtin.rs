use js::{property::Attribute, Context, JsValue};

use crate::*;

/// Trait representing a global built-in object such as `Math`, `Object` or
/// `String`.
///
/// This trait must be implemented for any global built-in accessible from
/// Javascript.
pub(crate) trait BuiltIn {
    /// Binding name of the built-in inside the global object.
    ///
    /// E.g. If you want access the properties of a `Complex` built-in
    /// with the name `Cplx` you must assign `"Cplx"` to this constant,
    /// making any property inside it accessible from Javascript as `Cplx.prop`
    const NAME: &'static str;

    /// Property attribute flags of the built-in.
    /// Check [Attribute] for more information.
    const ATTRIBUTE: Attribute = Attribute::WRITABLE
        .union(Attribute::NON_ENUMERABLE)
        .union(Attribute::CONFIGURABLE);

    /// Initialization code for the built-in.
    /// This is where the methods, properties, static methods and the constructor
    /// of a built-in must be initialized to be accessible from Javascript.
    ///
    /// # Note
    ///
    /// A return value of `None` indicates that the value must not be added as
    /// a global attribute for the global object.
    fn init(context: &mut Context) -> Option<JsValue>;
}

/// Utility function that checks if a type implements `BuiltIn` before
/// initializing it as a global built-in.
#[inline]
fn init_builtin<B: BuiltIn>(context: &mut Context) {
    if let Some(value) = B::init(context) {
        context.register_global_property(B::NAME, value, B::ATTRIBUTE);
    }
}

/// Initializes built-in objects and functions
#[inline]
pub fn init(context: &mut Context) {
    macro_rules! globals {
        ($( $builtin:ty ),*) => {
            $(init_builtin::<$builtin>(context)
            );*
        }
    }

    globals! {
        Document,
        Window
    };
}
