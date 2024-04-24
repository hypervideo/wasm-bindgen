#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUImageCopyExternalImage)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuImageCopyExternalImage` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyExternalImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuImageCopyExternalImage;
    #[wasm_bindgen(method, getter = "flipY")]
    fn flip_y_shim(this: &GpuImageCopyExternalImage) -> bool;
    #[wasm_bindgen(method, setter = "flipY")]
    fn set_flip_y_shim(this: &GpuImageCopyExternalImage, val: bool);
    #[wasm_bindgen(method, getter = "origin")]
    fn origin_shim(this: &GpuImageCopyExternalImage) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin_shim(this: &GpuImageCopyExternalImage, val: &::wasm_bindgen::JsValue);
    #[wasm_bindgen(method, getter = "source")]
    fn source_shim(this: &GpuImageCopyExternalImage) -> ::js_sys::Object;
    #[wasm_bindgen(method, setter = "source")]
    fn set_source_shim(this: &GpuImageCopyExternalImage, val: &::js_sys::Object);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuImageCopyExternalImage` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuImageCopyExternalImage`*"]
pub trait GpuImageCopyExternalImageGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `flipY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyExternalImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn flip_y(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyExternalImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn origin(&self) -> ::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyExternalImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn source(&self) -> ::js_sys::Object;
}
#[cfg(web_sys_unstable_apis)]
impl GpuImageCopyExternalImageGetters for GpuImageCopyExternalImage {
    #[cfg(web_sys_unstable_apis)]
    fn flip_y(&self) -> bool {
        self.flip_y_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn origin(&self) -> ::wasm_bindgen::JsValue {
        self.origin_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn source(&self) -> ::js_sys::Object {
        self.source_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuImageCopyExternalImage {
    #[doc = "Construct a new `GpuImageCopyExternalImage`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyExternalImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(source: &::js_sys::Object) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.source(source);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `flipY` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyExternalImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn flip_y(&mut self, val: bool) -> &mut Self {
        self.set_flip_y_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyExternalImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn origin(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_origin_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyExternalImage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn source(&mut self, val: &::js_sys::Object) -> &mut Self {
        self.set_source_shim(val);
        self
    }
}
