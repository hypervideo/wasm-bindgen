#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebSocketDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebSocketDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    pub type WebSocketDict;
}
#[doc = "The trait to access properties on the `WebSocketDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
pub trait WebSocketDictGetters {
    #[doc = "Get the `websockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    fn websockets(&self) -> &::wasm_bindgen::JsValue;
}
impl WebSocketDictGetters for WebSocketDict {
    fn websockets(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("websockets"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl WebSocketDict {
    #[doc = "Construct a new `WebSocketDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `websockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebSocketDict`*"]
    pub fn websockets(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("websockets"),
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
impl Default for WebSocketDict {
    fn default() -> Self {
        Self::new()
    }
}
