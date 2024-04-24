#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AttributeNameValue)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AttributeNameValue` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
    pub type AttributeNameValue;
}
#[doc = "The trait to access properties on the `AttributeNameValue` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
pub trait AttributeNameValueGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
    fn value(&self) -> &str;
}
impl AttributeNameValueGetters for AttributeNameValue {
    fn name(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("name"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn value(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("value"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl AttributeNameValue {
    #[doc = "Construct a new `AttributeNameValue`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
    pub fn new(name: &str, value: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.value(value);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
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
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AttributeNameValue`*"]
    pub fn value(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("value"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
