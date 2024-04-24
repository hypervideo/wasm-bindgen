#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUPipelineErrorInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuPipelineErrorInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuPipelineErrorInit;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `GpuPipelineErrorInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`*"]
pub trait GpuPipelineErrorInitGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`, `GpuPipelineErrorReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn reason(&self) -> GpuPipelineErrorReason;
}
#[cfg(web_sys_unstable_apis)]
impl GpuPipelineErrorInitGetters for GpuPipelineErrorInit {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorReason")]
    fn reason(&self) -> GpuPipelineErrorReason {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("reason"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
#[cfg(web_sys_unstable_apis)]
impl GpuPipelineErrorInit {
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[doc = "Construct a new `GpuPipelineErrorInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`, `GpuPipelineErrorReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(reason: GpuPipelineErrorReason) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.reason(reason);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuPipelineErrorReason")]
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuPipelineErrorInit`, `GpuPipelineErrorReason`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn reason(&mut self, val: GpuPipelineErrorReason) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("reason"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
