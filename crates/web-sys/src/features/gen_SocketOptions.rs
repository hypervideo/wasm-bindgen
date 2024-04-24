#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SocketOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SocketOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    pub type SocketOptions;
}
#[doc = "The trait to access properties on the `SocketOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
pub trait SocketOptionsGetters {
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Get the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocketBinaryType`*"]
    fn binary_type(&self) -> TcpSocketBinaryType;
    #[doc = "Get the `useSecureTransport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    fn use_secure_transport(&self) -> bool;
}
impl SocketOptionsGetters for SocketOptions {
    #[cfg(feature = "TcpSocketBinaryType")]
    fn binary_type(&self) -> TcpSocketBinaryType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("binaryType"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn use_secure_transport(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("useSecureTransport"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl SocketOptions {
    #[doc = "Construct a new `SocketOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Change the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`, `TcpSocketBinaryType`*"]
    pub fn binary_type(&mut self, val: TcpSocketBinaryType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("binaryType"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `useSecureTransport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SocketOptions`*"]
    pub fn use_secure_transport(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("useSecureTransport"),
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
impl Default for SocketOptions {
    fn default() -> Self {
        Self::new()
    }
}
