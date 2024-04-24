#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DisplayMediaStreamConstraints)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DisplayMediaStreamConstraints` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    pub type DisplayMediaStreamConstraints;
}
#[doc = "The trait to access properties on the `DisplayMediaStreamConstraints` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
pub trait DisplayMediaStreamConstraintsGetters {
    #[doc = "Get the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    fn audio(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    fn video(&self) -> &::wasm_bindgen::JsValue;
}
impl DisplayMediaStreamConstraintsGetters for DisplayMediaStreamConstraints {
    fn audio(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("audio"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn video(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("video"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl DisplayMediaStreamConstraints {
    #[doc = "Construct a new `DisplayMediaStreamConstraints`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `audio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    pub fn audio(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("audio"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `video` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DisplayMediaStreamConstraints`*"]
    pub fn video(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("video"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for DisplayMediaStreamConstraints {
    fn default() -> Self {
        Self::new()
    }
}
