#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DnsCacheEntry)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DnsCacheEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub type DnsCacheEntry;
}
#[doc = "The trait to access properties on the `DnsCacheEntry` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
pub trait DnsCacheEntryGetters {
    #[doc = "Get the `expiration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    fn expiration(&self) -> f64;
    #[doc = "Get the `family` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    fn family(&self) -> &str;
    #[doc = "Get the `hostaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    fn hostaddr(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    fn hostname(&self) -> &str;
    #[doc = "Get the `trr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    fn trr(&self) -> bool;
}
impl DnsCacheEntryGetters for DnsCacheEntry {
    fn expiration(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("expiration"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn family(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("family"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn hostaddr(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("hostaddr"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn hostname(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("hostname"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn trr(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("trr"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl DnsCacheEntry {
    #[doc = "Construct a new `DnsCacheEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `expiration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn expiration(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("expiration"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `family` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn family(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("family"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `hostaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn hostaddr(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("hostaddr"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `hostname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn hostname(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("hostname"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `trr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DnsCacheEntry`*"]
    pub fn trr(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("trr"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for DnsCacheEntry {
    fn default() -> Self {
        Self::new()
    }
}
