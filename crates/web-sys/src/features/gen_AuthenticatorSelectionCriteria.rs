#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticatorSelectionCriteria)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticatorSelectionCriteria` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    pub type AuthenticatorSelectionCriteria;
}
#[doc = "The trait to access properties on the `AuthenticatorSelectionCriteria` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
pub trait AuthenticatorSelectionCriteriaGetters {
    #[cfg(feature = "AuthenticatorAttachment")]
    #[doc = "Get the `authenticatorAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttachment`, `AuthenticatorSelectionCriteria`*"]
    fn authenticator_attachment(&self) -> AuthenticatorAttachment;
    #[doc = "Get the `requireResidentKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    fn require_resident_key(&self) -> bool;
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Get the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `UserVerificationRequirement`*"]
    fn user_verification(&self) -> UserVerificationRequirement;
}
impl AuthenticatorSelectionCriteriaGetters for AuthenticatorSelectionCriteria {
    #[cfg(feature = "AuthenticatorAttachment")]
    fn authenticator_attachment(&self) -> AuthenticatorAttachment {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("authenticatorAttachment"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn require_resident_key(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("requireResidentKey"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "UserVerificationRequirement")]
    fn user_verification(&self) -> UserVerificationRequirement {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("userVerification"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl AuthenticatorSelectionCriteria {
    #[doc = "Construct a new `AuthenticatorSelectionCriteria`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "AuthenticatorAttachment")]
    #[doc = "Change the `authenticatorAttachment` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttachment`, `AuthenticatorSelectionCriteria`*"]
    pub fn authenticator_attachment(&mut self, val: AuthenticatorAttachment) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("authenticatorAttachment"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `requireResidentKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`*"]
    pub fn require_resident_key(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("requireResidentKey"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "UserVerificationRequirement")]
    #[doc = "Change the `userVerification` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticatorSelectionCriteria`, `UserVerificationRequirement`*"]
    pub fn user_verification(&mut self, val: UserVerificationRequirement) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("userVerification"),
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
impl Default for AuthenticatorSelectionCriteria {
    fn default() -> Self {
        Self::new()
    }
}
