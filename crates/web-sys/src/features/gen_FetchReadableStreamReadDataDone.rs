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
    #[wasm_bindgen(method, getter = "done")]
    fn done_shim(this: &FetchReadableStreamReadDataDone) -> bool;
    #[wasm_bindgen(method, setter = "done")]
    fn set_done_shim(this: &FetchReadableStreamReadDataDone, val: bool);
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
        self.done_shim()
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
        self.set_done_shim(val);
        self
    }
}
impl Default for FetchReadableStreamReadDataDone {
    fn default() -> Self {
        Self::new()
    }
}
