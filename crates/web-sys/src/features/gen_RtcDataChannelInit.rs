#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCDataChannelInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDataChannelInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub type RtcDataChannelInit;
}
#[doc = "The trait to access properties on the `RtcDataChannelInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
pub trait RtcDataChannelInitGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn id(&self) -> u16;
    #[doc = "Get the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn max_packet_life_time(&self) -> u16;
    #[doc = "Get the `maxRetransmitTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn max_retransmit_time(&self) -> u16;
    #[doc = "Get the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn max_retransmits(&self) -> u16;
    #[doc = "Get the `negotiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn negotiated(&self) -> bool;
    #[doc = "Get the `ordered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn ordered(&self) -> bool;
    #[doc = "Get the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    fn protocol(&self) -> &str;
}
impl RtcDataChannelInitGetters for RtcDataChannelInit {
    fn id(&self) -> u16 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("id"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn max_packet_life_time(&self) -> u16 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("maxPacketLifeTime"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn max_retransmit_time(&self) -> u16 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("maxRetransmitTime"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn max_retransmits(&self) -> u16 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("maxRetransmits"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn negotiated(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("negotiated"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ordered(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ordered"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn protocol(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("protocol"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RtcDataChannelInit {
    #[doc = "Construct a new `RtcDataChannelInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn id(&mut self, val: u16) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("id"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `maxPacketLifeTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_packet_life_time(&mut self, val: u16) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("maxPacketLifeTime"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `maxRetransmitTime` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_retransmit_time(&mut self, val: u16) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("maxRetransmitTime"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `maxRetransmits` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn max_retransmits(&mut self, val: u16) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("maxRetransmits"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `negotiated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn negotiated(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("negotiated"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `ordered` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn ordered(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("ordered"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `protocol` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDataChannelInit`*"]
    pub fn protocol(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("protocol"),
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
impl Default for RtcDataChannelInit {
    fn default() -> Self {
        Self::new()
    }
}
