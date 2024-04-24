#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BaseComputedKeyframe)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BaseComputedKeyframe` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub type BaseComputedKeyframe;
}
#[doc = "The trait to access properties on the `BaseComputedKeyframe` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
pub trait BaseComputedKeyframeGetters {
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`, `CompositeOperation`*"]
    fn composite(&self) -> Option<CompositeOperation>;
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    fn easing(&self) -> &str;
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    fn offset(&self) -> Option<f64>;
    #[doc = "Get the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    fn simulate_compute_values_failure(&self) -> bool;
    #[doc = "Get the `computedOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    fn computed_offset(&self) -> f64;
}
impl BaseComputedKeyframeGetters for BaseComputedKeyframe {
    #[cfg(feature = "CompositeOperation")]
    fn composite(&self) -> Option<CompositeOperation> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("composite"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn easing(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("easing"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn offset(&self) -> Option<f64> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("offset"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn simulate_compute_values_failure(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(
            self.as_ref(),
            &JsValue::from("simulateComputeValuesFailure"),
        );
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn computed_offset(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("computedOffset"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl BaseComputedKeyframe {
    #[doc = "Construct a new `BaseComputedKeyframe`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`, `CompositeOperation`*"]
    pub fn composite(&mut self, val: Option<CompositeOperation>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("composite"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn easing(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("easing"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn offset(&mut self, val: Option<f64>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("offset"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn simulate_compute_values_failure(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("simulateComputeValuesFailure"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `computedOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
    pub fn computed_offset(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("computedOffset"),
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
impl Default for BaseComputedKeyframe {
    fn default() -> Self {
        Self::new()
    }
}
