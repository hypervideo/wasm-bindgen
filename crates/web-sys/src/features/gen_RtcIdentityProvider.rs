#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIdentityProvider)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIdentityProvider` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    pub type RtcIdentityProvider;
}
#[doc = "The trait to access properties on the `RtcIdentityProvider` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
pub trait RtcIdentityProviderGetters {
    #[doc = "Get the `generateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    fn generate_assertion(&self) -> &::js_sys::Function;
    #[doc = "Get the `validateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    fn validate_assertion(&self) -> &::js_sys::Function;
}
impl RtcIdentityProviderGetters for RtcIdentityProvider {
    fn generate_assertion(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("generateAssertion"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn validate_assertion(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("validateAssertion"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RtcIdentityProvider {
    #[doc = "Construct a new `RtcIdentityProvider`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    pub fn new(
        generate_assertion: &::js_sys::Function,
        validate_assertion: &::js_sys::Function,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.generate_assertion(generate_assertion);
        ret.validate_assertion(validate_assertion);
        ret
    }
    #[doc = "Change the `generateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    pub fn generate_assertion(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("generateAssertion"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `validateAssertion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIdentityProvider`*"]
    pub fn validate_assertion(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("validateAssertion"),
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
