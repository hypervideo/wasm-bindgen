#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = StreamPipeOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StreamPipeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub type StreamPipeOptions;
}
#[doc = "The trait to access properties on the `StreamPipeOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
pub trait StreamPipeOptionsGetters {
    #[doc = "Get the `preventAbort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    fn prevent_abort(&self) -> bool;
    #[doc = "Get the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    fn prevent_cancel(&self) -> bool;
    #[doc = "Get the `preventClose` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    fn prevent_close(&self) -> bool;
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `StreamPipeOptions`*"]
    fn signal(&self) -> &AbortSignal;
}
impl StreamPipeOptionsGetters for StreamPipeOptions {
    fn prevent_abort(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("preventAbort"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn prevent_cancel(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("preventCancel"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn prevent_close(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("preventClose"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "AbortSignal")]
    fn signal(&self) -> &AbortSignal {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("signal"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl StreamPipeOptions {
    #[doc = "Construct a new `StreamPipeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `preventAbort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub fn prevent_abort(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("preventAbort"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub fn prevent_cancel(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("preventCancel"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `preventClose` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
    pub fn prevent_close(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("preventClose"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `StreamPipeOptions`*"]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("signal"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for StreamPipeOptions {
    fn default() -> Self {
        Self::new()
    }
}
