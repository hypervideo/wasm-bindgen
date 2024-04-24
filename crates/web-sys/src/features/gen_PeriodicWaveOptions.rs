#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PeriodicWaveOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PeriodicWaveOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub type PeriodicWaveOptions;
}
#[doc = "The trait to access properties on the `PeriodicWaveOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
pub trait PeriodicWaveOptionsGetters {
    #[doc = "Get the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    fn disable_normalization(&self) -> bool;
    #[doc = "Get the `imag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    fn imag(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `real` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    fn real(&self) -> &::wasm_bindgen::JsValue;
}
impl PeriodicWaveOptionsGetters for PeriodicWaveOptions {
    fn disable_normalization(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("disableNormalization"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn imag(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("imag"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn real(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("real"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl PeriodicWaveOptions {
    #[doc = "Construct a new `PeriodicWaveOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `disableNormalization` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub fn disable_normalization(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("disableNormalization"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `imag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub fn imag(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("imag"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `real` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PeriodicWaveOptions`*"]
    pub fn real(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("real"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for PeriodicWaveOptions {
    fn default() -> Self {
        Self::new()
    }
}
