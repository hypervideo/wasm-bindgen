#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaTrackCapabilities)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaTrackCapabilities` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type MediaTrackCapabilities;
    #[cfg(feature = "DoubleRange")]
    #[wasm_bindgen(method, getter = "aspectRatio")]
    fn aspect_ratio_shim(this: &MediaTrackCapabilities) -> DoubleRange;
    #[cfg(feature = "DoubleRange")]
    #[wasm_bindgen(method, setter = "aspectRatio")]
    fn set_aspect_ratio_shim(this: &MediaTrackCapabilities, val: &DoubleRange);
    #[wasm_bindgen(method, getter = "autoGainControl")]
    fn auto_gain_control_shim(this: &MediaTrackCapabilities) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "autoGainControl")]
    fn set_auto_gain_control_shim(this: &MediaTrackCapabilities, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "ULongRange")]
    #[wasm_bindgen(method, getter = "channelCount")]
    fn channel_count_shim(this: &MediaTrackCapabilities) -> ULongRange;
    #[cfg(feature = "ULongRange")]
    #[wasm_bindgen(method, setter = "channelCount")]
    fn set_channel_count_shim(this: &MediaTrackCapabilities, val: &ULongRange);
    #[wasm_bindgen(method, getter = "deviceId")]
    fn device_id_shim(this: &MediaTrackCapabilities) -> String;
    #[wasm_bindgen(method, setter = "deviceId")]
    fn set_device_id_shim(this: &MediaTrackCapabilities, val: &str);
    #[wasm_bindgen(method, getter = "echoCancellation")]
    fn echo_cancellation_shim(this: &MediaTrackCapabilities) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "echoCancellation")]
    fn set_echo_cancellation_shim(this: &MediaTrackCapabilities, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "facingMode")]
    fn facing_mode_shim(this: &MediaTrackCapabilities) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "facingMode")]
    fn set_facing_mode_shim(this: &MediaTrackCapabilities, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "DoubleRange")]
    #[wasm_bindgen(method, getter = "frameRate")]
    fn frame_rate_shim(this: &MediaTrackCapabilities) -> DoubleRange;
    #[cfg(feature = "DoubleRange")]
    #[wasm_bindgen(method, setter = "frameRate")]
    fn set_frame_rate_shim(this: &MediaTrackCapabilities, val: &DoubleRange);
    #[wasm_bindgen(method, getter = "groupId")]
    fn group_id_shim(this: &MediaTrackCapabilities) -> String;
    #[wasm_bindgen(method, setter = "groupId")]
    fn set_group_id_shim(this: &MediaTrackCapabilities, val: &str);
    #[cfg(feature = "ULongRange")]
    #[wasm_bindgen(method, getter = "height")]
    fn height_shim(this: &MediaTrackCapabilities) -> ULongRange;
    #[cfg(feature = "ULongRange")]
    #[wasm_bindgen(method, setter = "height")]
    fn set_height_shim(this: &MediaTrackCapabilities, val: &ULongRange);
    #[cfg(feature = "DoubleRange")]
    #[wasm_bindgen(method, getter = "latency")]
    fn latency_shim(this: &MediaTrackCapabilities) -> DoubleRange;
    #[cfg(feature = "DoubleRange")]
    #[wasm_bindgen(method, setter = "latency")]
    fn set_latency_shim(this: &MediaTrackCapabilities, val: &DoubleRange);
    #[wasm_bindgen(method, getter = "noiseSuppression")]
    fn noise_suppression_shim(this: &MediaTrackCapabilities) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "noiseSuppression")]
    fn set_noise_suppression_shim(this: &MediaTrackCapabilities, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "resizeMode")]
    fn resize_mode_shim(this: &MediaTrackCapabilities) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "resizeMode")]
    fn set_resize_mode_shim(this: &MediaTrackCapabilities, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "ULongRange")]
    #[wasm_bindgen(method, getter = "sampleRate")]
    fn sample_rate_shim(this: &MediaTrackCapabilities) -> ULongRange;
    #[cfg(feature = "ULongRange")]
    #[wasm_bindgen(method, setter = "sampleRate")]
    fn set_sample_rate_shim(this: &MediaTrackCapabilities, val: &ULongRange);
    #[cfg(feature = "ULongRange")]
    #[wasm_bindgen(method, getter = "sampleSize")]
    fn sample_size_shim(this: &MediaTrackCapabilities) -> ULongRange;
    #[cfg(feature = "ULongRange")]
    #[wasm_bindgen(method, setter = "sampleSize")]
    fn set_sample_size_shim(this: &MediaTrackCapabilities, val: &ULongRange);
    #[cfg(feature = "ULongRange")]
    #[wasm_bindgen(method, getter = "width")]
    fn width_shim(this: &MediaTrackCapabilities) -> ULongRange;
    #[cfg(feature = "ULongRange")]
    #[wasm_bindgen(method, setter = "width")]
    fn set_width_shim(this: &MediaTrackCapabilities, val: &ULongRange);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `MediaTrackCapabilities` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
pub trait MediaTrackCapabilitiesGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Get the `aspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn aspect_ratio(&self) -> DoubleRange;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn auto_gain_control(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn channel_count(&self) -> ULongRange;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn device_id(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn echo_cancellation(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn facing_mode(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn frame_rate(&self) -> DoubleRange;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn group_id(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn height(&self) -> ULongRange;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Get the `latency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn latency(&self) -> DoubleRange;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn noise_suppression(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `resizeMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn resize_mode(&self) -> ::js_sys::Array;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn sample_rate(&self) -> ULongRange;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `sampleSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn sample_size(&self) -> ULongRange;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn width(&self) -> ULongRange;
}
#[cfg(web_sys_unstable_apis)]
impl MediaTrackCapabilitiesGetters for MediaTrackCapabilities {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    fn aspect_ratio(&self) -> DoubleRange {
        self.aspect_ratio_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn auto_gain_control(&self) -> ::js_sys::Array {
        self.auto_gain_control_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    fn channel_count(&self) -> ULongRange {
        self.channel_count_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn device_id(&self) -> String {
        self.device_id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn echo_cancellation(&self) -> ::js_sys::Array {
        self.echo_cancellation_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn facing_mode(&self) -> ::js_sys::Array {
        self.facing_mode_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    fn frame_rate(&self) -> DoubleRange {
        self.frame_rate_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn group_id(&self) -> String {
        self.group_id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    fn height(&self) -> ULongRange {
        self.height_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    fn latency(&self) -> DoubleRange {
        self.latency_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn noise_suppression(&self) -> ::js_sys::Array {
        self.noise_suppression_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn resize_mode(&self) -> ::js_sys::Array {
        self.resize_mode_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    fn sample_rate(&self) -> ULongRange {
        self.sample_rate_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    fn sample_size(&self) -> ULongRange {
        self.sample_size_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    fn width(&self) -> ULongRange {
        self.width_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl MediaTrackCapabilities {
    #[doc = "Construct a new `MediaTrackCapabilities`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Change the `aspectRatio` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn aspect_ratio(&mut self, val: &DoubleRange) -> &mut Self {
        self.set_aspect_ratio_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn auto_gain_control(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_auto_gain_control_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn channel_count(&mut self, val: &ULongRange) -> &mut Self {
        self.set_channel_count_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn device_id(&mut self, val: &str) -> &mut Self {
        self.set_device_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn echo_cancellation(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_echo_cancellation_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn facing_mode(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_facing_mode_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn frame_rate(&mut self, val: &DoubleRange) -> &mut Self {
        self.set_frame_rate_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `groupId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn group_id(&mut self, val: &str) -> &mut Self {
        self.set_group_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn height(&mut self, val: &ULongRange) -> &mut Self {
        self.set_height_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DoubleRange")]
    #[doc = "Change the `latency` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DoubleRange`, `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn latency(&mut self, val: &DoubleRange) -> &mut Self {
        self.set_latency_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn noise_suppression(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_noise_suppression_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `resizeMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn resize_mode(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_resize_mode_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `sampleRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sample_rate(&mut self, val: &ULongRange) -> &mut Self {
        self.set_sample_rate_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `sampleSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn sample_size(&mut self, val: &ULongRange) -> &mut Self {
        self.set_sample_size_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ULongRange")]
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackCapabilities`, `ULongRange`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn width(&mut self, val: &ULongRange) -> &mut Self {
        self.set_width_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for MediaTrackCapabilities {
    fn default() -> Self {
        Self::new()
    }
}
