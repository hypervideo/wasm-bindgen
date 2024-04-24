#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCMediaStreamTrackStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcMediaStreamTrackStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub type RtcMediaStreamTrackStats;
}
#[doc = "The trait to access properties on the `RtcMediaStreamTrackStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
pub trait RtcMediaStreamTrackStatsGetters {
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn id(&self) -> &str;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn timestamp(&self) -> f64;
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`, `RtcStatsType`*"]
    fn type_(&self) -> RtcStatsType;
    #[doc = "Get the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn audio_level(&self) -> f64;
    #[doc = "Get the `echoReturnLoss` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn echo_return_loss(&self) -> f64;
    #[doc = "Get the `echoReturnLossEnhancement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn echo_return_loss_enhancement(&self) -> f64;
    #[doc = "Get the `frameHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frame_height(&self) -> u32;
    #[doc = "Get the `frameWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frame_width(&self) -> u32;
    #[doc = "Get the `framesCorrupted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_corrupted(&self) -> u32;
    #[doc = "Get the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_decoded(&self) -> u32;
    #[doc = "Get the `framesDropped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_dropped(&self) -> u32;
    #[doc = "Get the `framesPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_per_second(&self) -> f64;
    #[doc = "Get the `framesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_received(&self) -> u32;
    #[doc = "Get the `framesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn frames_sent(&self) -> u32;
    #[doc = "Get the `remoteSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn remote_source(&self) -> bool;
    #[doc = "Get the `ssrcIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn ssrc_ids(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `trackIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    fn track_identifier(&self) -> &str;
}
impl RtcMediaStreamTrackStatsGetters for RtcMediaStreamTrackStats {
    fn id(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("id"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn timestamp(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("timestamp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "RtcStatsType")]
    fn type_(&self) -> RtcStatsType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn audio_level(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("audioLevel"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn echo_return_loss(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("echoReturnLoss"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn echo_return_loss_enhancement(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("echoReturnLossEnhancement"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn frame_height(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("frameHeight"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn frame_width(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("frameWidth"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn frames_corrupted(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("framesCorrupted"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn frames_decoded(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("framesDecoded"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn frames_dropped(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("framesDropped"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn frames_per_second(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("framesPerSecond"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn frames_received(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("framesReceived"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn frames_sent(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("framesSent"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn remote_source(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("remoteSource"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ssrc_ids(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ssrcIds"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn track_identifier(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("trackIdentifier"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RtcMediaStreamTrackStats {
    #[doc = "Construct a new `RtcMediaStreamTrackStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("id"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn timestamp(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("timestamp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "RtcStatsType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`, `RtcStatsType`*"]
    pub fn type_(&mut self, val: RtcStatsType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `audioLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn audio_level(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("audioLevel"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `echoReturnLoss` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn echo_return_loss(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("echoReturnLoss"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `echoReturnLossEnhancement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn echo_return_loss_enhancement(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("echoReturnLossEnhancement"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `frameHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frame_height(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("frameHeight"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `frameWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frame_width(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("frameWidth"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `framesCorrupted` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_corrupted(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("framesCorrupted"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `framesDecoded` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_decoded(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("framesDecoded"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `framesDropped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_dropped(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("framesDropped"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `framesPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_per_second(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("framesPerSecond"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `framesReceived` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_received(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("framesReceived"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `framesSent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn frames_sent(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("framesSent"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `remoteSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn remote_source(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("remoteSource"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `ssrcIds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn ssrc_ids(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("ssrcIds"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `trackIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcMediaStreamTrackStats`*"]
    pub fn track_identifier(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("trackIdentifier"),
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
impl Default for RtcMediaStreamTrackStats {
    fn default() -> Self {
        Self::new()
    }
}
