#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MediaKeySystemConfiguration)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeySystemConfiguration` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub type MediaKeySystemConfiguration;
}
#[doc = "The trait to access properties on the `MediaKeySystemConfiguration` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
pub trait MediaKeySystemConfigurationGetters {
    #[doc = "Get the `audioCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    fn audio_capabilities(&self) -> &::wasm_bindgen::JsValue;
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Get the `distinctiveIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    fn distinctive_identifier(&self) -> MediaKeysRequirement;
    #[doc = "Get the `initDataTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    fn init_data_types(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    fn label(&self) -> &str;
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Get the `persistentState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    fn persistent_state(&self) -> MediaKeysRequirement;
    #[doc = "Get the `sessionTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    fn session_types(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `videoCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    fn video_capabilities(&self) -> &::wasm_bindgen::JsValue;
}
impl MediaKeySystemConfigurationGetters for MediaKeySystemConfiguration {
    fn audio_capabilities(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("audioCapabilities"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "MediaKeysRequirement")]
    fn distinctive_identifier(&self) -> MediaKeysRequirement {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("distinctiveIdentifier"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn init_data_types(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("initDataTypes"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn label(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("label"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "MediaKeysRequirement")]
    fn persistent_state(&self) -> MediaKeysRequirement {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("persistentState"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn session_types(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("sessionTypes"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn video_capabilities(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("videoCapabilities"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl MediaKeySystemConfiguration {
    #[doc = "Construct a new `MediaKeySystemConfiguration`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `audioCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn audio_capabilities(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("audioCapabilities"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Change the `distinctiveIdentifier` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    pub fn distinctive_identifier(&mut self, val: MediaKeysRequirement) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("distinctiveIdentifier"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `initDataTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn init_data_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("initDataTypes"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `label` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn label(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("label"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "MediaKeysRequirement")]
    #[doc = "Change the `persistentState` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`, `MediaKeysRequirement`*"]
    pub fn persistent_state(&mut self, val: MediaKeysRequirement) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("persistentState"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `sessionTypes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn session_types(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("sessionTypes"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `videoCapabilities` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeySystemConfiguration`*"]
    pub fn video_capabilities(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("videoCapabilities"),
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
impl Default for MediaKeySystemConfiguration {
    fn default() -> Self {
        Self::new()
    }
}
