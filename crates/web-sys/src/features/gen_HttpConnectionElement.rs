#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HttpConnectionElement)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HttpConnectionElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub type HttpConnectionElement;
}
#[doc = "The trait to access properties on the `HttpConnectionElement` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
pub trait HttpConnectionElementGetters {
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn active(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `halfOpens` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn half_opens(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn host(&self) -> &str;
    #[doc = "Get the `idle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn idle(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn port(&self) -> u32;
    #[doc = "Get the `spdy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn spdy(&self) -> bool;
    #[doc = "Get the `ssl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    fn ssl(&self) -> bool;
}
impl HttpConnectionElementGetters for HttpConnectionElement {
    fn active(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("active"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn half_opens(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("halfOpens"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn host(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("host"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn idle(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("idle"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn port(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("port"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn spdy(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("spdy"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ssl(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ssl"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl HttpConnectionElement {
    #[doc = "Construct a new `HttpConnectionElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn active(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("active"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `halfOpens` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn half_opens(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("halfOpens"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn host(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("host"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `idle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn idle(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("idle"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn port(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("port"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `spdy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn spdy(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("spdy"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `ssl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HttpConnectionElement`*"]
    pub fn ssl(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ssl"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for HttpConnectionElement {
    fn default() -> Self {
        Self::new()
    }
}
