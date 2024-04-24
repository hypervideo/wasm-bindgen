#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RsaPssParams)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaPssParams` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    pub type RsaPssParams;
}
#[doc = "The trait to access properties on the `RsaPssParams` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
pub trait RsaPssParamsGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    fn name(&self) -> &str;
    #[doc = "Get the `saltLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    fn salt_length(&self) -> u32;
}
impl RsaPssParamsGetters for RsaPssParams {
    fn name(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("name"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn salt_length(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("saltLength"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RsaPssParams {
    #[doc = "Construct a new `RsaPssParams`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    pub fn new(name: &str, salt_length: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.salt_length(salt_length);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
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
    #[doc = "Change the `saltLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaPssParams`*"]
    pub fn salt_length(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("saltLength"),
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
