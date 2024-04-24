#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AutocompleteInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AutocompleteInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub type AutocompleteInfo;
}
#[doc = "The trait to access properties on the `AutocompleteInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
pub trait AutocompleteInfoGetters {
    #[doc = "Get the `addressType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    fn address_type(&self) -> &str;
    #[doc = "Get the `contactType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    fn contact_type(&self) -> &str;
    #[doc = "Get the `fieldName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    fn field_name(&self) -> &str;
    #[doc = "Get the `section` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    fn section(&self) -> &str;
}
impl AutocompleteInfoGetters for AutocompleteInfo {
    fn address_type(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("addressType"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn contact_type(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("contactType"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn field_name(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("fieldName"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn section(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("section"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl AutocompleteInfo {
    #[doc = "Construct a new `AutocompleteInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `addressType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn address_type(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("addressType"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `contactType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn contact_type(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("contactType"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `fieldName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn field_name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("fieldName"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `section` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AutocompleteInfo`*"]
    pub fn section(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("section"),
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
impl Default for AutocompleteInfo {
    fn default() -> Self {
        Self::new()
    }
}
