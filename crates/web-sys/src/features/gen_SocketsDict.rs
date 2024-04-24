#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SocketsDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketsDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub type SocketsDict;
}
#[doc = "The trait to access properties on the `SocketsDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
pub trait SocketsDictGetters {
    #[doc = "Get the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    fn received(&self) -> f64;
    #[doc = "Get the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    fn sent(&self) -> f64;
    #[doc = "Get the `sockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    fn sockets(&self) -> &::wasm_bindgen::JsValue;
}
impl SocketsDictGetters for SocketsDict {
    fn received(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("received"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn sent(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("sent"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn sockets(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("sockets"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl SocketsDict {
    #[doc = "Construct a new `SocketsDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub fn received(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("received"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub fn sent(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("sent"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `sockets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketsDict`*"]
    pub fn sockets(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("sockets"),
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
impl Default for SocketsDict {
    fn default() -> Self {
        Self::new()
    }
}
