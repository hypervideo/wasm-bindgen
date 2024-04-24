#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AesGcmParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AesGcmParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub type AesGcmParams;
}
#[doc = "The trait to access properties on the `AesGcmParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
pub trait AesGcmParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    fn additional_data(&self) -> &::js_sys::Object;
    #[doc = "Get the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    fn iv(&self) -> &::js_sys::Object;
    #[doc = "Get the `tagLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    fn tag_length(&self) -> u8;
}
impl AesGcmParamsGetters for AesGcmParams {
    fn name(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("name"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn additional_data(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("additionalData"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn iv(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("iv"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn tag_length(&self) -> u8 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("tagLength"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl AesGcmParams {
    #[doc = "Construct a new `AesGcmParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn new(name: &str, iv: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.iv(iv);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("name"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `additionalData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn additional_data(&mut self, val: &::js_sys::Object) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("additionalData"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `iv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn iv(&mut self, val: &::js_sys::Object) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("iv"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `tagLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AesGcmParams`*"]
    pub fn tag_length(&mut self, val: u8) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("tagLength"),
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
