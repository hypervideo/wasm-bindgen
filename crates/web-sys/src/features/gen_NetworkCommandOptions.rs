#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NetworkCommandOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NetworkCommandOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub type NetworkCommandOptions;
}
#[doc = "The trait to access properties on the `NetworkCommandOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
pub trait NetworkCommandOptionsGetters {
    #[doc = "Get the `cmd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn cmd(&self) -> &str;
    #[doc = "Get the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn cur_external_ifname(&self) -> &str;
    #[doc = "Get the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn cur_internal_ifname(&self) -> &str;
    #[doc = "Get the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn dns1(&self) -> &str;
    #[doc = "Get the `dns1_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn dns1_long(&self) -> i32;
    #[doc = "Get the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn dns2(&self) -> &str;
    #[doc = "Get the `dns2_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn dns2_long(&self) -> i32;
    #[doc = "Get the `dnses` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn dnses(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn domain(&self) -> &str;
    #[doc = "Get the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn enable(&self) -> bool;
    #[doc = "Get the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn enabled(&self) -> bool;
    #[doc = "Get the `endIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn end_ip(&self) -> &str;
    #[doc = "Get the `externalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn external_ifname(&self) -> &str;
    #[doc = "Get the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn gateway(&self) -> &str;
    #[doc = "Get the `gateway_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn gateway_long(&self) -> i32;
    #[doc = "Get the `gateways` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn gateways(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn id(&self) -> i32;
    #[doc = "Get the `ifname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn ifname(&self) -> &str;
    #[doc = "Get the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn interface_list(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `internalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn internal_ifname(&self) -> &str;
    #[doc = "Get the `ip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn ip(&self) -> &str;
    #[doc = "Get the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn ipaddr(&self) -> i32;
    #[doc = "Get the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn key(&self) -> &str;
    #[doc = "Get the `link` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn link(&self) -> &str;
    #[doc = "Get the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn mask(&self) -> i32;
    #[doc = "Get the `maskLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn mask_length(&self) -> &str;
    #[doc = "Get the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn mode(&self) -> &str;
    #[doc = "Get the `mtu` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn mtu(&self) -> i32;
    #[doc = "Get the `preExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn pre_external_ifname(&self) -> &str;
    #[doc = "Get the `preInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn pre_internal_ifname(&self) -> &str;
    #[doc = "Get the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn prefix(&self) -> &str;
    #[doc = "Get the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn prefix_length(&self) -> u32;
    #[doc = "Get the `report` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn report(&self) -> bool;
    #[doc = "Get the `security` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn security(&self) -> &str;
    #[doc = "Get the `serverIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn server_ip(&self) -> &str;
    #[doc = "Get the `ssid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn ssid(&self) -> &str;
    #[doc = "Get the `startIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn start_ip(&self) -> &str;
    #[doc = "Get the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn threshold(&self) -> f64;
    #[doc = "Get the `usbEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn usb_end_ip(&self) -> &str;
    #[doc = "Get the `usbStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn usb_start_ip(&self) -> &str;
    #[doc = "Get the `wifiEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn wifi_end_ip(&self) -> &str;
    #[doc = "Get the `wifiStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn wifi_start_ip(&self) -> &str;
    #[doc = "Get the `wifictrlinterfacename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    fn wifictrlinterfacename(&self) -> &str;
}
impl NetworkCommandOptionsGetters for NetworkCommandOptions {
    fn cmd(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("cmd"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn cur_external_ifname(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("curExternalIfname"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn cur_internal_ifname(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("curInternalIfname"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn dns1(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dns1"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn dns1_long(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dns1_long"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn dns2(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dns2"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn dns2_long(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dns2_long"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn dnses(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dnses"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn domain(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("domain"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn enable(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("enable"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn enabled(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("enabled"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn end_ip(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("endIp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn external_ifname(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("externalIfname"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn gateway(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("gateway"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn gateway_long(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("gateway_long"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn gateways(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("gateways"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn id(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("id"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ifname(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ifname"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn interface_list(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("interfaceList"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn internal_ifname(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("internalIfname"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ip(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ip"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ipaddr(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ipaddr"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn key(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("key"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn link(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("link"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn mask(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("mask"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn mask_length(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("maskLength"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn mode(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("mode"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn mtu(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("mtu"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn pre_external_ifname(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("preExternalIfname"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn pre_internal_ifname(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("preInternalIfname"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn prefix(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("prefix"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn prefix_length(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("prefixLength"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn report(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("report"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn security(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("security"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn server_ip(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("serverIp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ssid(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ssid"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn start_ip(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("startIp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn threshold(&self) -> f64 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("threshold"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn usb_end_ip(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("usbEndIp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn usb_start_ip(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("usbStartIp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn wifi_end_ip(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("wifiEndIp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn wifi_start_ip(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("wifiStartIp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn wifictrlinterfacename(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("wifictrlinterfacename"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl NetworkCommandOptions {
    #[doc = "Construct a new `NetworkCommandOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `cmd` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn cmd(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("cmd"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn cur_external_ifname(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("curExternalIfname"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn cur_internal_ifname(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("curInternalIfname"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns1(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("dns1"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `dns1_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns1_long(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("dns1_long"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns2(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("dns2"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `dns2_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dns2_long(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("dns2_long"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `dnses` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn dnses(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("dnses"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `domain` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn domain(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("domain"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn enable(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("enable"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `enabled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn enabled(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("enabled"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `endIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn end_ip(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("endIp"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `externalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn external_ifname(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("externalIfname"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateway(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("gateway"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `gateway_long` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateway_long(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("gateway_long"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `gateways` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn gateways(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("gateways"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn id(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("id"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `ifname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ifname(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ifname"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn interface_list(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("interfaceList"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `internalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn internal_ifname(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("internalIfname"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `ip` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ip(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ip"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ipaddr(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ipaddr"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `key` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn key(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("key"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `link` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn link(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("link"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mask(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("mask"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `maskLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mask_length(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("maskLength"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `mode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mode(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("mode"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `mtu` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn mtu(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("mtu"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `preExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn pre_external_ifname(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("preExternalIfname"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `preInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn pre_internal_ifname(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("preInternalIfname"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `prefix` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn prefix(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("prefix"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn prefix_length(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("prefixLength"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `report` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn report(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("report"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `security` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn security(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("security"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `serverIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn server_ip(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("serverIp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `ssid` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn ssid(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ssid"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `startIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn start_ip(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("startIp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `threshold` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn threshold(&mut self, val: f64) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("threshold"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `usbEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn usb_end_ip(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("usbEndIp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `usbStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn usb_start_ip(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("usbStartIp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `wifiEndIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifi_end_ip(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("wifiEndIp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `wifiStartIp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifi_start_ip(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("wifiStartIp"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `wifictrlinterfacename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkCommandOptions`*"]
    pub fn wifictrlinterfacename(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("wifictrlinterfacename"),
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
impl Default for NetworkCommandOptions {
    fn default() -> Self {
        Self::new()
    }
}
