#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UnderlyingSink)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UnderlyingSink` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub type UnderlyingSink;
}
#[doc = "The trait to access properties on the `UnderlyingSink` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
pub trait UnderlyingSinkGetters {
    #[doc = "Get the `abort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    fn abort(&self) -> &::js_sys::Function;
    #[doc = "Get the `close` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    fn close(&self) -> &::js_sys::Function;
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    fn start(&self) -> &::js_sys::Function;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    fn type_(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    fn write(&self) -> &::js_sys::Function;
}
impl UnderlyingSinkGetters for UnderlyingSink {
    fn abort(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("abort"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn close(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("close"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn start(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("start"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn type_(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn write(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("write"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl UnderlyingSink {
    #[doc = "Construct a new `UnderlyingSink`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `abort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn abort(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("abort"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `close` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn close(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("close"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("start"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn type_(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `write` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSink`*"]
    pub fn write(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("write"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for UnderlyingSink {
    fn default() -> Self {
        Self::new()
    }
}
