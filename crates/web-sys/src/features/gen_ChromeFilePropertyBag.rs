#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ChromeFilePropertyBag)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ChromeFilePropertyBag` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub type ChromeFilePropertyBag;
}
#[doc = "The trait to access properties on the `ChromeFilePropertyBag` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
pub trait ChromeFilePropertyBagGetters {
    #[doc = "Get the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    fn last_modified(&self) -> f64;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    fn type_(&self) -> &str;
    #[doc = "Get the `existenceCheck` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    fn existence_check(&self) -> bool;
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    fn name(&self) -> &str;
}
impl ChromeFilePropertyBagGetters for ChromeFilePropertyBag {
    fn last_modified(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("lastModified"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn type_(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn existence_check(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("existenceCheck"));
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
impl ChromeFilePropertyBag {
    #[doc = "Construct a new `ChromeFilePropertyBag`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `lastModified` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub fn last_modified(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("lastModified"),
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
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
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
    #[doc = "Change the `existenceCheck` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
    pub fn existence_check(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("existenceCheck"),
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
    #[doc = "*This API requires the following crate features to be activated: `ChromeFilePropertyBag`*"]
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
impl Default for ChromeFilePropertyBag {
    fn default() -> Self {
        Self::new()
    }
}
