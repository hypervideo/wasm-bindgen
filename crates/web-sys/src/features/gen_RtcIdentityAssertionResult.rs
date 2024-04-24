#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIdentityAssertionResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityAssertionResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
    pub type RtcIdentityAssertionResult;
}
#[doc = "The trait to access properties on the `RtcIdentityAssertionResult` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
pub trait RtcIdentityAssertionResultGetters {
    #[doc = "Get the `assertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
    fn assertion(&self) -> &str;
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Get the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
    fn idp(&self) -> &RtcIdentityProviderDetails;
}
impl RtcIdentityAssertionResultGetters for RtcIdentityAssertionResult {
    fn assertion(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("assertion"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "RtcIdentityProviderDetails")]
    fn idp(&self) -> &RtcIdentityProviderDetails {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("idp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RtcIdentityAssertionResult {
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Construct a new `RtcIdentityAssertionResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
    pub fn new(assertion: &str, idp: &RtcIdentityProviderDetails) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.assertion(assertion);
        ret.idp(idp);
        ret
    }
    #[doc = "Change the `assertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`*"]
    pub fn assertion(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("assertion"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "RtcIdentityProviderDetails")]
    #[doc = "Change the `idp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityAssertionResult`, `RtcIdentityProviderDetails`*"]
    pub fn idp(&mut self, val: &RtcIdentityProviderDetails) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("idp"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
