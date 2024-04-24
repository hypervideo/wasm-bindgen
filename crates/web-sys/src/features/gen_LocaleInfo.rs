#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = LocaleInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LocaleInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    pub type LocaleInfo;
}
#[doc = "The trait to access properties on the `LocaleInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
pub trait LocaleInfoGetters {
    #[doc = "Get the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    fn direction(&self) -> &str;
    #[doc = "Get the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    fn locale(&self) -> &str;
}
impl LocaleInfoGetters for LocaleInfo {
    fn direction(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("direction"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn locale(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("locale"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl LocaleInfo {
    #[doc = "Construct a new `LocaleInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `direction` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    pub fn direction(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("direction"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `locale` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LocaleInfo`*"]
    pub fn locale(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("locale"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for LocaleInfo {
    fn default() -> Self {
        Self::new()
    }
}
