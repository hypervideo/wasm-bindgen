#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableWritablePair)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableWritablePair` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`*"]
    pub type ReadableWritablePair;
}
#[doc = "The trait to access properties on the `ReadableWritablePair` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`*"]
pub trait ReadableWritablePairGetters {
    #[cfg(feature = "ReadableStream")]
    #[doc = "Get the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`*"]
    fn readable(&self) -> &ReadableStream;
    #[cfg(feature = "WritableStream")]
    #[doc = "Get the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`, `WritableStream`*"]
    fn writable(&self) -> &WritableStream;
}
impl ReadableWritablePairGetters for ReadableWritablePair {
    #[cfg(feature = "ReadableStream")]
    fn readable(&self) -> &ReadableStream {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("readable"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "WritableStream")]
    fn writable(&self) -> &WritableStream {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("writable"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ReadableWritablePair {
    #[cfg(all(feature = "ReadableStream", feature = "WritableStream",))]
    #[doc = "Construct a new `ReadableWritablePair`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`, `WritableStream`*"]
    pub fn new(readable: &ReadableStream, writable: &WritableStream) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.readable(readable);
        ret.writable(writable);
        ret
    }
    #[cfg(feature = "ReadableStream")]
    #[doc = "Change the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`*"]
    pub fn readable(&mut self, val: &ReadableStream) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("readable"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "WritableStream")]
    #[doc = "Change the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`, `WritableStream`*"]
    pub fn writable(&mut self, val: &WritableStream) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("writable"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
