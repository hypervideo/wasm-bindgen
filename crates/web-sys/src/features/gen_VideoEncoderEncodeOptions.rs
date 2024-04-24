#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = VideoEncoderEncodeOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoEncoderEncodeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderEncodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type VideoEncoderEncodeOptions;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `VideoEncoderEncodeOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `VideoEncoderEncodeOptions`*"]
pub trait VideoEncoderEncodeOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `keyFrame` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderEncodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn key_frame(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl VideoEncoderEncodeOptionsGetters for VideoEncoderEncodeOptions {
    #[cfg(web_sys_unstable_apis)]
    fn key_frame(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("keyFrame"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
#[cfg(web_sys_unstable_apis)]
impl VideoEncoderEncodeOptions {
    #[doc = "Construct a new `VideoEncoderEncodeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderEncodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `keyFrame` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoEncoderEncodeOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn key_frame(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("keyFrame"),
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
#[cfg(web_sys_unstable_apis)]
impl Default for VideoEncoderEncodeOptions {
    fn default() -> Self {
        Self::new()
    }
}
