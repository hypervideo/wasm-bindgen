#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = HIDDeviceFilter)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidDeviceFilter` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type HidDeviceFilter;
    #[wasm_bindgen(method, getter = "productId")]
    fn product_id_shim(this: &HidDeviceFilter) -> u16;
    #[wasm_bindgen(method, setter = "productId")]
    fn set_product_id_shim(this: &HidDeviceFilter, val: u16);
    #[wasm_bindgen(method, getter = "usage")]
    fn usage_shim(this: &HidDeviceFilter) -> u16;
    #[wasm_bindgen(method, setter = "usage")]
    fn set_usage_shim(this: &HidDeviceFilter, val: u16);
    #[wasm_bindgen(method, getter = "usagePage")]
    fn usage_page_shim(this: &HidDeviceFilter) -> u16;
    #[wasm_bindgen(method, setter = "usagePage")]
    fn set_usage_page_shim(this: &HidDeviceFilter, val: u16);
    #[wasm_bindgen(method, getter = "vendorId")]
    fn vendor_id_shim(this: &HidDeviceFilter) -> u32;
    #[wasm_bindgen(method, setter = "vendorId")]
    fn set_vendor_id_shim(this: &HidDeviceFilter, val: u32);
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `HidDeviceFilter` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"]
pub trait HidDeviceFilterGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn product_id(&self) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn usage(&self) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `usagePage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn usage_page(&self) -> u16;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn vendor_id(&self) -> u32;
}
#[cfg(web_sys_unstable_apis)]
impl HidDeviceFilterGetters for HidDeviceFilter {
    #[cfg(web_sys_unstable_apis)]
    fn product_id(&self) -> u16 {
        self.product_id_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn usage(&self) -> u16 {
        self.usage_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn usage_page(&self) -> u16 {
        self.usage_page_shim()
    }
    #[cfg(web_sys_unstable_apis)]
    fn vendor_id(&self) -> u32 {
        self.vendor_id_shim()
    }
}
#[cfg(web_sys_unstable_apis)]
impl HidDeviceFilter {
    #[doc = "Construct a new `HidDeviceFilter`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn product_id(&mut self, val: u16) -> &mut Self {
        self.set_product_id_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usage(&mut self, val: u16) -> &mut Self {
        self.set_usage_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `usagePage` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn usage_page(&mut self, val: u16) -> &mut Self {
        self.set_usage_page_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDeviceFilter`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn vendor_id(&mut self, val: u32) -> &mut Self {
        self.set_vendor_id_shim(val);
        self
    }
}
#[cfg(web_sys_unstable_apis)]
impl Default for HidDeviceFilter {
    fn default() -> Self {
        Self::new()
    }
}
