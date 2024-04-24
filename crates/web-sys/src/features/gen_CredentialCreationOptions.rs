#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CredentialCreationOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CredentialCreationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`*"]
    pub type CredentialCreationOptions;
}
#[doc = "The trait to access properties on the `CredentialCreationOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`*"]
pub trait CredentialCreationOptionsGetters {
    #[cfg(feature = "PublicKeyCredentialCreationOptions")]
    #[doc = "Get the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`, `PublicKeyCredentialCreationOptions`*"]
    fn public_key(&self) -> &PublicKeyCredentialCreationOptions;
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialCreationOptions`*"]
    fn signal(&self) -> &AbortSignal;
}
impl CredentialCreationOptionsGetters for CredentialCreationOptions {
    #[cfg(feature = "PublicKeyCredentialCreationOptions")]
    fn public_key(&self) -> &PublicKeyCredentialCreationOptions {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("publicKey"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "AbortSignal")]
    fn signal(&self) -> &AbortSignal {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("signal"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl CredentialCreationOptions {
    #[doc = "Construct a new `CredentialCreationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "PublicKeyCredentialCreationOptions")]
    #[doc = "Change the `publicKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CredentialCreationOptions`, `PublicKeyCredentialCreationOptions`*"]
    pub fn public_key(&mut self, val: &PublicKeyCredentialCreationOptions) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("publicKey"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `CredentialCreationOptions`*"]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("signal"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for CredentialCreationOptions {
    fn default() -> Self {
        Self::new()
    }
}
