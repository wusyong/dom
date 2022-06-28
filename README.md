# DOM

This is an experimimental crate for building DOM objects in pure Rust. This crate aims to
create DOM types that can not only reflect to both Rust and Javascript, but also easy to work with.

The basic structure is each DOM object belongs to a directory as a module. They define their
own and related types there. Each type must implement [Trace], [Finalize], and [DOM] traits
in order to be used in JavaScript [Context].

An usage example would look like this:


## How to create DOM objects in JavaScript Context?

We need to call [init] before creating any DOM object. This call initialize each type's
constructor to the context.

It also returns the [Window] instance for now which can be used in Rust world. (This will be breaking change.)

## How to create DOM objects in Rust and reflect to JavaScript Context?

When an instance of a DOM type is created in Rust, it's just a normal rust instance. To work
with a JavaScript context, call [DOM::js_object] to create a [JsObject] which is a JavaScript
reflection of the Rust instance. It can then be used in other [JsObject] or register as
context's global property.

[Context]: js::Context
[JsObject]: js::object::JsObject

