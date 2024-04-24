#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PlaneLayout)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PlaneLayout` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type PlaneLayout;
    #[wasm_bindgen(method, getter = "offset")]
    fn offset_shim(this: &PlaneLayout) -> u32;
    #[wasm_bindgen(method, setter = "offset")]
    fn set_offset_shim(this: &PlaneLayout, val: u32);
    #[wasm_bindgen(method, getter = "stride")]
    fn stride_shim(this: &PlaneLayout) -> u32;
    #[wasm_bindgen(method, setter = "stride")]
    fn set_stride_shim(this: &PlaneLayout, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `PlaneLayout` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
pub trait PlaneLayoutGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn offset(&self) -> u32;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `stride` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn stride(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl PlaneLayoutGetters for PlaneLayout {
    #[cfg(web_sys_unstable_apis)]
    fn offset(&self) -> u32 {
        self.offset_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn stride(&self) -> u32 {
        self.stride_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl PlaneLayout {
    #[doc = "Construct a new `PlaneLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(offset: u32, stride: u32) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::offset(&mut ret, offset);
        Self::stride(&mut ret, stride);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn offset(&mut self, val: u32) -> &mut Self {
        self.set_offset_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `stride` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PlaneLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn stride(&mut self, val: u32) -> &mut Self {
        self.set_stride_shim(val);
        self
    }
}
