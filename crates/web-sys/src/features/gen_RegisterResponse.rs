#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RegisterResponse)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegisterResponse` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub type RegisterResponse;
}
#[doc = "The trait to access properties on the `RegisterResponse` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
pub trait RegisterResponseGetters {
    #[doc = "Get the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    fn client_data(&self) -> &str;
    #[doc = "Get the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    fn error_code(&self) -> Option<u16>;
    #[doc = "Get the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    fn error_message(&self) -> Option<&str>;
    #[doc = "Get the `registrationData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    fn registration_data(&self) -> &str;
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    fn version(&self) -> &str;
}
impl RegisterResponseGetters for RegisterResponse {
    fn client_data(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("clientData"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn error_code(&self) -> Option<u16> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("errorCode"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn error_message(&self) -> Option<&str> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("errorMessage"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn registration_data(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("registrationData"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn version(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("version"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RegisterResponse {
    #[doc = "Construct a new `RegisterResponse`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `clientData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn client_data(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("clientData"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `errorCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn error_code(&mut self, val: Option<u16>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("errorCode"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `errorMessage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn error_message(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("errorMessage"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `registrationData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn registration_data(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("registrationData"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegisterResponse`*"]
    pub fn version(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("version"),
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
impl Default for RegisterResponse {
    fn default() -> Self {
        Self::new()
    }
}
