#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DoubleRange)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DoubleRange` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    pub type DoubleRange;
    #[doc = "Get the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    #[wasm_bindgen(method, getter = "max")]
    pub fn get_max(this: &DoubleRange) -> Option<f64>;
    #[doc = "Change the `max` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    #[wasm_bindgen(method, setter = "max")]
    pub fn set_max(this: &DoubleRange, val: f64);
    #[doc = "Get the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    #[wasm_bindgen(method, getter = "min")]
    pub fn get_min(this: &DoubleRange) -> Option<f64>;
    #[doc = "Change the `min` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    #[wasm_bindgen(method, setter = "min")]
    pub fn set_min(this: &DoubleRange, val: f64);
}
impl DoubleRange {
    #[doc = "Construct a new `DoubleRange`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[deprecated = "Use `set_max()` instead."]
    pub fn max(&mut self, val: f64) -> &mut Self {
        self.set_max(val);
        self
    }
    #[deprecated = "Use `set_min()` instead."]
    pub fn min(&mut self, val: f64) -> &mut Self {
        self.set_min(val);
        self
    }
}
impl Default for DoubleRange {
    fn default() -> Self {
        Self::new()
    }
}
