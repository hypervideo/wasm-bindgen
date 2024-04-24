#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ImageDecodeResult)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageDecodeResult` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type ImageDecodeResult;
    #[wasm_bindgen(method, getter = "complete")]
    fn complete_shim(this: &ImageDecodeResult) -> bool;
    #[wasm_bindgen(method, setter = "complete")]
    fn set_complete_shim(this: &ImageDecodeResult, val: bool);
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(method, getter = "image")]
    fn image_shim(this: &ImageDecodeResult) -> VideoFrame;
    #[cfg(feature = "VideoFrame")]
    #[wasm_bindgen(method, setter = "image")]
    fn set_image_shim(this: &ImageDecodeResult, val: &VideoFrame);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `ImageDecodeResult` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`*"]
pub trait ImageDecodeResultGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `complete` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn complete(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "Get the `image` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn image(&self) -> VideoFrame;
}
#[cfg(web_sys_unstable_apis)]
impl ImageDecodeResultGetters for ImageDecodeResult {
    #[cfg(web_sys_unstable_apis)]
    fn complete(&self) -> bool {
        self.complete_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    fn image(&self) -> VideoFrame {
        self.image_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl ImageDecodeResult {
    #[cfg(feature = "VideoFrame")]
    #[doc = "Construct a new `ImageDecodeResult`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(complete: bool, image: &VideoFrame) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.complete(complete);
        ret.image(image);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `complete` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn complete(&mut self, val: bool) -> &mut Self {
        self.set_complete_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "Change the `image` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageDecodeResult`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn image(&mut self, val: &VideoFrame) -> &mut Self {
        self.set_image_shim(val);
        self
    }
}
