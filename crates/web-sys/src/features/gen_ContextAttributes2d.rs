#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ContextAttributes2D)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ContextAttributes2d` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub type ContextAttributes2d;
}
#[doc = "The trait to access properties on the `ContextAttributes2d` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
pub trait ContextAttributes2dGetters {
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    fn alpha(&self) -> bool;
    #[doc = "Get the `willReadFrequently` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    fn will_read_frequently(&self) -> bool;
}
impl ContextAttributes2dGetters for ContextAttributes2d {
    fn alpha(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("alpha"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn will_read_frequently(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("willReadFrequently"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ContextAttributes2d {
    #[doc = "Construct a new `ContextAttributes2d`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub fn alpha(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("alpha"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `willReadFrequently` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ContextAttributes2d`*"]
    pub fn will_read_frequently(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("willReadFrequently"),
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
impl Default for ContextAttributes2d {
    fn default() -> Self {
        Self::new()
    }
}
