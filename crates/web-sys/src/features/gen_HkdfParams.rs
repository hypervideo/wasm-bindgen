#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HkdfParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HkdfParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub type HkdfParams;
}
#[doc = "The trait to access properties on the `HkdfParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
pub trait HkdfParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    fn hash(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    fn info(&self) -> &::js_sys::Object;
    #[doc = "Get the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    fn salt(&self) -> &::js_sys::Object;
}
impl HkdfParamsGetters for HkdfParams {
    fn name(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("name"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn hash(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("hash"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn info(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("info"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn salt(&self) -> &::js_sys::Object {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("salt"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl HkdfParams {
    #[doc = "Construct a new `HkdfParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn new(
        name: &str,
        hash: &::wasm_bindgen::JsValue,
        info: &::js_sys::Object,
        salt: &::js_sys::Object,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.hash(hash);
        ret.info(info);
        ret.salt(salt);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
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
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
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
    #[doc = "Change the `info` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn info(&mut self, val: &::js_sys::Object) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("info"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `salt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HkdfParams`*"]
    pub fn salt(&mut self, val: &::js_sys::Object) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("salt"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
