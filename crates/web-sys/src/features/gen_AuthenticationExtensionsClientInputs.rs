#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AuthenticationExtensionsClientInputs)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AuthenticationExtensionsClientInputs` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub type AuthenticationExtensionsClientInputs;
}
#[doc = "The trait to access properties on the `AuthenticationExtensionsClientInputs` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
pub trait AuthenticationExtensionsClientInputsGetters {
    #[doc = "Get the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    fn appid(&self) -> &str;
}
impl AuthenticationExtensionsClientInputsGetters for AuthenticationExtensionsClientInputs {
    fn appid(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("appid"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl AuthenticationExtensionsClientInputs {
    #[doc = "Construct a new `AuthenticationExtensionsClientInputs`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `appid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AuthenticationExtensionsClientInputs`*"]
    pub fn appid(&mut self, val: &str) -> &mut Self {
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
impl Default for AuthenticationExtensionsClientInputs {
    fn default() -> Self {
        Self::new()
    }
}
