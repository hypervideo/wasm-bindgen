#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = SerialInputSignals)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SerialInputSignals` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type SerialInputSignals;
}
#[cfg(web_sys_unstable_apis)]
#[doc = "The trait to access properties on the `SerialInputSignals` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
pub trait SerialInputSignalsGetters {
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `clearToSend` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn clear_to_send(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dataCarrierDetect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn data_carrier_detect(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `dataSetReady` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn data_set_ready(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `ringIndicator` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn ring_indicator(&self) -> bool;
}
#[cfg(web_sys_unstable_apis)]
impl SerialInputSignalsGetters for SerialInputSignals {
    #[cfg(web_sys_unstable_apis)]
    fn clear_to_send(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("clearToSend"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    fn data_carrier_detect(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dataCarrierDetect"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    fn data_set_ready(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dataSetReady"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    fn ring_indicator(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ringIndicator"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
#[cfg(web_sys_unstable_apis)]
impl SerialInputSignals {
    #[doc = "Construct a new `SerialInputSignals`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(
        clear_to_send: bool,
        data_carrier_detect: bool,
        data_set_ready: bool,
        ring_indicator: bool,
    ) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.clear_to_send(clear_to_send);
        ret.data_carrier_detect(data_carrier_detect);
        ret.data_set_ready(data_set_ready);
        ret.ring_indicator(ring_indicator);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `clearToSend` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn clear_to_send(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("clearToSend"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataCarrierDetect` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data_carrier_detect(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("dataCarrierDetect"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `dataSetReady` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn data_set_ready(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("dataSetReady"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `ringIndicator` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SerialInputSignals`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn ring_indicator(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("ringIndicator"),
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
