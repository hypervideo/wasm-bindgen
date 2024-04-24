#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FilePropertyBag)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FilePropertyBag` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    pub type FilePropertyBag;
    #[wasm_bindgen(method, getter = "lastModified")]
    fn last_modified_shim(this: &FilePropertyBag) -> f64;
    #[wasm_bindgen(method, setter = "lastModified")]
    fn set_last_modified_shim(this: &FilePropertyBag, val: f64);
    #[wasm_bindgen(method, getter = "type")]
    fn type__shim(this: &FilePropertyBag) -> String;
    #[wasm_bindgen(method, setter = "type")]
    fn set_type__shim(this: &FilePropertyBag, val: &str);
}
#[doc = "The trait to access properties on the `FilePropertyBag` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
pub trait FilePropertyBagGetters {
    #[doc = "Get the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    fn last_modified(&self) -> f64;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    fn type_(&self) -> String;
}
impl FilePropertyBagGetters for FilePropertyBag {
    fn last_modified(&self) -> f64 {
        self.last_modified_shim()
    }
    fn type_(&self) -> String {
        self.type__shim()
    }
}
impl FilePropertyBag {
    #[doc = "Construct a new `FilePropertyBag`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    pub fn last_modified(&mut self, val: f64) -> &mut Self {
        self.set_last_modified_shim(val);
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FilePropertyBag`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        self.set_type__shim(val);
        self
    }
}
impl Default for FilePropertyBag {
    fn default() -> Self {
        Self::new()
    }
}
