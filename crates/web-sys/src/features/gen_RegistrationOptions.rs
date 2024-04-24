#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RegistrationOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegistrationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    pub type RegistrationOptions;
}
#[doc = "The trait to access properties on the `RegistrationOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
pub trait RegistrationOptionsGetters {
    #[doc = "Get the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    fn scope(&self) -> &str;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    fn type_(&self) -> &str;
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[doc = "Get the `updateViaCache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerUpdateViaCache`*"]
    fn update_via_cache(&self) -> ServiceWorkerUpdateViaCache;
}
impl RegistrationOptionsGetters for RegistrationOptions {
    fn scope(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("scope"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn type_(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    fn update_via_cache(&self) -> ServiceWorkerUpdateViaCache {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("updateViaCache"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl RegistrationOptions {
    #[doc = "Construct a new `RegistrationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    pub fn scope(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("scope"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
    pub fn type_(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("type"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[doc = "Change the `updateViaCache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerUpdateViaCache`*"]
    pub fn update_via_cache(&mut self, val: ServiceWorkerUpdateViaCache) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("updateViaCache"),
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
impl Default for RegistrationOptions {
    fn default() -> Self {
        Self::new()
    }
}
