#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BasePropertyIndexedKeyframe)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BasePropertyIndexedKeyframe` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    pub type BasePropertyIndexedKeyframe;
}
#[doc = "The trait to access properties on the `BasePropertyIndexedKeyframe` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
pub trait BasePropertyIndexedKeyframeGetters {
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    fn composite(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    fn easing(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    fn offset(&self) -> &::wasm_bindgen::JsValue;
}
impl BasePropertyIndexedKeyframeGetters for BasePropertyIndexedKeyframe {
    fn composite(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("composite"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn easing(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("easing"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn offset(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("offset"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl BasePropertyIndexedKeyframe {
    #[doc = "Construct a new `BasePropertyIndexedKeyframe`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    pub fn composite(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("composite"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    pub fn easing(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("easing"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BasePropertyIndexedKeyframe`*"]
    pub fn offset(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("offset"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for BasePropertyIndexedKeyframe {
    fn default() -> Self {
        Self::new()
    }
}
