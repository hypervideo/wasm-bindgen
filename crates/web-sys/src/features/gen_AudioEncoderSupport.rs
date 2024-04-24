#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AudioEncoderSupport)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AudioEncoderSupport` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AudioEncoderSupport;
    #[cfg(feature = "AudioEncoderConfig")]
    #[wasm_bindgen(method, getter = "config")]
    fn config_shim(this: &AudioEncoderSupport) -> AudioEncoderConfig;
    #[cfg(feature = "AudioEncoderConfig")]
    #[wasm_bindgen(method, setter = "config")]
    fn set_config_shim(this: &AudioEncoderSupport, val: &AudioEncoderConfig);
    #[wasm_bindgen(method, getter = "supported")]
    fn supported_shim(this: &AudioEncoderSupport) -> bool;
    #[wasm_bindgen(method, setter = "supported")]
    fn set_supported_shim(this: &AudioEncoderSupport, val: bool);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `AudioEncoderSupport` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AudioEncoderSupport`*"]
pub trait AudioEncoderSupportGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioEncoderConfig")]
    #[doc = "Get the `config` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`, `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn config(&self) -> AudioEncoderConfig;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn supported(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl AudioEncoderSupportGetters for AudioEncoderSupport {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioEncoderConfig")]
    fn config(&self) -> AudioEncoderConfig {
        self.config_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn supported(&self) -> bool {
        self.supported_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl AudioEncoderSupport {
    #[doc = "Construct a new `AudioEncoderSupport`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AudioEncoderConfig")]
    #[doc = "Change the `config` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderConfig`, `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn config(&mut self, val: &AudioEncoderConfig) -> &mut Self {
        self.set_config_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `supported` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AudioEncoderSupport`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn supported(&mut self, val: bool) -> &mut Self {
        self.set_supported_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for AudioEncoderSupport {
    fn default() -> Self {
        Self::new()
    }
}
