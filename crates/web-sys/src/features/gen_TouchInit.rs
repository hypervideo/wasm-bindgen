#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TouchInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TouchInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub type TouchInit;
}
#[doc = "The trait to access properties on the `TouchInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
pub trait TouchInitGetters {
    #[doc = "Get the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn client_x(&self) -> i32;
    #[doc = "Get the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn client_y(&self) -> i32;
    #[doc = "Get the `force` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn force(&self) -> f32;
    #[doc = "Get the `identifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn identifier(&self) -> i32;
    #[doc = "Get the `pageX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn page_x(&self) -> i32;
    #[doc = "Get the `pageY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn page_y(&self) -> i32;
    #[doc = "Get the `radiusX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn radius_x(&self) -> f32;
    #[doc = "Get the `radiusY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn radius_y(&self) -> f32;
    #[doc = "Get the `rotationAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn rotation_angle(&self) -> f32;
    #[doc = "Get the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn screen_x(&self) -> i32;
    #[doc = "Get the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    fn screen_y(&self) -> i32;
    #[cfg(feature = "EventTarget")]
    #[doc = "Get the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*"]
    fn target(&self) -> &EventTarget;
}
impl TouchInitGetters for TouchInit {
    fn client_x(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("clientX"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn client_y(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("clientY"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn force(&self) -> f32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("force"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn identifier(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("identifier"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn page_x(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("pageX"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn page_y(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("pageY"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn radius_x(&self) -> f32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("radiusX"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn radius_y(&self) -> f32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("radiusY"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn rotation_angle(&self) -> f32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("rotationAngle"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn screen_x(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("screenX"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn screen_y(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("screenY"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "EventTarget")]
    fn target(&self) -> &EventTarget {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("target"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl TouchInit {
    #[cfg(feature = "EventTarget")]
    #[doc = "Construct a new `TouchInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*"]
    pub fn new(identifier: i32, target: &EventTarget) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.identifier(identifier);
        ret.target(target);
        ret
    }
    #[doc = "Change the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn client_x(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("clientX"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn client_y(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("clientY"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `force` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn force(&mut self, val: f32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("force"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `identifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn identifier(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("identifier"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `pageX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn page_x(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("pageX"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `pageY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn page_y(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("pageY"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `radiusX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn radius_x(&mut self, val: f32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("radiusX"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `radiusY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn radius_y(&mut self, val: f32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("radiusY"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `rotationAngle` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn rotation_angle(&mut self, val: f32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("rotationAngle"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn screen_x(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("screenX"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TouchInit`*"]
    pub fn screen_y(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("screenY"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "EventTarget")]
    #[doc = "Change the `target` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `TouchInit`*"]
    pub fn target(&mut self, val: &EventTarget) -> &mut Self {
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
}
