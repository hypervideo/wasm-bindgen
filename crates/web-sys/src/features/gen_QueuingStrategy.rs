#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = QueuingStrategy)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `QueuingStrategy` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    pub type QueuingStrategy;
}
#[doc = "The trait to access properties on the `QueuingStrategy` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
pub trait QueuingStrategyGetters {
    #[doc = "Get the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    fn high_water_mark(&self) -> f64;
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    fn size(&self) -> &::js_sys::Function;
}
impl QueuingStrategyGetters for QueuingStrategy {
    fn high_water_mark(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("highWaterMark"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn size(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("size"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl QueuingStrategy {
    #[doc = "Construct a new `QueuingStrategy`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `highWaterMark` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
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
    #[doc = "Change the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueuingStrategy`*"]
    pub fn size(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("size"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for QueuingStrategy {
    fn default() -> Self {
        Self::new()
    }
}
