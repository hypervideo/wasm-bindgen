#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SecurityPolicyViolationEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SecurityPolicyViolationEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub type SecurityPolicyViolationEventInit;
}
#[doc = "The trait to access properties on the `SecurityPolicyViolationEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
pub trait SecurityPolicyViolationEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `blockedURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn blocked_uri(&self) -> &str;
    #[doc = "Get the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn column_number(&self) -> i32;
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[doc = "Get the `disposition` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`, `SecurityPolicyViolationEventInit`*"]
    fn disposition(&self) -> SecurityPolicyViolationEventDisposition;
    #[doc = "Get the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn document_uri(&self) -> &str;
    #[doc = "Get the `effectiveDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn effective_directive(&self) -> &str;
    #[doc = "Get the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn line_number(&self) -> i32;
    #[doc = "Get the `originalPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn original_policy(&self) -> &str;
    #[doc = "Get the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn referrer(&self) -> &str;
    #[doc = "Get the `sample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn sample(&self) -> &str;
    #[doc = "Get the `sourceFile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn source_file(&self) -> &str;
    #[doc = "Get the `statusCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn status_code(&self) -> u16;
    #[doc = "Get the `violatedDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    fn violated_directive(&self) -> &str;
}
impl SecurityPolicyViolationEventInitGetters for SecurityPolicyViolationEventInit {
    fn bubbles(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("bubbles"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn cancelable(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("cancelable"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn composed(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("composed"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn blocked_uri(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("blockedURI"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn column_number(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("columnNumber"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    fn disposition(&self) -> SecurityPolicyViolationEventDisposition {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("disposition"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn document_uri(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("documentURI"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn effective_directive(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("effectiveDirective"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn line_number(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("lineNumber"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn original_policy(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("originalPolicy"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn referrer(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("referrer"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn sample(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("sample"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn source_file(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("sourceFile"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn status_code(&self) -> u16 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("statusCode"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn violated_directive(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("violatedDirective"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl SecurityPolicyViolationEventInit {
    #[doc = "Construct a new `SecurityPolicyViolationEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("bubbles"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("cancelable"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("composed"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `blockedURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn blocked_uri(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("blockedURI"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn column_number(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("columnNumber"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[doc = "Change the `disposition` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventDisposition`, `SecurityPolicyViolationEventInit`*"]
    pub fn disposition(&mut self, val: SecurityPolicyViolationEventDisposition) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("disposition"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn document_uri(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("documentURI"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `effectiveDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn effective_directive(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("effectiveDirective"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn line_number(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("lineNumber"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `originalPolicy` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn original_policy(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("originalPolicy"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn referrer(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("referrer"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `sample` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn sample(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("sample"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `sourceFile` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn source_file(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("sourceFile"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `statusCode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn status_code(&mut self, val: u16) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("statusCode"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `violatedDirective` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEventInit`*"]
    pub fn violated_directive(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("violatedDirective"),
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
impl Default for SecurityPolicyViolationEventInit {
    fn default() -> Self {
        Self::new()
    }
}
