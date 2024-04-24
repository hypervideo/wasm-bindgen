#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CheckerboardReport)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CheckerboardReport` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    pub type CheckerboardReport;
}
#[doc = "The trait to access properties on the `CheckerboardReport` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
pub trait CheckerboardReportGetters {
    #[doc = "Get the `log` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    fn log(&self) -> &str;
    #[cfg(feature = "CheckerboardReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReason`, `CheckerboardReport`*"]
    fn reason(&self) -> CheckerboardReason;
    #[doc = "Get the `severity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    fn severity(&self) -> u32;
    #[doc = "Get the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    fn timestamp(&self) -> f64;
}
impl CheckerboardReportGetters for CheckerboardReport {
    fn log(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("log"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "CheckerboardReason")]
    fn reason(&self) -> CheckerboardReason {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("reason"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn severity(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("severity"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn timestamp(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("timestamp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl CheckerboardReport {
    #[doc = "Construct a new `CheckerboardReport`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `log` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    pub fn log(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("log"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "CheckerboardReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReason`, `CheckerboardReport`*"]
    pub fn reason(&mut self, val: CheckerboardReason) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("reason"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `severity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
    pub fn severity(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("severity"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReport`*"]
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
}
impl Default for CheckerboardReport {
    fn default() -> Self {
        Self::new()
    }
}
