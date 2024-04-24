#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WriteParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WriteParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    pub type WriteParams;
}
#[doc = "The trait to access properties on the `WriteParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
pub trait WriteParamsGetters {
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    fn data(&self) -> Option<&::wasm_bindgen::JsValue>;
    #[doc = "Get the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    fn position(&self) -> Option<f64>;
    #[doc = "Get the `size` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    fn size(&self) -> Option<f64>;
    #[cfg(feature = "WriteCommandType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteCommandType`, `WriteParams`*"]
    fn type_(&self) -> WriteCommandType;
}
impl WriteParamsGetters for WriteParams {
    fn data(&self) -> Option<&::wasm_bindgen::JsValue> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("data"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn position(&self) -> Option<f64> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("position"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn size(&self) -> Option<f64> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("size"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "WriteCommandType")]
    fn type_(&self) -> WriteCommandType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl WriteParams {
    #[cfg(feature = "WriteCommandType")]
    #[doc = "Construct a new `WriteParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteCommandType`, `WriteParams`*"]
    pub fn new(type_: WriteCommandType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    pub fn data(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("data"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `position` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    pub fn position(&mut self, val: Option<f64>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("position"),
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
    #[doc = "*This API requires the following crate features to be activated: `WriteParams`*"]
    pub fn size(&mut self, val: Option<f64>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("size"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "WriteCommandType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WriteCommandType`, `WriteParams`*"]
    pub fn type_(&mut self, val: WriteCommandType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
