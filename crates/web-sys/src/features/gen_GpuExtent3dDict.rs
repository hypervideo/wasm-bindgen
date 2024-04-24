#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUExtent3DDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuExtent3dDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuExtent3dDict;
    #[wasm_bindgen(method, getter = "depthOrArrayLayers")]
    fn depth_or_array_layers_shim(this: &GpuExtent3dDict) -> u32;
    #[wasm_bindgen(method, setter = "depthOrArrayLayers")]
    fn set_depth_or_array_layers_shim(this: &GpuExtent3dDict, val: u32);
    #[wasm_bindgen(method, getter = "height")]
    fn height_shim(this: &GpuExtent3dDict) -> u32;
    #[wasm_bindgen(method, setter = "height")]
    fn set_height_shim(this: &GpuExtent3dDict, val: u32);
    #[wasm_bindgen(method, getter = "width")]
    fn width_shim(this: &GpuExtent3dDict) -> u32;
    #[wasm_bindgen(method, setter = "width")]
    fn set_width_shim(this: &GpuExtent3dDict, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuExtent3dDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`*"]
pub trait GpuExtent3dDictGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `depthOrArrayLayers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn depth_or_array_layers(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn height(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn width(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl GpuExtent3dDictGetters for GpuExtent3dDict {
    #[cfg(web_sys_unstable_apis)]
    fn depth_or_array_layers(&self) -> u32 {
        self.depth_or_array_layers_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn height(&self) -> u32 {
        self.height_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn width(&self) -> u32 {
        self.width_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuExtent3dDict {
    #[doc = "Construct a new `GpuExtent3dDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(width: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.width(width);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `depthOrArrayLayers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn depth_or_array_layers(&mut self, val: u32) -> &mut Self {
        self.set_depth_or_array_layers_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `height` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn height(&mut self, val: u32) -> &mut Self {
        self.set_height_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `width` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuExtent3dDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn width(&mut self, val: u32) -> &mut Self {
        self.set_width_shim(val);
        self
    }
}
