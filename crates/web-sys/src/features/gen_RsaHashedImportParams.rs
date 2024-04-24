#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RsaHashedImportParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaHashedImportParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub type RsaHashedImportParams;
}
#[doc = "The trait to access properties on the `RsaHashedImportParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
pub trait RsaHashedImportParamsGetters {
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    fn hash(&self) -> &::wasm_bindgen::JsValue;
}
impl RsaHashedImportParamsGetters for RsaHashedImportParams {
    fn hash(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("hash"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RsaHashedImportParams {
    #[doc = "Construct a new `RsaHashedImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub fn new(hash: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.hash(hash);
        ret
    }
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaHashedImportParams`*"]
    pub fn hash(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("hash"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
