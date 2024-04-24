#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DisplayNameOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DisplayNameOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub type DisplayNameOptions;
}
#[doc = "The trait to access properties on the `DisplayNameOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
pub trait DisplayNameOptionsGetters {
    #[doc = "Get the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    fn keys(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    fn style(&self) -> &str;
}
impl DisplayNameOptionsGetters for DisplayNameOptions {
    fn keys(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("keys"));
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
impl DisplayNameOptions {
    #[doc = "Construct a new `DisplayNameOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
    pub fn keys(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("keys"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `style` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayNameOptions`*"]
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
impl Default for DisplayNameOptions {
    fn default() -> Self {
        Self::new()
    }
}
