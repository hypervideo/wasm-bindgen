#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioSinkOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioSinkOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioSinkOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioSinkOptions;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `AudioSinkOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioSinkOptions`*"]
pub trait AudioSinkOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSinkType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioSinkOptions`, `AudioSinkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn type_(&self) -> AudioSinkType;
}
#[cfg(web_sys_unstable_apis)]
impl AudioSinkOptionsGetters for AudioSinkOptions {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSinkType")]
    fn type_(&self) -> AudioSinkType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
#[cfg(web_sys_unstable_apis)]
impl AudioSinkOptions {
    #[cfg(feature = "AudioSinkType")]
    #[doc = "Construct a new `AudioSinkOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioSinkOptions`, `AudioSinkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(type_: AudioSinkType) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.type_(type_);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioSinkType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioSinkOptions`, `AudioSinkType`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn type_(&mut self, val: AudioSinkType) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
