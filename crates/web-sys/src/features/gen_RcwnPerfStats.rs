#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RcwnPerfStats)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RcwnPerfStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub type RcwnPerfStats;
}
#[doc = "The trait to access properties on the `RcwnPerfStats` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
pub trait RcwnPerfStatsGetters {
    #[doc = "Get the `avgLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    fn avg_long(&self) -> u32;
    #[doc = "Get the `avgShort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    fn avg_short(&self) -> u32;
    #[doc = "Get the `stddevLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    fn stddev_long(&self) -> u32;
}
impl RcwnPerfStatsGetters for RcwnPerfStats {
    fn avg_long(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("avgLong"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn avg_short(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("avgShort"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn stddev_long(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("stddevLong"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RcwnPerfStats {
    #[doc = "Construct a new `RcwnPerfStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `avgLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub fn avg_long(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("avgLong"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `avgShort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub fn avg_short(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("avgShort"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `stddevLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
    pub fn stddev_long(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("stddevLong"),
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
impl Default for RcwnPerfStats {
    fn default() -> Self {
        Self::new()
    }
}
