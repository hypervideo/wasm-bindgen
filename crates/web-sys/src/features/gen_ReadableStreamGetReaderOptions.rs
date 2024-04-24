#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamGetReaderOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamGetReaderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`*"]
    pub type ReadableStreamGetReaderOptions;
}
#[doc = "The trait to access properties on the `ReadableStreamGetReaderOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`*"]
pub trait ReadableStreamGetReaderOptionsGetters {
    #[cfg(feature = "ReadableStreamReaderMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`, `ReadableStreamReaderMode`*"]
    fn mode(&self) -> ReadableStreamReaderMode;
}
impl ReadableStreamGetReaderOptionsGetters for ReadableStreamGetReaderOptions {
    #[cfg(feature = "ReadableStreamReaderMode")]
    fn mode(&self) -> ReadableStreamReaderMode {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("mode"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ReadableStreamGetReaderOptions {
    #[doc = "Construct a new `ReadableStreamGetReaderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ReadableStreamReaderMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamGetReaderOptions`, `ReadableStreamReaderMode`*"]
    pub fn mode(&mut self, val: ReadableStreamReaderMode) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("mode"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for ReadableStreamGetReaderOptions {
    fn default() -> Self {
        Self::new()
    }
}
