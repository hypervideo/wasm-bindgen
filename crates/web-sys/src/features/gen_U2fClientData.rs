#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = U2FClientData)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `U2fClientData` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    pub type U2fClientData;
}
#[doc = "The trait to access properties on the `U2fClientData` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
pub trait U2fClientDataGetters {
    #[doc = "Get the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    fn challenge(&self) -> &str;
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    fn origin(&self) -> &str;
    #[doc = "Get the `typ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    fn typ(&self) -> &str;
}
impl U2fClientDataGetters for U2fClientData {
    fn challenge(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("challenge"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn origin(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("origin"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn typ(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("typ"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl U2fClientData {
    #[doc = "Construct a new `U2fClientData`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `challenge` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
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
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
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
    #[doc = "Change the `typ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2fClientData`*"]
    pub fn typ(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("typ"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for U2fClientData {
    fn default() -> Self {
        Self::new()
    }
}
