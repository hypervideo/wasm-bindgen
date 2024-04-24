#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HalfOpenInfoDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HalfOpenInfoDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
    pub type HalfOpenInfoDict;
}
#[doc = "The trait to access properties on the `HalfOpenInfoDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
pub trait HalfOpenInfoDictGetters {
    #[doc = "Get the `speculative` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
    fn speculative(&self) -> bool;
}
impl HalfOpenInfoDictGetters for HalfOpenInfoDict {
    fn speculative(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("speculative"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl HalfOpenInfoDict {
    #[doc = "Construct a new `HalfOpenInfoDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `speculative` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HalfOpenInfoDict`*"]
    pub fn speculative(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("speculative"),
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
impl Default for HalfOpenInfoDict {
    fn default() -> Self {
        Self::new()
    }
}
