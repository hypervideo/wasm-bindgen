#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PushSubscriptionJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    pub type PushSubscriptionJson;
}
#[doc = "The trait to access properties on the `PushSubscriptionJson` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
pub trait PushSubscriptionJsonGetters {
    #[doc = "Get the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    fn endpoint(&self) -> &str;
    #[cfg(feature = "PushSubscriptionKeys")]
    #[doc = "Get the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`, `PushSubscriptionKeys`*"]
    fn keys(&self) -> &PushSubscriptionKeys;
}
impl PushSubscriptionJsonGetters for PushSubscriptionJson {
    fn endpoint(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("endpoint"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "PushSubscriptionKeys")]
    fn keys(&self) -> &PushSubscriptionKeys {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("keys"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl PushSubscriptionJson {
    #[doc = "Construct a new `PushSubscriptionJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`*"]
    pub fn endpoint(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("endpoint"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "PushSubscriptionKeys")]
    #[doc = "Change the `keys` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionJson`, `PushSubscriptionKeys`*"]
    pub fn keys(&mut self, val: &PushSubscriptionKeys) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("keys"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for PushSubscriptionJson {
    fn default() -> Self {
        Self::new()
    }
}
