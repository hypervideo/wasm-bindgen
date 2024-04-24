#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConstantSourceOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConstantSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
    pub type ConstantSourceOptions;
    #[wasm_bindgen(method, getter = "offset")]
    fn offset_shim(this: &ConstantSourceOptions) -> f32;
    #[wasm_bindgen(method, setter = "offset")]
    fn set_offset_shim(this: &ConstantSourceOptions, val: f32);
}
#[doc = "The trait to access properties on the `ConstantSourceOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
pub trait ConstantSourceOptionsGetters {
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
    fn offset(&self) -> f32;
}
impl ConstantSourceOptionsGetters for ConstantSourceOptions {
    fn offset(&self) -> f32 {
        self.offset_shim()
    }
}
impl ConstantSourceOptions {
    #[doc = "Construct a new `ConstantSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConstantSourceOptions`*"]
    pub fn offset(&mut self, val: f32) -> &mut Self {
        self.set_offset_shim(val);
        self
    }
}
impl Default for ConstantSourceOptions {
    fn default() -> Self {
        Self::new()
    }
}
