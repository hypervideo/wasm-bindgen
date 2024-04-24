#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = NetworkResultOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NetworkResultOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub type NetworkResultOptions;
}
#[doc = "The trait to access properties on the `NetworkResultOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
pub trait NetworkResultOptionsGetters {
    #[doc = "Get the `broadcast` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn broadcast(&self) -> bool;
    #[doc = "Get the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn cur_external_ifname(&self) -> &str;
    #[doc = "Get the `curInternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn cur_internal_ifname(&self) -> &str;
    #[doc = "Get the `dns1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn dns1(&self) -> i32;
    #[doc = "Get the `dns1_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn dns1_str(&self) -> &str;
    #[doc = "Get the `dns2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn dns2(&self) -> i32;
    #[doc = "Get the `dns2_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn dns2_str(&self) -> &str;
    #[doc = "Get the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn enable(&self) -> bool;
    #[doc = "Get the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn error(&self) -> bool;
    #[doc = "Get the `flag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn flag(&self) -> &str;
    #[doc = "Get the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn gateway(&self) -> i32;
    #[doc = "Get the `gateway_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn gateway_str(&self) -> &str;
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn id(&self) -> i32;
    #[doc = "Get the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn interface_list(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `ipAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn ip_addr(&self) -> &str;
    #[doc = "Get the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn ipaddr(&self) -> i32;
    #[doc = "Get the `ipaddr_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn ipaddr_str(&self) -> &str;
    #[doc = "Get the `lease` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn lease(&self) -> i32;
    #[doc = "Get the `macAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn mac_addr(&self) -> &str;
    #[doc = "Get the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn mask(&self) -> i32;
    #[doc = "Get the `mask_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn mask_str(&self) -> &str;
    #[doc = "Get the `netId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn net_id(&self) -> &str;
    #[doc = "Get the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn prefix_length(&self) -> i32;
    #[doc = "Get the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn reason(&self) -> &str;
    #[doc = "Get the `reply` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn reply(&self) -> &str;
    #[doc = "Get the `result` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn result(&self) -> bool;
    #[doc = "Get the `resultCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn result_code(&self) -> i32;
    #[doc = "Get the `resultReason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn result_reason(&self) -> &str;
    #[doc = "Get the `ret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn ret(&self) -> bool;
    #[doc = "Get the `route` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn route(&self) -> &str;
    #[doc = "Get the `server` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn server(&self) -> i32;
    #[doc = "Get the `server_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn server_str(&self) -> &str;
    #[doc = "Get the `success` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn success(&self) -> bool;
    #[doc = "Get the `topic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn topic(&self) -> &str;
    #[doc = "Get the `vendor_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    fn vendor_str(&self) -> &str;
}
impl NetworkResultOptionsGetters for NetworkResultOptions {
    fn broadcast(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("broadcast"));
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
    fn dns1(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dns1"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn dns1_str(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dns1_str"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn dns2(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dns2"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn dns2_str(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dns2_str"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn enable(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("enable"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn error(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("error"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn flag(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("flag"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn gateway(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("gateway"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn gateway_str(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("gateway_str"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn id(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("id"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn interface_list(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("interfaceList"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ip_addr(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ipAddr"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ipaddr(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ipaddr"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ipaddr_str(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ipaddr_str"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn lease(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("lease"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn mac_addr(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("macAddr"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn mask(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("mask"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn mask_str(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("mask_str"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn net_id(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("netId"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn prefix_length(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("prefixLength"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn reason(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("reason"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn reply(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("reply"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn result(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("result"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn result_code(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("resultCode"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn result_reason(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("resultReason"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ret(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ret"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn route(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("route"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn server(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("server"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn server_str(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("server_str"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn success(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("success"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn topic(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("topic"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn vendor_str(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("vendor_str"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl NetworkResultOptions {
    #[doc = "Construct a new `NetworkResultOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `broadcast` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn broadcast(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("broadcast"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `curExternalIfname` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns1(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("dns1"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `dns1_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns1_str(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("dns1_str"),
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
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns2(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("dns2"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `dns2_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn dns2_str(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("dns2_str"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `enable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
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
    #[doc = "Change the `error` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn error(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("error"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `flag` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn flag(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("flag"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `gateway` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn gateway(&mut self, val: i32) -> &mut Self {
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
    #[doc = "Change the `gateway_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn gateway_str(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("gateway_str"),
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
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
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
    #[doc = "Change the `interfaceList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
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
    #[doc = "Change the `ipAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ip_addr(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ipAddr"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `ipaddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
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
    #[doc = "Change the `ipaddr_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ipaddr_str(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("ipaddr_str"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `lease` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn lease(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("lease"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `macAddr` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn mac_addr(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("macAddr"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `mask` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
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
    #[doc = "Change the `mask_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn mask_str(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("mask_str"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `netId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn net_id(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("netId"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `prefixLength` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn prefix_length(&mut self, val: i32) -> &mut Self {
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
    #[doc = "Change the `reason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn reason(&mut self, val: &str) -> &mut Self {
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
    #[doc = "Change the `reply` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn reply(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("reply"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `result` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("result"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `resultCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result_code(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("resultCode"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `resultReason` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn result_reason(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("resultReason"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `ret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn ret(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ret"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `route` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn route(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("route"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `server` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn server(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("server"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `server_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn server_str(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("server_str"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `success` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn success(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("success"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `topic` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn topic(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("topic"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `vendor_str` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NetworkResultOptions`*"]
    pub fn vendor_str(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("vendor_str"),
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
impl Default for NetworkResultOptions {
    fn default() -> Self {
        Self::new()
    }
}
