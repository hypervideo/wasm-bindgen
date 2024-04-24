#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUFragmentState)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuFragmentState` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuFragmentState;
    #[wasm_bindgen(method, getter = "entryPoint")]
    fn entry_point_shim(this: &GpuFragmentState) -> String;
    #[wasm_bindgen(method, setter = "entryPoint")]
    fn set_entry_point_shim(this: &GpuFragmentState, val: &str);
    #[cfg(feature = "GpuShaderModule")]
    #[wasm_bindgen(method, getter = "module")]
    fn module_shim(this: &GpuFragmentState) -> GpuShaderModule;
    #[cfg(feature = "GpuShaderModule")]
    #[wasm_bindgen(method, setter = "module")]
    fn set_module_shim(this: &GpuFragmentState, val: &GpuShaderModule);
    #[wasm_bindgen(method, getter = "targets")]
    fn targets_shim(this: &GpuFragmentState) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "targets")]
    fn set_targets_shim(this: &GpuFragmentState, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuFragmentState` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
pub trait GpuFragmentStateGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn entry_point(&self) -> String;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Get the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn module(&self) -> GpuShaderModule;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `targets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn targets(&self) -> ::js_sys::Array;
}
#[cfg(web_sys_unstable_apis)]
impl GpuFragmentStateGetters for GpuFragmentState {
    #[cfg(web_sys_unstable_apis)]
    fn entry_point(&self) -> String {
        self.entry_point_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    fn module(&self) -> GpuShaderModule {
        self.module_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn targets(&self) -> ::js_sys::Array {
        self.targets_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuFragmentState {
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Construct a new `GpuFragmentState`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(module: &GpuShaderModule, targets: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::module(&mut ret, module);
        Self::targets(&mut ret, targets);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `entryPoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn entry_point(&mut self, val: &str) -> &mut Self {
        self.set_entry_point_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuShaderModule")]
    #[doc = "Change the `module` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`, `GpuShaderModule`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn module(&mut self, val: &GpuShaderModule) -> &mut Self {
        self.set_module_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `targets` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuFragmentState`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn targets(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_targets_shim(val);
        self
    }
}
