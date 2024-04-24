#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EcdhKeyDeriveParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EcdhKeyDeriveParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"]
    pub type EcdhKeyDeriveParams;
}
#[doc = "The trait to access properties on the `EcdhKeyDeriveParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"]
pub trait EcdhKeyDeriveParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"]
    fn name(&self) -> &str;
    #[cfg(feature = "CryptoKey")]
    #[doc = "Get the `public` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `EcdhKeyDeriveParams`*"]
    fn public(&self) -> &CryptoKey;
}
impl EcdhKeyDeriveParamsGetters for EcdhKeyDeriveParams {
    fn name(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("name"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "CryptoKey")]
    fn public(&self) -> &CryptoKey {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("public"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl EcdhKeyDeriveParams {
    #[cfg(feature = "CryptoKey")]
    #[doc = "Construct a new `EcdhKeyDeriveParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `EcdhKeyDeriveParams`*"]
    pub fn new(name: &str, public: &CryptoKey) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.public(public);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcdhKeyDeriveParams`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("name"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "CryptoKey")]
    #[doc = "Change the `public` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CryptoKey`, `EcdhKeyDeriveParams`*"]
    pub fn public(&mut self, val: &CryptoKey) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("public"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
