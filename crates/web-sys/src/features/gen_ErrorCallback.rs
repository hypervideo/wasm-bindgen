#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ErrorCallback)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ErrorCallback` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`*"]
    pub type ErrorCallback;
}
#[doc = "The trait to access properties on the `ErrorCallback` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ErrorCallback`*"]
pub trait ErrorCallbackGetters {
    #[doc = "Get the `handleEvent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`*"]
    fn handle_event(&self) -> &::js_sys::Function;
}
impl ErrorCallbackGetters for ErrorCallback {
    fn handle_event(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("handleEvent"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ErrorCallback {
    #[doc = "Construct a new `ErrorCallback`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `handleEvent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ErrorCallback`*"]
    pub fn handle_event(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("handleEvent"),
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
impl Default for ErrorCallback {
    fn default() -> Self {
        Self::new()
    }
}
