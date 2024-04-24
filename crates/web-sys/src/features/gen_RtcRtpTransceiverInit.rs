#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCRtpTransceiverInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcRtpTransceiverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    pub type RtcRtpTransceiverInit;
}
#[doc = "The trait to access properties on the `RtcRtpTransceiverInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
pub trait RtcRtpTransceiverInitGetters {
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverDirection`, `RtcRtpTransceiverInit`*"]
    fn direction(&self) -> RtcRtpTransceiverDirection;
    #[doc = "Get the `sendEncodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    fn send_encodings(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    fn streams(&self) -> &::wasm_bindgen::JsValue;
}
impl RtcRtpTransceiverInitGetters for RtcRtpTransceiverInit {
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    fn direction(&self) -> RtcRtpTransceiverDirection {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("direction"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn send_encodings(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("sendEncodings"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn streams(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("streams"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RtcRtpTransceiverInit {
    #[doc = "Construct a new `RtcRtpTransceiverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "RtcRtpTransceiverDirection")]
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverDirection`, `RtcRtpTransceiverInit`*"]
    pub fn direction(&mut self, val: RtcRtpTransceiverDirection) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("direction"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `sendEncodings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    pub fn send_encodings(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("sendEncodings"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `streams` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcRtpTransceiverInit`*"]
    pub fn streams(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("streams"),
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
impl Default for RtcRtpTransceiverInit {
    fn default() -> Self {
        Self::new()
    }
}
