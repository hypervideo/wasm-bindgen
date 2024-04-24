#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsClientOutputs)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsClientOutputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    pub type AuthenticationExtensionsClientOutputs;
}
#[doc = "The trait to access properties on the `AuthenticationExtensionsClientOutputs` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
pub trait AuthenticationExtensionsClientOutputsGetters {
    #[doc = "Get the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    fn appid(&self) -> bool;
}
impl AuthenticationExtensionsClientOutputsGetters for AuthenticationExtensionsClientOutputs {
    fn appid(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("appid"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl AuthenticationExtensionsClientOutputs {
    #[doc = "Construct a new `AuthenticationExtensionsClientOutputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientOutputs`*"]
    pub fn appid(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("appid"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for AuthenticationExtensionsClientOutputs {
    fn default() -> Self {
        Self::new()
    }
}
