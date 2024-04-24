#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DOMQuadJSON)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomQuadJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadJson`*"]
    pub type DomQuadJson;
}
#[doc = "The trait to access properties on the `DomQuadJson` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DomQuadJson`*"]
pub trait DomQuadJsonGetters {
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    fn p1(&self) -> &DomPoint;
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    fn p2(&self) -> &DomPoint;
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    fn p3(&self) -> &DomPoint;
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    fn p4(&self) -> &DomPoint;
}
impl DomQuadJsonGetters for DomQuadJson {
    #[cfg(feature = "DomPoint")]
    fn p1(&self) -> &DomPoint {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("p1"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "DomPoint")]
    fn p2(&self) -> &DomPoint {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("p2"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "DomPoint")]
    fn p3(&self) -> &DomPoint {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("p3"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "DomPoint")]
    fn p4(&self) -> &DomPoint {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("p4"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl DomQuadJson {
    #[doc = "Construct a new `DomQuadJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadJson`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    pub fn p1(&mut self, val: &DomPoint) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("p1"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    pub fn p2(&mut self, val: &DomPoint) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("p2"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    pub fn p3(&mut self, val: &DomPoint) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("p3"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
    pub fn p4(&mut self, val: &DomPoint) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("p4"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for DomQuadJson {
    fn default() -> Self {
        Self::new()
    }
}
