#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PushSubscriptionInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub type PushSubscriptionInit;
}
#[doc = "The trait to access properties on the `PushSubscriptionInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
pub trait PushSubscriptionInitGetters {
    #[doc = "Get the `appServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    fn app_server_key(&self) -> Option<&::js_sys::Object>;
    #[doc = "Get the `authSecret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    fn auth_secret(&self) -> Option<&::js_sys::ArrayBuffer>;
    #[doc = "Get the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    fn endpoint(&self) -> &str;
    #[doc = "Get the `p256dhKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    fn p256dh_key(&self) -> Option<&::js_sys::ArrayBuffer>;
    #[doc = "Get the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    fn scope(&self) -> &str;
}
impl PushSubscriptionInitGetters for PushSubscriptionInit {
    fn app_server_key(&self) -> Option<&::js_sys::Object> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("appServerKey"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn auth_secret(&self) -> Option<&::js_sys::ArrayBuffer> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("authSecret"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn endpoint(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("endpoint"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn p256dh_key(&self) -> Option<&::js_sys::ArrayBuffer> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("p256dhKey"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn scope(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("scope"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl PushSubscriptionInit {
    #[doc = "Construct a new `PushSubscriptionInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub fn new(endpoint: &str, scope: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.endpoint(endpoint);
        ret.scope(scope);
        ret
    }
    #[doc = "Change the `appServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub fn app_server_key(&mut self, val: Option<&::js_sys::Object>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("appServerKey"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `authSecret` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub fn auth_secret(&mut self, val: Option<&::js_sys::ArrayBuffer>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("authSecret"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `endpoint` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub fn endpoint(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("endpoint"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `p256dhKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
    pub fn p256dh_key(&mut self, val: Option<&::js_sys::ArrayBuffer>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("p256dhKey"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionInit`*"]
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
}
