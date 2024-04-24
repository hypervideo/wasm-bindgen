#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCFecParameters)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcFecParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    pub type RtcFecParameters;
}
#[doc = "The trait to access properties on the `RtcFecParameters` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
pub trait RtcFecParametersGetters {
    #[doc = "Get the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    fn ssrc(&self) -> u32;
}
impl RtcFecParametersGetters for RtcFecParameters {
    fn ssrc(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ssrc"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RtcFecParameters {
    #[doc = "Construct a new `RtcFecParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `ssrc` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcFecParameters`*"]
    pub fn ssrc(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ssrc"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for RtcFecParameters {
    fn default() -> Self {
        Self::new()
    }
}
