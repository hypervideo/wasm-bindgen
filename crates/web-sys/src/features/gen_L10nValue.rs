#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = L10nValue)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `L10nValue` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    pub type L10nValue;
}
#[doc = "The trait to access properties on the `L10nValue` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
pub trait L10nValueGetters {
    #[doc = "Get the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    fn attributes(&self) -> Option<&::wasm_bindgen::JsValue>;
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    fn value(&self) -> Option<&str>;
}
impl L10nValueGetters for L10nValue {
    fn attributes(&self) -> Option<&::wasm_bindgen::JsValue> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("attributes"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn value(&self) -> Option<&str> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("value"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl L10nValue {
    #[doc = "Construct a new `L10nValue`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    pub fn attributes(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("attributes"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `L10nValue`*"]
    pub fn value(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("value"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for L10nValue {
    fn default() -> Self {
        Self::new()
    }
}
