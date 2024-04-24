#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ShadowRootInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ShadowRootInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`*"]
    pub type ShadowRootInit;
}
#[doc = "The trait to access properties on the `ShadowRootInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`*"]
pub trait ShadowRootInitGetters {
    #[cfg(feature = "ShadowRootMode")]
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"]
    fn mode(&self) -> ShadowRootMode;
}
impl ShadowRootInitGetters for ShadowRootInit {
    #[cfg(feature = "ShadowRootMode")]
    fn mode(&self) -> ShadowRootMode {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("mode"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ShadowRootInit {
    #[cfg(feature = "ShadowRootMode")]
    #[doc = "Construct a new `ShadowRootInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"]
    pub fn new(mode: ShadowRootMode) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.mode(mode);
        ret
    }
    #[cfg(feature = "ShadowRootMode")]
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ShadowRootInit`, `ShadowRootMode`*"]
    pub fn mode(&mut self, val: ShadowRootMode) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("mode"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
