#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstrainDOMStringParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstrainDomStringParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    pub type ConstrainDomStringParameters;
}
#[doc = "The trait to access properties on the `ConstrainDomStringParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
pub trait ConstrainDomStringParametersGetters {
    #[doc = "Get the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    fn exact(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    fn ideal(&self) -> &::wasm_bindgen::JsValue;
}
impl ConstrainDomStringParametersGetters for ConstrainDomStringParameters {
    fn exact(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("exact"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ideal(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ideal"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ConstrainDomStringParameters {
    #[doc = "Construct a new `ConstrainDomStringParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `exact` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    pub fn exact(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("exact"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `ideal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstrainDomStringParameters`*"]
    pub fn ideal(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ideal"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for ConstrainDomStringParameters {
    fn default() -> Self {
        Self::new()
    }
}
