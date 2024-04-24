#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PannerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PannerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub type PannerOptions;
}
#[doc = "The trait to access properties on the `PannerOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
pub trait PannerOptionsGetters {
    #[doc = "Get the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn channel_count(&self) -> u32;
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Get the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `PannerOptions`*"]
    fn channel_count_mode(&self) -> ChannelCountMode;
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Get the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `PannerOptions`*"]
    fn channel_interpretation(&self) -> ChannelInterpretation;
    #[doc = "Get the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn cone_inner_angle(&self) -> f64;
    #[doc = "Get the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn cone_outer_angle(&self) -> f64;
    #[doc = "Get the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn cone_outer_gain(&self) -> f64;
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Get the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerOptions`*"]
    fn distance_model(&self) -> DistanceModelType;
    #[doc = "Get the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn max_distance(&self) -> f64;
    #[doc = "Get the `orientationX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn orientation_x(&self) -> f32;
    #[doc = "Get the `orientationY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn orientation_y(&self) -> f32;
    #[doc = "Get the `orientationZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn orientation_z(&self) -> f32;
    #[cfg(feature = "PanningModelType")]
    #[doc = "Get the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`, `PanningModelType`*"]
    fn panning_model(&self) -> PanningModelType;
    #[doc = "Get the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn position_x(&self) -> f32;
    #[doc = "Get the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn position_y(&self) -> f32;
    #[doc = "Get the `positionZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn position_z(&self) -> f32;
    #[doc = "Get the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn ref_distance(&self) -> f64;
    #[doc = "Get the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    fn rolloff_factor(&self) -> f64;
}
impl PannerOptionsGetters for PannerOptions {
    fn channel_count(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("channelCount"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "ChannelCountMode")]
    fn channel_count_mode(&self) -> ChannelCountMode {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("channelCountMode"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "ChannelInterpretation")]
    fn channel_interpretation(&self) -> ChannelInterpretation {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("channelInterpretation"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn cone_inner_angle(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("coneInnerAngle"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn cone_outer_angle(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("coneOuterAngle"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn cone_outer_gain(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("coneOuterGain"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "DistanceModelType")]
    fn distance_model(&self) -> DistanceModelType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("distanceModel"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn max_distance(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("maxDistance"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn orientation_x(&self) -> f32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("orientationX"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn orientation_y(&self) -> f32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("orientationY"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn orientation_z(&self) -> f32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("orientationZ"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "PanningModelType")]
    fn panning_model(&self) -> PanningModelType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("panningModel"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn position_x(&self) -> f32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("positionX"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn position_y(&self) -> f32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("positionY"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn position_z(&self) -> f32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("positionZ"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ref_distance(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("refDistance"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn rolloff_factor(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("rolloffFactor"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl PannerOptions {
    #[doc = "Construct a new `PannerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `channelCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn channel_count(&mut self, val: u32) -> &mut Self {
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
    #[cfg(feature = "ChannelCountMode")]
    #[doc = "Change the `channelCountMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelCountMode`, `PannerOptions`*"]
    pub fn channel_count_mode(&mut self, val: ChannelCountMode) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("channelCountMode"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "ChannelInterpretation")]
    #[doc = "Change the `channelInterpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChannelInterpretation`, `PannerOptions`*"]
    pub fn channel_interpretation(&mut self, val: ChannelInterpretation) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("channelInterpretation"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `coneInnerAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn cone_inner_angle(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("coneInnerAngle"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `coneOuterAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn cone_outer_angle(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("coneOuterAngle"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `coneOuterGain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn cone_outer_gain(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("coneOuterGain"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "DistanceModelType")]
    #[doc = "Change the `distanceModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DistanceModelType`, `PannerOptions`*"]
    pub fn distance_model(&mut self, val: DistanceModelType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("distanceModel"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `maxDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn max_distance(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("maxDistance"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `orientationX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn orientation_x(&mut self, val: f32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("orientationX"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `orientationY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn orientation_y(&mut self, val: f32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("orientationY"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `orientationZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn orientation_z(&mut self, val: f32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("orientationZ"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "PanningModelType")]
    #[doc = "Change the `panningModel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`, `PanningModelType`*"]
    pub fn panning_model(&mut self, val: PanningModelType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("panningModel"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `positionX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn position_x(&mut self, val: f32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("positionX"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `positionY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn position_y(&mut self, val: f32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("positionY"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `positionZ` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn position_z(&mut self, val: f32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("positionZ"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `refDistance` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn ref_distance(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("refDistance"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `rolloffFactor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PannerOptions`*"]
    pub fn rolloff_factor(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rolloffFactor"),
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
impl Default for PannerOptions {
    fn default() -> Self {
        Self::new()
    }
}
