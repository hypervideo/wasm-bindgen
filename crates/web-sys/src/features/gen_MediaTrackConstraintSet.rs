#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaTrackConstraintSet)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaTrackConstraintSet` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub type MediaTrackConstraintSet;
}
#[doc = "The trait to access properties on the `MediaTrackConstraintSet` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
pub trait MediaTrackConstraintSetGetters {
    #[doc = "Get the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn auto_gain_control(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `browserWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn browser_window(&self) -> f64;
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn channel_count(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn device_id(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn echo_cancellation(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn facing_mode(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn frame_rate(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn height(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `mediaSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn media_source(&self) -> &str;
    #[doc = "Get the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn noise_suppression(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `scrollWithPage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn scroll_with_page(&self) -> bool;
    #[doc = "Get the `viewportHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn viewport_height(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `viewportOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn viewport_offset_x(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `viewportOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn viewport_offset_y(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `viewportWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn viewport_width(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    fn width(&self) -> &::wasm_bindgen::JsValue;
}
impl MediaTrackConstraintSetGetters for MediaTrackConstraintSet {
    fn auto_gain_control(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("autoGainControl"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn browser_window(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("browserWindow"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn channel_count(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("channelCount"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn device_id(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("deviceId"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn echo_cancellation(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("echoCancellation"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn facing_mode(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("facingMode"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn frame_rate(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("frameRate"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn height(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("height"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn media_source(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("mediaSource"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn noise_suppression(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("noiseSuppression"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn scroll_with_page(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("scrollWithPage"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn viewport_height(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("viewportHeight"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn viewport_offset_x(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("viewportOffsetX"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn viewport_offset_y(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("viewportOffsetY"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn viewport_width(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("viewportWidth"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn width(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("width"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl MediaTrackConstraintSet {
    #[doc = "Construct a new `MediaTrackConstraintSet`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `autoGainControl` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn auto_gain_control(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("autoGainControl"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `browserWindow` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn browser_window(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("browserWindow"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn channel_count(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("channelCount"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `deviceId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn device_id(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("deviceId"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `echoCancellation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn echo_cancellation(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("echoCancellation"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `facingMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn facing_mode(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("facingMode"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `frameRate` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn frame_rate(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("frameRate"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn height(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("height"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `mediaSource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn media_source(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("mediaSource"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `noiseSuppression` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn noise_suppression(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("noiseSuppression"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `scrollWithPage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn scroll_with_page(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("scrollWithPage"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `viewportHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_height(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("viewportHeight"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `viewportOffsetX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_offset_x(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("viewportOffsetX"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `viewportOffsetY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_offset_y(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("viewportOffsetY"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `viewportWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn viewport_width(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("viewportWidth"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaTrackConstraintSet`*"]
    pub fn width(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("width"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for MediaTrackConstraintSet {
    fn default() -> Self {
        Self::new()
    }
}
