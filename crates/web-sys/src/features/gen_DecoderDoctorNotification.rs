#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = DecoderDoctorNotification)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DecoderDoctorNotification` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub type DecoderDoctorNotification;
}
#[doc = "The trait to access properties on the `DecoderDoctorNotification` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
pub trait DecoderDoctorNotificationGetters {
    #[doc = "Get the `decodeIssue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn decode_issue(&self) -> &str;
    #[doc = "Get the `decoderDoctorReportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn decoder_doctor_report_id(&self) -> &str;
    #[doc = "Get the `docURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn doc_url(&self) -> &str;
    #[doc = "Get the `formats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn formats(&self) -> &str;
    #[doc = "Get the `isSolved` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn is_solved(&self) -> bool;
    #[doc = "Get the `resourceURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    fn resource_url(&self) -> &str;
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
    fn type_(&self) -> DecoderDoctorNotificationType;
}
impl DecoderDoctorNotificationGetters for DecoderDoctorNotification {
    fn decode_issue(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("decodeIssue"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn decoder_doctor_report_id(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("decoderDoctorReportId"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn doc_url(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("docURL"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn formats(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("formats"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn is_solved(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("isSolved"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn resource_url(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("resourceURL"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "DecoderDoctorNotificationType")]
    fn type_(&self) -> DecoderDoctorNotificationType {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("type"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl DecoderDoctorNotification {
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Construct a new `DecoderDoctorNotification`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
    pub fn new(
        decoder_doctor_report_id: &str,
        is_solved: bool,
        type_: DecoderDoctorNotificationType,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.decoder_doctor_report_id(decoder_doctor_report_id);
        ret.is_solved(is_solved);
        ret.type_(type_);
        ret
    }
    #[doc = "Change the `decodeIssue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn decode_issue(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("decodeIssue"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `decoderDoctorReportId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn decoder_doctor_report_id(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("decoderDoctorReportId"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `docURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn doc_url(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("docURL"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `formats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn formats(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("formats"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `isSolved` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn is_solved(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("isSolved"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `resourceURL` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`*"]
    pub fn resource_url(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("resourceURL"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "DecoderDoctorNotificationType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DecoderDoctorNotification`, `DecoderDoctorNotificationType`*"]
    pub fn type_(&mut self, val: DecoderDoctorNotificationType) -> &mut Self {
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
