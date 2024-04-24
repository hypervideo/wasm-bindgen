#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TextDecodeOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TextDecodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    pub type TextDecodeOptions;
}
#[doc = "The trait to access properties on the `TextDecodeOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
pub trait TextDecodeOptionsGetters {
    #[doc = "Get the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    fn stream(&self) -> bool;
}
impl TextDecodeOptionsGetters for TextDecodeOptions {
    fn stream(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("stream"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl TextDecodeOptions {
    #[doc = "Construct a new `TextDecodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `stream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TextDecodeOptions`*"]
    pub fn stream(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("stream"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for TextDecodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
