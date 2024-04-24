#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ServerSocketOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ServerSocketOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`*"]
    pub type ServerSocketOptions;
}
#[doc = "The trait to access properties on the `ServerSocketOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`*"]
pub trait ServerSocketOptionsGetters {
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Get the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpSocketBinaryType`*"]
    fn binary_type(&self) -> TcpSocketBinaryType;
}
impl ServerSocketOptionsGetters for ServerSocketOptions {
    #[cfg(feature = "TcpSocketBinaryType")]
    fn binary_type(&self) -> TcpSocketBinaryType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("binaryType"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ServerSocketOptions {
    #[doc = "Construct a new `ServerSocketOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "TcpSocketBinaryType")]
    #[doc = "Change the `binaryType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ServerSocketOptions`, `TcpSocketBinaryType`*"]
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
}
impl Default for ServerSocketOptions {
    fn default() -> Self {
        Self::new()
    }
}
