#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleProfileEvent)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleProfileEvent` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub type ConsoleProfileEvent;
}
#[doc = "The trait to access properties on the `ConsoleProfileEvent` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
pub trait ConsoleProfileEventGetters {
    #[doc = "Get the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    fn action(&self) -> &str;
    #[doc = "Get the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    fn arguments(&self) -> &::wasm_bindgen::JsValue;
}
impl ConsoleProfileEventGetters for ConsoleProfileEvent {
    fn action(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("action"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn arguments(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("arguments"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ConsoleProfileEvent {
    #[doc = "Construct a new `ConsoleProfileEvent`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `action` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub fn action(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("action"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `arguments` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleProfileEvent`*"]
    pub fn arguments(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("arguments"),
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
impl Default for ConsoleProfileEvent {
    fn default() -> Self {
        Self::new()
    }
}
