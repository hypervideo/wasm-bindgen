#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FetchReadableStreamReadDataDone)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchReadableStreamReadDataDone` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
    pub type FetchReadableStreamReadDataDone;
}
#[doc = "The trait to access properties on the `FetchReadableStreamReadDataDone` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
pub trait FetchReadableStreamReadDataDoneGetters {
    #[doc = "Get the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
    fn done(&self) -> bool;
}
impl FetchReadableStreamReadDataDoneGetters for FetchReadableStreamReadDataDone {
    fn done(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("done"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl FetchReadableStreamReadDataDone {
    #[doc = "Construct a new `FetchReadableStreamReadDataDone`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `done` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchReadableStreamReadDataDone`*"]
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
}
impl Default for FetchReadableStreamReadDataDone {
    fn default() -> Self {
        Self::new()
    }
}
