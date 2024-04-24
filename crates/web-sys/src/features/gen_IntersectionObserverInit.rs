#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IntersectionObserverInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    pub type IntersectionObserverInit;
}
#[doc = "The trait to access properties on the `IntersectionObserverInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
pub trait IntersectionObserverInitGetters {
    #[cfg(feature = "Element")]
    #[doc = "Get the `root` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverInit`*"]
    fn root(&self) -> Option<&Element>;
    #[doc = "Get the `rootMargin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    fn root_margin(&self) -> &str;
    #[doc = "Get the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    fn threshold(&self) -> &::wasm_bindgen::JsValue;
}
impl IntersectionObserverInitGetters for IntersectionObserverInit {
    #[cfg(feature = "Element")]
    fn root(&self) -> Option<&Element> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("root"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn root_margin(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("rootMargin"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn threshold(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("threshold"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl IntersectionObserverInit {
    #[doc = "Construct a new `IntersectionObserverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "Element")]
    #[doc = "Change the `root` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverInit`*"]
    pub fn root(&mut self, val: Option<&Element>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("root"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `rootMargin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    pub fn root_margin(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rootMargin"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverInit`*"]
    pub fn threshold(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("threshold"),
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
impl Default for IntersectionObserverInit {
    fn default() -> Self {
        Self::new()
    }
}
