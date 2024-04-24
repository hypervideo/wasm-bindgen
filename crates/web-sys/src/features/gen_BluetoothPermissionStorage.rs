#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = BluetoothPermissionStorage)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BluetoothPermissionStorage` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type BluetoothPermissionStorage;
    #[wasm_bindgen(method, getter = "allowedDevices")]
    fn allowed_devices_shim(this: &BluetoothPermissionStorage) -> ::js_sys::Array;
    #[wasm_bindgen(method, setter = "allowedDevices")]
    fn set_allowed_devices_shim(this: &BluetoothPermissionStorage, val: &::wasm_bindgen::JsValue);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `BluetoothPermissionStorage` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionStorage`*"]
pub trait BluetoothPermissionStorageGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `allowedDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn allowed_devices(&self) -> ::js_sys::Array;
}
#[cfg(web_sys_unstable_apis)]
impl BluetoothPermissionStorageGetters for BluetoothPermissionStorage {
    #[cfg(web_sys_unstable_apis)]
    fn allowed_devices(&self) -> ::js_sys::Array {
        self.allowed_devices_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl BluetoothPermissionStorage {
    #[doc = "Construct a new `BluetoothPermissionStorage`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(allowed_devices: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.allowed_devices(allowed_devices);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `allowedDevices` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BluetoothPermissionStorage`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn allowed_devices(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_allowed_devices_shim(val);
        self
    }
}
