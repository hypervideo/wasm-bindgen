#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DOMQuadInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomQuadInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadInit`*"]
    pub type DomQuadInit;
}
#[doc = "The trait to access properties on the `DomQuadInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DomQuadInit`*"]
pub trait DomQuadInitGetters {
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    fn p1(&self) -> &DomPointInit;
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    fn p2(&self) -> &DomPointInit;
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    fn p3(&self) -> &DomPointInit;
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    fn p4(&self) -> &DomPointInit;
}
impl DomQuadInitGetters for DomQuadInit {
    #[cfg(feature = "DomPointInit")]
    fn p1(&self) -> &DomPointInit {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("p1"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "DomPointInit")]
    fn p2(&self) -> &DomPointInit {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("p2"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "DomPointInit")]
    fn p3(&self) -> &DomPointInit {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("p3"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "DomPointInit")]
    fn p4(&self) -> &DomPointInit {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("p4"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl DomQuadInit {
    #[doc = "Construct a new `DomQuadInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p1(&mut self, val: &DomPointInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("p1"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p2(&mut self, val: &DomPointInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("p2"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p3(&mut self, val: &DomPointInit) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("p3"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
    pub fn p4(&mut self, val: &DomPointInit) -> &mut Self {
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
impl Default for DomQuadInit {
    fn default() -> Self {
        Self::new()
    }
}
