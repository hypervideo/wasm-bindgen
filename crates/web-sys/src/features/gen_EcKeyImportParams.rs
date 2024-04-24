#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EcKeyImportParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EcKeyImportParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    pub type EcKeyImportParams;
}
#[doc = "The trait to access properties on the `EcKeyImportParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
pub trait EcKeyImportParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    fn named_curve(&self) -> &str;
}
impl EcKeyImportParamsGetters for EcKeyImportParams {
    fn name(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("name"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn named_curve(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("namedCurve"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl EcKeyImportParams {
    #[doc = "Construct a new `EcKeyImportParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    pub fn new(name: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
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
    #[doc = "Change the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyImportParams`*"]
    pub fn named_curve(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("namedCurve"),
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
