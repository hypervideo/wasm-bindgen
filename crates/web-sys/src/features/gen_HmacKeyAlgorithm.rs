#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HmacKeyAlgorithm)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HmacKeyAlgorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    pub type HmacKeyAlgorithm;
}
#[doc = "The trait to access properties on the `HmacKeyAlgorithm` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
pub trait HmacKeyAlgorithmGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    fn name(&self) -> &str;
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Get the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
    fn hash(&self) -> &KeyAlgorithm;
    #[doc = "Get the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    fn length(&self) -> u32;
}
impl HmacKeyAlgorithmGetters for HmacKeyAlgorithm {
    fn name(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("name"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "KeyAlgorithm")]
    fn hash(&self) -> &KeyAlgorithm {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("hash"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn length(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("length"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl HmacKeyAlgorithm {
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Construct a new `HmacKeyAlgorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
    pub fn new(name: &str, hash: &KeyAlgorithm, length: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.name(name);
        ret.hash(hash);
        ret.length(length);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
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
    #[cfg(feature = "KeyAlgorithm")]
    #[doc = "Change the `hash` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`, `KeyAlgorithm`*"]
    pub fn hash(&mut self, val: &KeyAlgorithm) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("hash"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `length` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HmacKeyAlgorithm`*"]
    pub fn length(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("length"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
