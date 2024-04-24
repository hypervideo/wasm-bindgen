#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaRecorderOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaRecorderOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub type MediaRecorderOptions;
}
#[doc = "The trait to access properties on the `MediaRecorderOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
pub trait MediaRecorderOptionsGetters {
    #[doc = "Get the `audioBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    fn audio_bits_per_second(&self) -> u32;
    #[doc = "Get the `bitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    fn bits_per_second(&self) -> u32;
    #[doc = "Get the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    fn mime_type(&self) -> &str;
    #[doc = "Get the `videoBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    fn video_bits_per_second(&self) -> u32;
}
impl MediaRecorderOptionsGetters for MediaRecorderOptions {
    fn audio_bits_per_second(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("audioBitsPerSecond"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn bits_per_second(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("bitsPerSecond"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn mime_type(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("mimeType"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn video_bits_per_second(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("videoBitsPerSecond"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl MediaRecorderOptions {
    #[doc = "Construct a new `MediaRecorderOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `audioBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub fn audio_bits_per_second(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("audioBitsPerSecond"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `bitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub fn bits_per_second(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("bitsPerSecond"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `mimeType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub fn mime_type(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("mimeType"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `videoBitsPerSecond` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaRecorderOptions`*"]
    pub fn video_bits_per_second(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("videoBitsPerSecond"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for MediaRecorderOptions {
    fn default() -> Self {
        Self::new()
    }
}
