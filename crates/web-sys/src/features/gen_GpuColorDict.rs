#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUColorDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuColorDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuColorDict;
    #[wasm_bindgen(method, getter = "a")]
    fn a_shim(this: &GpuColorDict) -> f64;
    #[wasm_bindgen(method, setter = "a")]
    fn set_a_shim(this: &GpuColorDict, val: f64);
    #[wasm_bindgen(method, getter = "b")]
    fn b_shim(this: &GpuColorDict) -> f64;
    #[wasm_bindgen(method, setter = "b")]
    fn set_b_shim(this: &GpuColorDict, val: f64);
    #[wasm_bindgen(method, getter = "g")]
    fn g_shim(this: &GpuColorDict) -> f64;
    #[wasm_bindgen(method, setter = "g")]
    fn set_g_shim(this: &GpuColorDict, val: f64);
    #[wasm_bindgen(method, getter = "r")]
    fn r_shim(this: &GpuColorDict) -> f64;
    #[wasm_bindgen(method, setter = "r")]
    fn set_r_shim(this: &GpuColorDict, val: f64);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuColorDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"]
pub trait GpuColorDictGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn a(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn b(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `g` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn g(&self) -> f64;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `r` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn r(&self) -> f64;
}
#[cfg(web_sys_unstable_apis)]
impl GpuColorDictGetters for GpuColorDict {
    #[cfg(web_sys_unstable_apis)]
    fn a(&self) -> f64 {
        self.a_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn b(&self) -> f64 {
        self.b_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn g(&self) -> f64 {
        self.g_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn r(&self) -> f64 {
        self.r_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuColorDict {
    #[doc = "Construct a new `GpuColorDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(a: f64, b: f64, g: f64, r: f64) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::a(&mut ret, a);
        Self::b(&mut ret, b);
        Self::g(&mut ret, g);
        Self::r(&mut ret, r);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `a` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn a(&mut self, val: f64) -> &mut Self {
        self.set_a_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `b` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn b(&mut self, val: f64) -> &mut Self {
        self.set_b_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `g` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn g(&mut self, val: f64) -> &mut Self {
        self.set_g_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `r` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuColorDict`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn r(&mut self, val: f64) -> &mut Self {
        self.set_r_shim(val);
        self
    }
}
