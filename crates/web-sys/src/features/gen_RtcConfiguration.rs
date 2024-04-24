#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCConfiguration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub type RtcConfiguration;
}
#[doc = "The trait to access properties on the `RtcConfiguration` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
pub trait RtcConfigurationGetters {
    #[cfg(feature = "RtcBundlePolicy")]
    #[doc = "Get the `bundlePolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcBundlePolicy`, `RtcConfiguration`*"]
    fn bundle_policy(&self) -> RtcBundlePolicy;
    #[doc = "Get the `certificates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    fn certificates(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `iceServers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    fn ice_servers(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[doc = "Get the `iceTransportPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcIceTransportPolicy`*"]
    fn ice_transport_policy(&self) -> RtcIceTransportPolicy;
    #[doc = "Get the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    fn peer_identity(&self) -> Option<&str>;
}
impl RtcConfigurationGetters for RtcConfiguration {
    #[cfg(feature = "RtcBundlePolicy")]
    fn bundle_policy(&self) -> RtcBundlePolicy {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("bundlePolicy"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn certificates(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("certificates"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ice_servers(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("iceServers"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "RtcIceTransportPolicy")]
    fn ice_transport_policy(&self) -> RtcIceTransportPolicy {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("iceTransportPolicy"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn peer_identity(&self) -> Option<&str> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("peerIdentity"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RtcConfiguration {
    #[doc = "Construct a new `RtcConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "RtcBundlePolicy")]
    #[doc = "Change the `bundlePolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcBundlePolicy`, `RtcConfiguration`*"]
    pub fn bundle_policy(&mut self, val: RtcBundlePolicy) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("bundlePolicy"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `certificates` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn certificates(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("certificates"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `iceServers` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn ice_servers(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("iceServers"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "RtcIceTransportPolicy")]
    #[doc = "Change the `iceTransportPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`, `RtcIceTransportPolicy`*"]
    pub fn ice_transport_policy(&mut self, val: RtcIceTransportPolicy) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("iceTransportPolicy"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `peerIdentity` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcConfiguration`*"]
    pub fn peer_identity(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("peerIdentity"),
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
impl Default for RtcConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
