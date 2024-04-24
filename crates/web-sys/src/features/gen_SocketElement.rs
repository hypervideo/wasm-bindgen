#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SocketElement)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketElement` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub type SocketElement;
}
#[doc = "The trait to access properties on the `SocketElement` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
pub trait SocketElementGetters {
    #[doc = "Get the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn active(&self) -> bool;
    #[doc = "Get the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn host(&self) -> &str;
    #[doc = "Get the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn port(&self) -> u32;
    #[doc = "Get the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn received(&self) -> f64;
    #[doc = "Get the `sent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn sent(&self) -> f64;
    #[doc = "Get the `tcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    fn tcp(&self) -> bool;
}
impl SocketElementGetters for SocketElement {
    fn active(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("active"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn host(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("host"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn port(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("port"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
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
    fn tcp(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("tcp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl SocketElement {
    #[doc = "Construct a new `SocketElement`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `active` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn active(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("active"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `host` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn host(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("host"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `port` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn port(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("port"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `received` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
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
    #[doc = "Change the `tcp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketElement`*"]
    pub fn tcp(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("tcp"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for SocketElement {
    fn default() -> Self {
        Self::new()
    }
}
