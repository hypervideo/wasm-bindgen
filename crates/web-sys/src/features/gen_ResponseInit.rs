#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ResponseInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ResponseInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub type ResponseInit;
}
#[doc = "The trait to access properties on the `ResponseInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
pub trait ResponseInitGetters {
    #[doc = "Get the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    fn headers(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    fn status(&self) -> u16;
    #[doc = "Get the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    fn status_text(&self) -> &str;
}
impl ResponseInitGetters for ResponseInit {
    fn headers(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("headers"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn status(&self) -> u16 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("status"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn status_text(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("statusText"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ResponseInit {
    #[doc = "Construct a new `ResponseInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `headers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn headers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("headers"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn status(&mut self, val: u16) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("status"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `statusText` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ResponseInit`*"]
    pub fn status_text(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("statusText"),
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
impl Default for ResponseInit {
    fn default() -> Self {
        Self::new()
    }
}
