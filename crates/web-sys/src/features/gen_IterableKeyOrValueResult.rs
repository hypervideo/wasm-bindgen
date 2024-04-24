#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IterableKeyOrValueResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IterableKeyOrValueResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    pub type IterableKeyOrValueResult;
}
#[doc = "The trait to access properties on the `IterableKeyOrValueResult` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
pub trait IterableKeyOrValueResultGetters {
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    fn done(&self) -> bool;
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    fn value(&self) -> &::wasm_bindgen::JsValue;
}
impl IterableKeyOrValueResultGetters for IterableKeyOrValueResult {
    fn done(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("done"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn value(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("value"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl IterableKeyOrValueResult {
    #[doc = "Construct a new `IterableKeyOrValueResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    pub fn done(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("done"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IterableKeyOrValueResult`*"]
    pub fn value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
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
impl Default for IterableKeyOrValueResult {
    fn default() -> Self {
        Self::new()
    }
}
