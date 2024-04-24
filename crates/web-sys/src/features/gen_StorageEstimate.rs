#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = StorageEstimate)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StorageEstimate` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
    pub type StorageEstimate;
}
#[doc = "The trait to access properties on the `StorageEstimate` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
pub trait StorageEstimateGetters {
    #[doc = "Get the `quota` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
    fn quota(&self) -> f64;
    #[doc = "Get the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
    fn usage(&self) -> f64;
}
impl StorageEstimateGetters for StorageEstimate {
    fn quota(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("quota"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn usage(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("usage"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl StorageEstimate {
    #[doc = "Construct a new `StorageEstimate`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `quota` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
    pub fn quota(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("quota"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
    pub fn usage(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("usage"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for StorageEstimate {
    fn default() -> Self {
        Self::new()
    }
}
