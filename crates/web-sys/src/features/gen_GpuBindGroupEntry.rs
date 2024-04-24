#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUBindGroupEntry)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuBindGroupEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuBindGroupEntry;
    #[wasm_bindgen(method, getter = "binding")]
    fn binding_shim(this: &GpuBindGroupEntry) -> u32;
    #[wasm_bindgen(method, setter = "binding")]
    fn set_binding_shim(this: &GpuBindGroupEntry, val: u32);
    #[wasm_bindgen(method, getter = "resource")]
    fn resource_shim(this: &GpuBindGroupEntry) -> ::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "resource")]
    fn set_resource_shim(this: &GpuBindGroupEntry, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuBindGroupEntry` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuBindGroupEntry`*"]
pub trait GpuBindGroupEntryGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `binding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn binding(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `resource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn resource(&self) -> ::wasm_bindgen::JsValue;
}
#[cfg(web_sys_unstable_apis)]
impl GpuBindGroupEntryGetters for GpuBindGroupEntry {
    #[cfg(web_sys_unstable_apis)]
    fn binding(&self) -> u32 {
        self.binding_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn resource(&self) -> ::wasm_bindgen::JsValue {
        self.resource_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuBindGroupEntry {
    #[doc = "Construct a new `GpuBindGroupEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(binding: u32, resource: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::binding(&mut ret, binding);
        Self::resource(&mut ret, resource);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `binding` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn binding(&mut self, val: u32) -> &mut Self {
        self.set_binding_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `resource` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuBindGroupEntry`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn resource(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_resource_shim(val);
        self
    }
}
