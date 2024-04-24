#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DisplayNameResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DisplayNameResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    pub type DisplayNameResult;
}
#[doc = "The trait to access properties on the `DisplayNameResult` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
pub trait DisplayNameResultGetters {
    #[doc = "Get the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    fn locale(&self) -> &str;
    #[doc = "Get the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    fn style(&self) -> &str;
}
impl DisplayNameResultGetters for DisplayNameResult {
    fn locale(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("locale"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn style(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("style"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl DisplayNameResult {
    #[doc = "Construct a new `DisplayNameResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    pub fn locale(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("locale"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameResult`*"]
    pub fn style(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("style"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for DisplayNameResult {
    fn default() -> Self {
        Self::new()
    }
}
