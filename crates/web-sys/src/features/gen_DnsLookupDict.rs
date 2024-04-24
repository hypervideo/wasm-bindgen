#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DNSLookupDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsLookupDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub type DnsLookupDict;
}
#[doc = "The trait to access properties on the `DnsLookupDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
pub trait DnsLookupDictGetters {
    #[doc = "Get the `address` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    fn address(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `answer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    fn answer(&self) -> bool;
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    fn error(&self) -> &str;
}
impl DnsLookupDictGetters for DnsLookupDict {
    fn address(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("address"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn answer(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("answer"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn error(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("error"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl DnsLookupDict {
    #[doc = "Construct a new `DnsLookupDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `address` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub fn address(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("address"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `answer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub fn answer(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("answer"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsLookupDict`*"]
    pub fn error(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("error"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for DnsLookupDict {
    fn default() -> Self {
        Self::new()
    }
}
