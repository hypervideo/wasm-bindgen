#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DateTimeValue)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DateTimeValue` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub type DateTimeValue;
}
#[doc = "The trait to access properties on the `DateTimeValue` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
pub trait DateTimeValueGetters {
    #[doc = "Get the `day` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    fn day(&self) -> i32;
    #[doc = "Get the `hour` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    fn hour(&self) -> i32;
    #[doc = "Get the `minute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    fn minute(&self) -> i32;
    #[doc = "Get the `month` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    fn month(&self) -> i32;
    #[doc = "Get the `year` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    fn year(&self) -> i32;
}
impl DateTimeValueGetters for DateTimeValue {
    fn day(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("day"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn hour(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("hour"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn minute(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("minute"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn month(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("month"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn year(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("year"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl DateTimeValue {
    #[doc = "Construct a new `DateTimeValue`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `day` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn day(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("day"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `hour` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn hour(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("hour"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `minute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn minute(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("minute"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `month` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn month(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("month"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `year` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
    pub fn year(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("year"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for DateTimeValue {
    fn default() -> Self {
        Self::new()
    }
}
