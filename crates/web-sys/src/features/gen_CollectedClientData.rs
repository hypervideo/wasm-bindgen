#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CollectedClientData)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CollectedClientData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub type CollectedClientData;
}
#[doc = "The trait to access properties on the `CollectedClientData` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
pub trait CollectedClientDataGetters {
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    fn challenge(&self) -> &str;
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Get the `clientExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `CollectedClientData`*"]
    fn client_extensions(&self) -> &AuthenticationExtensionsClientInputs;
    #[doc = "Get the `hashAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    fn hash_algorithm(&self) -> &str;
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    fn origin(&self) -> &str;
    #[doc = "Get the `tokenBindingId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    fn token_binding_id(&self) -> &str;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    fn type_(&self) -> &str;
}
impl CollectedClientDataGetters for CollectedClientData {
    fn challenge(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("challenge"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    fn client_extensions(&self) -> &AuthenticationExtensionsClientInputs {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("clientExtensions"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn hash_algorithm(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("hashAlgorithm"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn origin(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("origin"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn token_binding_id(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("tokenBindingId"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn type_(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl CollectedClientData {
    #[doc = "Construct a new `CollectedClientData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn new(challenge: &str, hash_algorithm: &str, origin: &str, type_: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.challenge(challenge);
        ret.hash_algorithm(hash_algorithm);
        ret.origin(origin);
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn challenge(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("challenge"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "AuthenticationExtensionsClientInputs")]
    #[doc = "Change the `clientExtensions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`, `CollectedClientData`*"]
    pub fn client_extensions(&mut self, val: &AuthenticationExtensionsClientInputs) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("clientExtensions"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `hashAlgorithm` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn hash_algorithm(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("hashAlgorithm"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn origin(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("origin"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `tokenBindingId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn token_binding_id(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("tokenBindingId"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CollectedClientData`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
