#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DNSCacheDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsCacheDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    pub type DnsCacheDict;
}
#[doc = "The trait to access properties on the `DnsCacheDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
pub trait DnsCacheDictGetters {
    #[doc = "Get the `entries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    fn entries(&self) -> &::wasm_bindgen::JsValue;
}
impl DnsCacheDictGetters for DnsCacheDict {
    fn entries(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("entries"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl DnsCacheDict {
    #[doc = "Construct a new `DnsCacheDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `entries` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheDict`*"]
    pub fn entries(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("entries"),
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
impl Default for DnsCacheDict {
    fn default() -> Self {
        Self::new()
    }
}
