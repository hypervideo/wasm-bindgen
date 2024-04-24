#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = QueuingStrategyInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `QueuingStrategyInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
    pub type QueuingStrategyInit;
}
#[doc = "The trait to access properties on the `QueuingStrategyInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
pub trait QueuingStrategyInitGetters {
    #[doc = "Get the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
    fn high_water_mark(&self) -> f64;
}
impl QueuingStrategyInitGetters for QueuingStrategyInit {
    fn high_water_mark(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("highWaterMark"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl QueuingStrategyInit {
    #[doc = "Construct a new `QueuingStrategyInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
    pub fn new(high_water_mark: f64) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.high_water_mark(high_water_mark);
        ret
    }
    #[doc = "Change the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategyInit`*"]
    pub fn high_water_mark(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("highWaterMark"),
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
