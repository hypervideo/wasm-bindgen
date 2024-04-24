#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ScrollOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"]
    pub type ScrollOptions;
}
#[doc = "The trait to access properties on the `ScrollOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"]
pub trait ScrollOptionsGetters {
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Get the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollOptions`*"]
    fn behavior(&self) -> ScrollBehavior;
}
impl ScrollOptionsGetters for ScrollOptions {
    #[cfg(feature = "ScrollBehavior")]
    fn behavior(&self) -> ScrollBehavior {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("behavior"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ScrollOptions {
    #[doc = "Construct a new `ScrollOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Change the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollOptions`*"]
    pub fn behavior(&mut self, val: ScrollBehavior) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("behavior"),
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
impl Default for ScrollOptions {
    fn default() -> Self {
        Self::new()
    }
}
