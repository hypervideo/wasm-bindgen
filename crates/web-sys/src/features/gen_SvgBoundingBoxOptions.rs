#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SVGBoundingBoxOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SvgBoundingBoxOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub type SvgBoundingBoxOptions;
}
#[doc = "The trait to access properties on the `SvgBoundingBoxOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
pub trait SvgBoundingBoxOptionsGetters {
    #[doc = "Get the `clipped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    fn clipped(&self) -> bool;
    #[doc = "Get the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    fn fill(&self) -> bool;
    #[doc = "Get the `markers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    fn markers(&self) -> bool;
    #[doc = "Get the `stroke` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    fn stroke(&self) -> bool;
}
impl SvgBoundingBoxOptionsGetters for SvgBoundingBoxOptions {
    fn clipped(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("clipped"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn fill(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("fill"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn markers(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("markers"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn stroke(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("stroke"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl SvgBoundingBoxOptions {
    #[doc = "Construct a new `SvgBoundingBoxOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `clipped` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn clipped(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("clipped"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `fill` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn fill(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("fill"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `markers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn markers(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("markers"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `stroke` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgBoundingBoxOptions`*"]
    pub fn stroke(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("stroke"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for SvgBoundingBoxOptions {
    fn default() -> Self {
        Self::new()
    }
}
