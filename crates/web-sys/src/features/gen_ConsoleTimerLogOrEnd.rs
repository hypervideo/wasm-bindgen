#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConsoleTimerLogOrEnd)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleTimerLogOrEnd` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    pub type ConsoleTimerLogOrEnd;
}
#[doc = "The trait to access properties on the `ConsoleTimerLogOrEnd` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
pub trait ConsoleTimerLogOrEndGetters {
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    fn duration(&self) -> f64;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    fn name(&self) -> &str;
}
impl ConsoleTimerLogOrEndGetters for ConsoleTimerLogOrEnd {
    fn duration(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("duration"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn name(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("name"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ConsoleTimerLogOrEnd {
    #[doc = "Construct a new `ConsoleTimerLogOrEnd`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    pub fn duration(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("duration"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("name"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for ConsoleTimerLogOrEnd {
    fn default() -> Self {
        Self::new()
    }
}
