#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = CacheBatchOperation)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CacheBatchOperation` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    pub type CacheBatchOperation;
}
#[doc = "The trait to access properties on the `CacheBatchOperation` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
pub trait CacheBatchOperationGetters {
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "Get the `options` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `CacheQueryOptions`*"]
    fn options(&self) -> &CacheQueryOptions;
    #[cfg(feature = "Request")]
    #[doc = "Get the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Request`*"]
    fn request(&self) -> &Request;
    #[cfg(feature = "Response")]
    #[doc = "Get the `response` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Response`*"]
    fn response(&self) -> &Response;
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    fn type_(&self) -> &str;
}
impl CacheBatchOperationGetters for CacheBatchOperation {
    #[cfg(feature = "CacheQueryOptions")]
    fn options(&self) -> &CacheQueryOptions {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("options"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "Request")]
    fn request(&self) -> &Request {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("request"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "Response")]
    fn response(&self) -> &Response {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("response"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn type_(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl CacheBatchOperation {
    #[doc = "Construct a new `CacheBatchOperation`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "CacheQueryOptions")]
    #[doc = "Change the `options` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `CacheQueryOptions`*"]
    pub fn options(&mut self, val: &CacheQueryOptions) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("options"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "Request")]
    #[doc = "Change the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Request`*"]
    pub fn request(&mut self, val: &Request) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("request"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "Response")]
    #[doc = "Change the `response` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`, `Response`*"]
    pub fn response(&mut self, val: &Response) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("response"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheBatchOperation`*"]
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
}
impl Default for CacheBatchOperation {
    fn default() -> Self {
        Self::new()
    }
}
