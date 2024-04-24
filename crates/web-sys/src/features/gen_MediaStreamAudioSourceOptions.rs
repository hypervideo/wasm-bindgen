#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaStreamAudioSourceOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaStreamAudioSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStreamAudioSourceOptions`*"]
    pub type MediaStreamAudioSourceOptions;
}
#[doc = "The trait to access properties on the `MediaStreamAudioSourceOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaStreamAudioSourceOptions`*"]
pub trait MediaStreamAudioSourceOptionsGetters {
    #[cfg(feature = "MediaStream")]
    #[doc = "Get the `mediaStream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
    fn media_stream(&self) -> &MediaStream;
}
impl MediaStreamAudioSourceOptionsGetters for MediaStreamAudioSourceOptions {
    #[cfg(feature = "MediaStream")]
    fn media_stream(&self) -> &MediaStream {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("mediaStream"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl MediaStreamAudioSourceOptions {
    #[cfg(feature = "MediaStream")]
    #[doc = "Construct a new `MediaStreamAudioSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
    pub fn new(media_stream: &MediaStream) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.media_stream(media_stream);
        ret
    }
    #[cfg(feature = "MediaStream")]
    #[doc = "Change the `mediaStream` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaStream`, `MediaStreamAudioSourceOptions`*"]
    pub fn media_stream(&mut self, val: &MediaStream) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("mediaStream"),
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
