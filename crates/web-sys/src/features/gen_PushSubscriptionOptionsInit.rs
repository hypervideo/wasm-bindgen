#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PushSubscriptionOptionsInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PushSubscriptionOptionsInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    pub type PushSubscriptionOptionsInit;
}
#[doc = "The trait to access properties on the `PushSubscriptionOptionsInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
pub trait PushSubscriptionOptionsInitGetters {
    #[doc = "Get the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    fn application_server_key(&self) -> Option<&::wasm_bindgen::JsValue>;
    #[doc = "Get the `userVisibleOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    fn user_visible_only(&self) -> bool;
}
impl PushSubscriptionOptionsInitGetters for PushSubscriptionOptionsInit {
    fn application_server_key(&self) -> Option<&::wasm_bindgen::JsValue> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("applicationServerKey"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn user_visible_only(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("userVisibleOnly"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl PushSubscriptionOptionsInit {
    #[doc = "Construct a new `PushSubscriptionOptionsInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `applicationServerKey` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    pub fn application_server_key(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("applicationServerKey"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `userVisibleOnly` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PushSubscriptionOptionsInit`*"]
    pub fn user_visible_only(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("userVisibleOnly"),
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
impl Default for PushSubscriptionOptionsInit {
    fn default() -> Self {
        Self::new()
    }
}
