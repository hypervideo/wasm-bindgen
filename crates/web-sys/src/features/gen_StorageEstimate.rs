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
    #[wasm_bindgen(method, getter = "quota")]
    fn quota_shim(this: &StorageEstimate) -> f64;
    #[wasm_bindgen(method, setter = "quota")]
    fn set_quota_shim(this: &StorageEstimate, val: f64);
    #[wasm_bindgen(method, getter = "usage")]
    fn usage_shim(this: &StorageEstimate) -> f64;
    #[wasm_bindgen(method, setter = "usage")]
    fn set_usage_shim(this: &StorageEstimate, val: f64);
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
        self.quota_shim()
    }
    fn usage(&self) -> f64 {
        self.usage_shim()
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
        self.set_quota_shim(val);
        self
    }
    #[doc = "Change the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StorageEstimate`*"]
    pub fn usage(&mut self, val: f64) -> &mut Self {
        self.set_usage_shim(val);
        self
    }
}
impl Default for StorageEstimate {
    fn default() -> Self {
        Self::new()
    }
}
