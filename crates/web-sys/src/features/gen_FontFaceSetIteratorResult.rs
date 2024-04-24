#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FontFaceSetIteratorResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSetIteratorResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    pub type FontFaceSetIteratorResult;
}
#[doc = "The trait to access properties on the `FontFaceSetIteratorResult` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
pub trait FontFaceSetIteratorResultGetters {
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    fn done(&self) -> bool;
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    fn value(&self) -> &::wasm_bindgen::JsValue;
}
impl FontFaceSetIteratorResultGetters for FontFaceSetIteratorResult {
    fn done(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("done"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn value(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("value"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl FontFaceSetIteratorResult {
    #[doc = "Construct a new `FontFaceSetIteratorResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    pub fn new(done: bool, value: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.done(done);
        ret.value(value);
        ret
    }
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    pub fn done(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("done"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetIteratorResult`*"]
    pub fn value(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("value"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
