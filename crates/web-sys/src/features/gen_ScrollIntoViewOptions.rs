#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ScrollIntoViewOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ScrollIntoViewOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*"]
    pub type ScrollIntoViewOptions;
}
#[doc = "The trait to access properties on the `ScrollIntoViewOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*"]
pub trait ScrollIntoViewOptionsGetters {
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Get the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollIntoViewOptions`*"]
    fn behavior(&self) -> ScrollBehavior;
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Get the `block` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
    fn block(&self) -> ScrollLogicalPosition;
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Get the `inline` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
    fn inline(&self) -> ScrollLogicalPosition;
}
impl ScrollIntoViewOptionsGetters for ScrollIntoViewOptions {
    #[cfg(feature = "ScrollBehavior")]
    fn behavior(&self) -> ScrollBehavior {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("behavior"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "ScrollLogicalPosition")]
    fn block(&self) -> ScrollLogicalPosition {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("block"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "ScrollLogicalPosition")]
    fn inline(&self) -> ScrollLogicalPosition {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("inline"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ScrollIntoViewOptions {
    #[doc = "Construct a new `ScrollIntoViewOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ScrollBehavior")]
    #[doc = "Change the `behavior` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollBehavior`, `ScrollIntoViewOptions`*"]
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
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Change the `block` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
    pub fn block(&mut self, val: ScrollLogicalPosition) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("block"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "ScrollLogicalPosition")]
    #[doc = "Change the `inline` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollIntoViewOptions`, `ScrollLogicalPosition`*"]
    pub fn inline(&mut self, val: ScrollLogicalPosition) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("inline"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for ScrollIntoViewOptions {
    fn default() -> Self {
        Self::new()
    }
}
