#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = USBPermissionStorage)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UsbPermissionStorage` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type UsbPermissionStorage;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `UsbPermissionStorage` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `UsbPermissionStorage`*"]
pub trait UsbPermissionStorageGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `allowedDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn allowed_devices(&self) -> &::wasm_bindgen::JsValue;
}
#[cfg(web_sys_unstable_apis)]
impl UsbPermissionStorageGetters for UsbPermissionStorage {
    #[cfg(web_sys_unstable_apis)]
    fn allowed_devices(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("allowedDevices"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
#[cfg(web_sys_unstable_apis)]
impl UsbPermissionStorage {
    #[doc = "Construct a new `UsbPermissionStorage`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `allowedDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UsbPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn allowed_devices(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("allowedDevices"),
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
impl Default for UsbPermissionStorage {
    fn default() -> Self {
        Self::new()
    }
}
