#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WatchAdvertisementsOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WatchAdvertisementsOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WatchAdvertisementsOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type WatchAdvertisementsOptions;
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, getter = "signal")]
    fn signal_shim(this: &WatchAdvertisementsOptions) -> AbortSignal;
    #[cfg(feature = "AbortSignal")]
    #[wasm_bindgen(method, setter = "signal")]
    fn set_signal_shim(this: &WatchAdvertisementsOptions, val: &AbortSignal);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `WatchAdvertisementsOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WatchAdvertisementsOptions`*"]
pub trait WatchAdvertisementsOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `WatchAdvertisementsOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn signal(&self) -> AbortSignal;
}
#[cfg(web_sys_unstable_apis)]
impl WatchAdvertisementsOptionsGetters for WatchAdvertisementsOptions {
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AbortSignal")]
    fn signal(&self) -> AbortSignal {
        self.signal_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl WatchAdvertisementsOptions {
    #[doc = "Construct a new `WatchAdvertisementsOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WatchAdvertisementsOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `WatchAdvertisementsOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn signal(&mut self, val: &AbortSignal) -> &mut Self {
        self.set_signal_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for WatchAdvertisementsOptions {
    fn default() -> Self {
        Self::new()
    }
}
