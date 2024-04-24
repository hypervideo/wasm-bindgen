#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UDPOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UdpOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub type UdpOptions;
}
#[doc = "The trait to access properties on the `UdpOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
pub trait UdpOptionsGetters {
    #[doc = "Get the `addressReuse` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn address_reuse(&self) -> bool;
    #[doc = "Get the `localAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn local_address(&self) -> &str;
    #[doc = "Get the `localPort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn local_port(&self) -> u16;
    #[doc = "Get the `loopback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn loopback(&self) -> bool;
    #[doc = "Get the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn remote_address(&self) -> &str;
    #[doc = "Get the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    fn remote_port(&self) -> u16;
}
impl UdpOptionsGetters for UdpOptions {
    fn address_reuse(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("addressReuse"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn local_address(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("localAddress"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn local_port(&self) -> u16 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("localPort"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn loopback(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("loopback"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn remote_address(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("remoteAddress"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn remote_port(&self) -> u16 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("remotePort"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl UdpOptions {
    #[doc = "Construct a new `UdpOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `addressReuse` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn address_reuse(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("addressReuse"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `localAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn local_address(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("localAddress"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `localPort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn local_port(&mut self, val: u16) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("localPort"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `loopback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn loopback(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("loopback"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `remoteAddress` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn remote_address(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("remoteAddress"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `remotePort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UdpOptions`*"]
    pub fn remote_port(&mut self, val: u16) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("remotePort"),
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
impl Default for UdpOptions {
    fn default() -> Self {
        Self::new()
    }
}
