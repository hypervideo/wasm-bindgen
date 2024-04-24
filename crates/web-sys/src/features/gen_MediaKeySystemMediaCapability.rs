#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaKeySystemMediaCapability)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySystemMediaCapability` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    pub type MediaKeySystemMediaCapability;
}
#[doc = "The trait to access properties on the `MediaKeySystemMediaCapability` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
pub trait MediaKeySystemMediaCapabilityGetters {
    #[doc = "Get the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    fn content_type(&self) -> &str;
    #[doc = "Get the `robustness` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    fn robustness(&self) -> &str;
}
impl MediaKeySystemMediaCapabilityGetters for MediaKeySystemMediaCapability {
    fn content_type(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("contentType"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn robustness(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("robustness"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl MediaKeySystemMediaCapability {
    #[doc = "Construct a new `MediaKeySystemMediaCapability`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `contentType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    pub fn content_type(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("contentType"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `robustness` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemMediaCapability`*"]
    pub fn robustness(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("robustness"),
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
impl Default for MediaKeySystemMediaCapability {
    fn default() -> Self {
        Self::new()
    }
}
