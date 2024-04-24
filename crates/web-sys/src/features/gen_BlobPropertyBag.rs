#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BlobPropertyBag)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BlobPropertyBag` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
    pub type BlobPropertyBag;
}
#[doc = "The trait to access properties on the `BlobPropertyBag` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
pub trait BlobPropertyBagGetters {
    #[cfg(feature = "EndingTypes")]
    #[doc = "Get the `endings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`, `EndingTypes`*"]
    fn endings(&self) -> EndingTypes;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
    fn type_(&self) -> &str;
}
impl BlobPropertyBagGetters for BlobPropertyBag {
    #[cfg(feature = "EndingTypes")]
    fn endings(&self) -> EndingTypes {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("endings"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn type_(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl BlobPropertyBag {
    #[doc = "Construct a new `BlobPropertyBag`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "EndingTypes")]
    #[doc = "Change the `endings` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`, `EndingTypes`*"]
    pub fn endings(&mut self, val: EndingTypes) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("endings"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlobPropertyBag`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
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
impl Default for BlobPropertyBag {
    fn default() -> Self {
        Self::new()
    }
}
