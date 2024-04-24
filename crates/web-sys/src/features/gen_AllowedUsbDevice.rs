#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AllowedUSBDevice)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AllowedUsbDevice` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type AllowedUsbDevice;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `AllowedUsbDevice` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
pub trait AllowedUsbDeviceGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn product_id(&self) -> u8;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn serial_number(&self) -> &str;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn vendor_id(&self) -> u8;
}
#[cfg(web_sys_unstable_apis)]
impl AllowedUsbDeviceGetters for AllowedUsbDevice {
    #[cfg(web_sys_unstable_apis)]
    fn product_id(&self) -> u8 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("productId"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    fn serial_number(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("serialNumber"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    fn vendor_id(&self) -> u8 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("vendorId"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
#[cfg(web_sys_unstable_apis)]
impl AllowedUsbDevice {
    #[doc = "Construct a new `AllowedUsbDevice`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(product_id: u8, vendor_id: u8) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.product_id(product_id);
        ret.vendor_id(vendor_id);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `productId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn product_id(&mut self, val: u8) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("productId"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `serialNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn serial_number(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("serialNumber"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `vendorId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AllowedUsbDevice`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn vendor_id(&mut self, val: u8) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("vendorId"),
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
