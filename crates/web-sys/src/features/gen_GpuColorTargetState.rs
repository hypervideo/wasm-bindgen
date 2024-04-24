#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUColorTargetState)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuColorTargetState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorTargetState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuColorTargetState;
    #[cfg(feature = "GpuBlendState")]
    #[wasm_bindgen(method, getter = "blend")]
    fn blend_shim(this: &GpuColorTargetState) -> GpuBlendState;
    #[cfg(feature = "GpuBlendState")]
    #[wasm_bindgen(method, setter = "blend")]
    fn set_blend_shim(this: &GpuColorTargetState, val: &GpuBlendState);
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, getter = "format")]
    fn format_shim(this: &GpuColorTargetState) -> GpuTextureFormat;
    #[cfg(feature = "GpuTextureFormat")]
    #[wasm_bindgen(method, setter = "format")]
    fn set_format_shim(this: &GpuColorTargetState, val: GpuTextureFormat);
    #[wasm_bindgen(method, getter = "writeMask")]
    fn write_mask_shim(this: &GpuColorTargetState) -> u32;
    #[wasm_bindgen(method, setter = "writeMask")]
    fn set_write_mask_shim(this: &GpuColorTargetState, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuColorTargetState` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuColorTargetState`*"]
pub trait GpuColorTargetStateGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendState")]
    #[doc = "Get the `blend` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendState`, `GpuColorTargetState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn blend(&self) -> GpuBlendState;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Get the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorTargetState`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn format(&self) -> GpuTextureFormat;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `writeMask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorTargetState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn write_mask(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl GpuColorTargetStateGetters for GpuColorTargetState {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendState")]
    fn blend(&self) -> GpuBlendState {
        self.blend_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    fn format(&self) -> GpuTextureFormat {
        self.format_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn write_mask(&self) -> u32 {
        self.write_mask_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuColorTargetState {
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Construct a new `GpuColorTargetState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorTargetState`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(format: GpuTextureFormat) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::format(&mut ret, format);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuBlendState")]
    #[doc = "Change the `blend` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBlendState`, `GpuColorTargetState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn blend(&mut self, val: &GpuBlendState) -> &mut Self {
        self.set_blend_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuTextureFormat")]
    #[doc = "Change the `format` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorTargetState`, `GpuTextureFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn format(&mut self, val: GpuTextureFormat) -> &mut Self {
        self.set_format_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `writeMask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorTargetState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn write_mask(&mut self, val: u32) -> &mut Self {
        self.set_write_mask_shim(val);
        self
    }
}
