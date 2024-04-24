#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = IsInputPendingOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IsInputPendingOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type IsInputPendingOptions;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `IsInputPendingOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
pub trait IsInputPendingOptionsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `includeContinuous` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn include_continuous(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl IsInputPendingOptionsGetters for IsInputPendingOptions {
    #[cfg(web_sys_unstable_apis)]
    fn include_continuous(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("includeContinuous"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
#[cfg(web_sys_unstable_apis)]
impl IsInputPendingOptions {
    #[doc = "Construct a new `IsInputPendingOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `includeContinuous` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IsInputPendingOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn include_continuous(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("includeContinuous"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for IsInputPendingOptions {
    fn default() -> Self {
        Self::new()
    }
}
