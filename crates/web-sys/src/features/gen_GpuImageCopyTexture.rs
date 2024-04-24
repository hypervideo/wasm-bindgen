#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUImageCopyTexture)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuImageCopyTexture` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuImageCopyTexture;
    #[cfg(feature = "GpuTextureAspect")]
    #[wasm_bindgen(method, getter = "aspect")]
    fn aspect_shim(this: &GpuImageCopyTexture) -> GpuTextureAspect;
    #[cfg(feature = "GpuTextureAspect")]
    #[wasm_bindgen(method, setter = "aspect")]
    fn set_aspect_shim(this: &GpuImageCopyTexture, val: GpuTextureAspect);
    #[wasm_bindgen(method, getter = "mipLevel")]
    fn mip_level_shim(this: &GpuImageCopyTexture) -> u32;
    #[wasm_bindgen(method, setter = "mipLevel")]
    fn set_mip_level_shim(this: &GpuImageCopyTexture, val: u32);
    #[wasm_bindgen(method, getter = "origin")]
    fn origin_shim(this: &GpuImageCopyTexture) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "origin")]
    fn set_origin_shim(this: &GpuImageCopyTexture, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "GpuTexture")]
    #[wasm_bindgen(method, getter = "texture")]
    fn texture_shim(this: &GpuImageCopyTexture) -> &GpuTexture;
    #[cfg(feature = "GpuTexture")]
    #[wasm_bindgen(method, setter = "texture")]
    fn set_texture_shim(this: &GpuImageCopyTexture, val: &GpuTexture);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuImageCopyTexture` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`*"]
pub trait GpuImageCopyTextureGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureAspect")]
    #[doc = "Get the `aspect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`, `GpuTextureAspect`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn aspect(&self) -> GpuTextureAspect;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `mipLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn mip_level(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn origin(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[doc = "Get the `texture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`, `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn texture(&self) -> &GpuTexture;
}
#[cfg(web_sys_unstable_apis)]
impl GpuImageCopyTextureGetters for GpuImageCopyTexture {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureAspect")]
    fn aspect(&self) -> GpuTextureAspect {
        self.aspect_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn mip_level(&self) -> u32 {
        self.mip_level_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn origin(&self) -> &::wasm_bindgen::JsValue {
        self.origin_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    fn texture(&self) -> &GpuTexture {
        self.texture_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuImageCopyTexture {
    #[cfg(feature = "GpuTexture")]
    #[doc = "Construct a new `GpuImageCopyTexture`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`, `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(texture: &GpuTexture) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.texture(texture);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureAspect")]
    #[doc = "Change the `aspect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`, `GpuTextureAspect`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn aspect(&mut self, val: GpuTextureAspect) -> &mut Self {
        self.set_aspect_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `mipLevel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn mip_level(&mut self, val: u32) -> &mut Self {
        self.set_mip_level_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `origin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn origin(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_origin_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTexture")]
    #[doc = "Change the `texture` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuImageCopyTexture`, `GpuTexture`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn texture(&mut self, val: &GpuTexture) -> &mut Self {
        self.set_texture_shim(val);
        self
    }
}
