#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BasicCardRequest)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BasicCardRequest` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    pub type BasicCardRequest;
}
#[doc = "The trait to access properties on the `BasicCardRequest` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
pub trait BasicCardRequestGetters {
    #[doc = "Get the `supportedNetworks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    fn supported_networks(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `supportedTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    fn supported_types(&self) -> &::wasm_bindgen::JsValue;
}
impl BasicCardRequestGetters for BasicCardRequest {
    fn supported_networks(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("supportedNetworks"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn supported_types(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("supportedTypes"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl BasicCardRequest {
    #[doc = "Construct a new `BasicCardRequest`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `supportedNetworks` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    pub fn supported_networks(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("supportedNetworks"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `supportedTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasicCardRequest`*"]
    pub fn supported_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("supportedTypes"),
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
impl Default for BasicCardRequest {
    fn default() -> Self {
        Self::new()
    }
}
