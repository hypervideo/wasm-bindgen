#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RsaOtherPrimesInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaOtherPrimesInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub type RsaOtherPrimesInfo;
}
#[doc = "The trait to access properties on the `RsaOtherPrimesInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
pub trait RsaOtherPrimesInfoGetters {
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    fn d(&self) -> &str;
    #[doc = "Get the `r` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    fn r(&self) -> &str;
    #[doc = "Get the `t` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    fn t(&self) -> &str;
}
impl RsaOtherPrimesInfoGetters for RsaOtherPrimesInfo {
    fn d(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("d"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn r(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("r"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn t(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("t"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RsaOtherPrimesInfo {
    #[doc = "Construct a new `RsaOtherPrimesInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub fn new(d: &str, r: &str, t: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.d(d);
        ret.r(r);
        ret.t(t);
        ret
    }
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub fn d(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("d"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `r` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub fn r(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("r"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `t` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
    pub fn t(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("t"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
