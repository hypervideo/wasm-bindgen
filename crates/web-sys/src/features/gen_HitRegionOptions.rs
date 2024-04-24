#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HitRegionOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HitRegionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    pub type HitRegionOptions;
}
#[doc = "The trait to access properties on the `HitRegionOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
pub trait HitRegionOptionsGetters {
    #[cfg(feature = "Element")]
    #[doc = "Get the `control` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HitRegionOptions`*"]
    fn control(&self) -> Option<&Element>;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    fn id(&self) -> &str;
    #[cfg(feature = "Path2d")]
    #[doc = "Get the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`, `Path2d`*"]
    fn path(&self) -> Option<&Path2d>;
}
impl HitRegionOptionsGetters for HitRegionOptions {
    #[cfg(feature = "Element")]
    fn control(&self) -> Option<&Element> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("control"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn id(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("id"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "Path2d")]
    fn path(&self) -> Option<&Path2d> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("path"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl HitRegionOptions {
    #[doc = "Construct a new `HitRegionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "Element")]
    #[doc = "Change the `control` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HitRegionOptions`*"]
    pub fn control(&mut self, val: Option<&Element>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("control"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
    pub fn id(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("id"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "Path2d")]
    #[doc = "Change the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`, `Path2d`*"]
    pub fn path(&mut self, val: Option<&Path2d>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("path"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for HitRegionOptions {
    fn default() -> Self {
        Self::new()
    }
}
