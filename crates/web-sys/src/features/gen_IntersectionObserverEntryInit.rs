#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IntersectionObserverEntryInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IntersectionObserverEntryInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
    pub type IntersectionObserverEntryInit;
}
#[doc = "The trait to access properties on the `IntersectionObserverEntryInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
pub trait IntersectionObserverEntryInitGetters {
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    fn bounding_client_rect(&self) -> &DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `intersectionRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    fn intersection_rect(&self) -> &DomRectInit;
    #[cfg(feature = "DomRectInit")]
    #[doc = "Get the `rootBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    fn root_bounds(&self) -> &DomRectInit;
    #[cfg(feature = "Element")]
    #[doc = "Get the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntryInit`*"]
    fn target(&self) -> &Element;
    #[doc = "Get the `time` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
    fn time(&self) -> f64;
}
impl IntersectionObserverEntryInitGetters for IntersectionObserverEntryInit {
    #[cfg(feature = "DomRectInit")]
    fn bounding_client_rect(&self) -> &DomRectInit {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("boundingClientRect"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "DomRectInit")]
    fn intersection_rect(&self) -> &DomRectInit {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("intersectionRect"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "DomRectInit")]
    fn root_bounds(&self) -> &DomRectInit {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("rootBounds"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "Element")]
    fn target(&self) -> &Element {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("target"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn time(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("time"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl IntersectionObserverEntryInit {
    #[cfg(all(feature = "DomRectInit", feature = "Element",))]
    #[doc = "Construct a new `IntersectionObserverEntryInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `Element`, `IntersectionObserverEntryInit`*"]
    pub fn new(
        bounding_client_rect: &DomRectInit,
        intersection_rect: &DomRectInit,
        root_bounds: &DomRectInit,
        target: &Element,
        time: f64,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.bounding_client_rect(bounding_client_rect);
        ret.intersection_rect(intersection_rect);
        ret.root_bounds(root_bounds);
        ret.target(target);
        ret.time(time);
        ret
    }
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `boundingClientRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    pub fn bounding_client_rect(&mut self, val: &DomRectInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("boundingClientRect"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `intersectionRect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    pub fn intersection_rect(&mut self, val: &DomRectInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("intersectionRect"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "DomRectInit")]
    #[doc = "Change the `rootBounds` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectInit`, `IntersectionObserverEntryInit`*"]
    pub fn root_bounds(&mut self, val: &DomRectInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rootBounds"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "Element")]
    #[doc = "Change the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `IntersectionObserverEntryInit`*"]
    pub fn target(&mut self, val: &Element) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("target"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `time` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IntersectionObserverEntryInit`*"]
    pub fn time(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("time"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
