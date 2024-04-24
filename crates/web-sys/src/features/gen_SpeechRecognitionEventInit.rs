#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SpeechRecognitionEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SpeechRecognitionEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub type SpeechRecognitionEventInit;
}
#[doc = "The trait to access properties on the `SpeechRecognitionEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
pub trait SpeechRecognitionEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    fn composed(&self) -> bool;
    #[cfg(feature = "Document")]
    #[doc = "Get the `emma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SpeechRecognitionEventInit`*"]
    fn emma(&self) -> Option<&Document>;
    #[doc = "Get the `interpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    fn interpretation(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `resultIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    fn result_index(&self) -> u32;
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[doc = "Get the `results` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`, `SpeechRecognitionResultList`*"]
    fn results(&self) -> Option<&SpeechRecognitionResultList>;
}
impl SpeechRecognitionEventInitGetters for SpeechRecognitionEventInit {
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
    #[cfg(feature = "Document")]
    fn emma(&self) -> Option<&Document> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("emma"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn interpretation(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("interpretation"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn result_index(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("resultIndex"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "SpeechRecognitionResultList")]
    fn results(&self) -> Option<&SpeechRecognitionResultList> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("results"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl SpeechRecognitionEventInit {
    #[doc = "Construct a new `SpeechRecognitionEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
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
    #[cfg(feature = "Document")]
    #[doc = "Change the `emma` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `SpeechRecognitionEventInit`*"]
    pub fn emma(&mut self, val: Option<&Document>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("emma"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `interpretation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub fn interpretation(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("interpretation"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `resultIndex` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`*"]
    pub fn result_index(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("resultIndex"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "SpeechRecognitionResultList")]
    #[doc = "Change the `results` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechRecognitionEventInit`, `SpeechRecognitionResultList`*"]
    pub fn results(&mut self, val: Option<&SpeechRecognitionResultList>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("results"),
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
impl Default for SpeechRecognitionEventInit {
    fn default() -> Self {
        Self::new()
    }
}
