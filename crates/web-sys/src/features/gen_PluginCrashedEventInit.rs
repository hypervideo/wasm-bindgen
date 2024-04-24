#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = PluginCrashedEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PluginCrashedEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub type PluginCrashedEventInit;
}
#[doc = "The trait to access properties on the `PluginCrashedEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
pub trait PluginCrashedEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `browserDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn browser_dump_id(&self) -> Option<&str>;
    #[doc = "Get the `gmpPlugin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn gmp_plugin(&self) -> bool;
    #[doc = "Get the `pluginDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn plugin_dump_id(&self) -> &str;
    #[doc = "Get the `pluginFilename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn plugin_filename(&self) -> Option<&str>;
    #[doc = "Get the `pluginID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn plugin_id(&self) -> u32;
    #[doc = "Get the `pluginName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn plugin_name(&self) -> &str;
    #[doc = "Get the `submittedCrashReport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    fn submitted_crash_report(&self) -> bool;
}
impl PluginCrashedEventInitGetters for PluginCrashedEventInit {
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
    fn browser_dump_id(&self) -> Option<&str> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("browserDumpID"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn gmp_plugin(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("gmpPlugin"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn plugin_dump_id(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("pluginDumpID"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn plugin_filename(&self) -> Option<&str> {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("pluginFilename"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn plugin_id(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("pluginID"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn plugin_name(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("pluginName"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn submitted_crash_report(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("submittedCrashReport"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl PluginCrashedEventInit {
    #[doc = "Construct a new `PluginCrashedEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
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
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
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
    #[doc = "Change the `browserDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn browser_dump_id(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("browserDumpID"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `gmpPlugin` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn gmp_plugin(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("gmpPlugin"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `pluginDumpID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_dump_id(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("pluginDumpID"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `pluginFilename` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_filename(&mut self, val: Option<&str>) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("pluginFilename"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `pluginID` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_id(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("pluginID"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `pluginName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn plugin_name(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("pluginName"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `submittedCrashReport` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginCrashedEventInit`*"]
    pub fn submitted_crash_report(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("submittedCrashReport"),
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
impl Default for PluginCrashedEventInit {
    fn default() -> Self {
        Self::new()
    }
}
