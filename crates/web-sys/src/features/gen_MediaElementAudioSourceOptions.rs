#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaElementAudioSourceOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaElementAudioSourceOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaElementAudioSourceOptions`*"]
    pub type MediaElementAudioSourceOptions;
}
#[doc = "The trait to access properties on the `MediaElementAudioSourceOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaElementAudioSourceOptions`*"]
pub trait MediaElementAudioSourceOptionsGetters {
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Get the `mediaElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
    fn media_element(&self) -> &HtmlMediaElement;
}
impl MediaElementAudioSourceOptionsGetters for MediaElementAudioSourceOptions {
    #[cfg(feature = "HtmlMediaElement")]
    fn media_element(&self) -> &HtmlMediaElement {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("mediaElement"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl MediaElementAudioSourceOptions {
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Construct a new `MediaElementAudioSourceOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
    pub fn new(media_element: &HtmlMediaElement) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.media_element(media_element);
        ret
    }
    #[cfg(feature = "HtmlMediaElement")]
    #[doc = "Change the `mediaElement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlMediaElement`, `MediaElementAudioSourceOptions`*"]
    pub fn media_element(&mut self, val: &HtmlMediaElement) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("mediaElement"),
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
