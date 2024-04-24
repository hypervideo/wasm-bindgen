#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BrowserElementDownloadOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BrowserElementDownloadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    pub type BrowserElementDownloadOptions;
}
#[doc = "The trait to access properties on the `BrowserElementDownloadOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
pub trait BrowserElementDownloadOptionsGetters {
    #[doc = "Get the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    fn filename(&self) -> Option<&str>;
    #[doc = "Get the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    fn referrer(&self) -> Option<&str>;
}
impl BrowserElementDownloadOptionsGetters for BrowserElementDownloadOptions {
    fn filename(&self) -> Option<&str> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("filename"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn referrer(&self) -> Option<&str> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("referrer"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl BrowserElementDownloadOptions {
    #[doc = "Construct a new `BrowserElementDownloadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `filename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    pub fn filename(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("filename"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserElementDownloadOptions`*"]
    pub fn referrer(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("referrer"),
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
impl Default for BrowserElementDownloadOptions {
    fn default() -> Self {
        Self::new()
    }
}
