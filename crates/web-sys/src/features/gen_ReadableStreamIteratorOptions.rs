#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ReadableStreamIteratorOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableStreamIteratorOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
    pub type ReadableStreamIteratorOptions;
}
#[doc = "The trait to access properties on the `ReadableStreamIteratorOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
pub trait ReadableStreamIteratorOptionsGetters {
    #[doc = "Get the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
    fn prevent_cancel(&self) -> bool;
}
impl ReadableStreamIteratorOptionsGetters for ReadableStreamIteratorOptions {
    fn prevent_cancel(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("preventCancel"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ReadableStreamIteratorOptions {
    #[doc = "Construct a new `ReadableStreamIteratorOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamIteratorOptions`*"]
    pub fn prevent_cancel(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("preventCancel"),
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
impl Default for ReadableStreamIteratorOptions {
    fn default() -> Self {
        Self::new()
    }
}
