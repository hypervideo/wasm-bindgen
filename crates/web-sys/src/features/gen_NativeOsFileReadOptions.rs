#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NativeOSFileReadOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NativeOsFileReadOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    pub type NativeOsFileReadOptions;
}
#[doc = "The trait to access properties on the `NativeOsFileReadOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
pub trait NativeOsFileReadOptionsGetters {
    #[doc = "Get the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    fn bytes(&self) -> Option<f64>;
    #[doc = "Get the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    fn encoding(&self) -> Option<&str>;
}
impl NativeOsFileReadOptionsGetters for NativeOsFileReadOptions {
    fn bytes(&self) -> Option<f64> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("bytes"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn encoding(&self) -> Option<&str> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("encoding"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl NativeOsFileReadOptions {
    #[doc = "Construct a new `NativeOsFileReadOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    pub fn bytes(&mut self, val: Option<f64>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("bytes"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `encoding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileReadOptions`*"]
    pub fn encoding(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("encoding"),
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
impl Default for NativeOsFileReadOptions {
    fn default() -> Self {
        Self::new()
    }
}
