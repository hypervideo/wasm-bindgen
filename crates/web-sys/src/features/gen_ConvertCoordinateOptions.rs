#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConvertCoordinateOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConvertCoordinateOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`*"]
    pub type ConvertCoordinateOptions;
}
#[doc = "The trait to access properties on the `ConvertCoordinateOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`*"]
pub trait ConvertCoordinateOptionsGetters {
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `fromBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    fn from_box(&self) -> CssBoxType;
    #[cfg(feature = "CssBoxType")]
    #[doc = "Get the `toBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    fn to_box(&self) -> CssBoxType;
}
impl ConvertCoordinateOptionsGetters for ConvertCoordinateOptions {
    #[cfg(feature = "CssBoxType")]
    fn from_box(&self) -> CssBoxType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("fromBox"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "CssBoxType")]
    fn to_box(&self) -> CssBoxType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("toBox"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ConvertCoordinateOptions {
    #[doc = "Construct a new `ConvertCoordinateOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `fromBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    pub fn from_box(&mut self, val: CssBoxType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("fromBox"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "CssBoxType")]
    #[doc = "Change the `toBox` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConvertCoordinateOptions`, `CssBoxType`*"]
    pub fn to_box(&mut self, val: CssBoxType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("toBox"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for ConvertCoordinateOptions {
    fn default() -> Self {
        Self::new()
    }
}
