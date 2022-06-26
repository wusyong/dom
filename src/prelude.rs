use crate::*;

// /// Trait representing a DOM object such as `Window`, `Document` or
// /// `Node`.
// ///
// /// This trait must be implemented for any DOM types in order to be used under
// /// Javascript Context.
// pub trait V8DOM: Sized {
//     /// Binding name of the DOM object inside the global object.
//     ///
//     /// E.g. If you want access the properties of a `Complex` built-in
//     /// with the name `Cplx` you must assign `"Cplx"` to this constant,
//     /// making any property inside it accessible from Javascript as `Cplx.prop`
//     const NAME: &'static str;
//
//     /// Property attribute flags of the DOM object.
//     /// Check [Attribute] for more information.
//     const ATTRIBUTE: Attribute = Attribute::WRITABLE
//         .union(Attribute::NON_ENUMERABLE)
//         .union(Attribute::CONFIGURABLE);
//
//     /// The constructor of the DOM object.
//     fn constructor(this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue>;
//
//     /// Initialization code for the DOM object.
//     /// This is where the methods, properties, static methods and the constructor
//     /// of a DOM object must be initialized to be accessible from Javascript.
//     ///
//     /// # Note
//     ///
//     /// A return value of `None` indicates that the value must not be added as
//     /// a global attribute for the global object.
//     fn init(context: &mut Context) -> Option<JsValue>;
//
//     /// Create a JavaScript object reflection to current instance.
//     ///
//     /// This object can be used in other object or register as context's global
//     /// property.
//     fn js_object(&self, context: &mut Context) -> JsResult<JsObject>;
// }

// /// Utility function that checks if a type implements `BuiltIn` before
// /// initializing it as a global built-in.
// #[inline]
// fn init_dom<D: DOM>(context: &mut Context) {
//     if let Some(value) = D::init(context) {
//         context.register_global_property(D::NAME, value, D::ATTRIBUTE);
//     }
// }

/// Registers DOM object constructors to JS Context.
/// And the return the `Window` DOM object.
#[inline]
pub fn init() {
    v8::V8::initialize_platform(v8::new_default_platform(0, false).make_shared());
    v8::V8::initialize();

    // macro_rules! globals {
    //     ($( $builtin:ty ),*) => {
    //         $(init_dom::<$builtin>(context)
    //         );*
    //     }
    // }
    //
    // // Order is important. Some types require previous type's prototype.
    // globals! {
    //     Node,
    //     Document,
    //     Window
    // };

    // let window = Window::new(false);
    // let object = window.js_object(context).unwrap();
    // context.register_global_property("window", object, Attribute::WRITABLE);
    //
    // Ok(window)
}
