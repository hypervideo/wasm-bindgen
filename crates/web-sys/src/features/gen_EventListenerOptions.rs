#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EventListenerOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EventListenerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
    pub type EventListenerOptions;
    #[wasm_bindgen(method, getter = "capture")]
    fn capture_shim(this: &EventListenerOptions) -> bool;
    #[wasm_bindgen(method, setter = "capture")]
    fn set_capture_shim(this: &EventListenerOptions, val: bool);
}
#[doc = "The trait to access properties on the `EventListenerOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
pub trait EventListenerOptionsGetters {
    #[doc = "Get the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
    fn capture(&self) -> bool;
}
impl EventListenerOptionsGetters for EventListenerOptions {
    fn capture(&self) -> bool {
        self.capture_shim()
    }
}
impl EventListenerOptions {
    #[doc = "Construct a new `EventListenerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `capture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventListenerOptions`*"]
    pub fn capture(&mut self, val: bool) -> &mut Self {
        self.set_capture_shim(val);
        self
    }
}
impl Default for EventListenerOptions {
    fn default() -> Self {
        Self::new()
    }
}
