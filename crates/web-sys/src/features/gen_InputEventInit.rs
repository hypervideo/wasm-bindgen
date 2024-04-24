#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = InputEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `InputEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub type InputEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &InputEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &InputEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &InputEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &InputEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &InputEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &InputEventInit, val: bool);
    #[wasm_bindgen(method, getter = "detail")]
    fn detail_shim(this: &InputEventInit) -> i32;
    #[wasm_bindgen(method, setter = "detail")]
    fn set_detail_shim(this: &InputEventInit, val: i32);
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, getter = "view")]
    fn view_shim(this: &InputEventInit) -> Option<&Window>;
    #[cfg(feature = "Window")]
    #[wasm_bindgen(method, setter = "view")]
    fn set_view_shim(this: &InputEventInit, val: Option<&Window>);
    #[wasm_bindgen(method, getter = "data")]
    fn data_shim(this: &InputEventInit) -> Option<&str>;
    #[wasm_bindgen(method, setter = "data")]
    fn set_data_shim(this: &InputEventInit, val: Option<&str>);
    #[cfg(feature = "DataTransfer")]
    #[wasm_bindgen(method, getter = "dataTransfer")]
    fn data_transfer_shim(this: &InputEventInit) -> Option<&DataTransfer>;
    #[cfg(feature = "DataTransfer")]
    #[wasm_bindgen(method, setter = "dataTransfer")]
    fn set_data_transfer_shim(this: &InputEventInit, val: Option<&DataTransfer>);
    #[wasm_bindgen(method, getter = "inputType")]
    fn input_type_shim(this: &InputEventInit) -> &str;
    #[wasm_bindgen(method, setter = "inputType")]
    fn set_input_type_shim(this: &InputEventInit, val: &str);
    #[wasm_bindgen(method, getter = "isComposing")]
    fn is_composing_shim(this: &InputEventInit) -> bool;
    #[wasm_bindgen(method, setter = "isComposing")]
    fn set_is_composing_shim(this: &InputEventInit, val: bool);
    #[wasm_bindgen(method, getter = "targetRanges")]
    fn target_ranges_shim(this: &InputEventInit) -> &::wasm_bindgen::JsValue;
    #[wasm_bindgen(method, setter = "targetRanges")]
    fn set_target_ranges_shim(this: &InputEventInit, val: &::wasm_bindgen::JsValue);
}
#[doc = "The trait to access properties on the `InputEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
pub trait InputEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    fn detail(&self) -> i32;
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`, `Window`*"]
    fn view(&self) -> Option<&Window>;
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    fn data(&self) -> Option<&str>;
    #[cfg(feature = "DataTransfer")]
    #[doc = "Get the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `InputEventInit`*"]
    fn data_transfer(&self) -> Option<&DataTransfer>;
    #[doc = "Get the `inputType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    fn input_type(&self) -> &str;
    #[doc = "Get the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    fn is_composing(&self) -> bool;
    #[doc = "Get the `targetRanges` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    fn target_ranges(&self) -> &::wasm_bindgen::JsValue;
}
impl InputEventInitGetters for InputEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn detail(&self) -> i32 {
        self.detail_shim()
    }
    #[cfg(feature = "Window")]
    fn view(&self) -> Option<&Window> {
        self.view_shim()
    }
    fn data(&self) -> Option<&str> {
        self.data_shim()
    }
    #[cfg(feature = "DataTransfer")]
    fn data_transfer(&self) -> Option<&DataTransfer> {
        self.data_transfer_shim()
    }
    fn input_type(&self) -> &str {
        self.input_type_shim()
    }
    fn is_composing(&self) -> bool {
        self.is_composing_shim()
    }
    fn target_ranges(&self) -> &::wasm_bindgen::JsValue {
        self.target_ranges_shim()
    }
}
impl InputEventInit {
    #[doc = "Construct a new `InputEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn detail(&mut self, val: i32) -> &mut Self {
        self.set_detail_shim(val);
        self
    }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`, `Window`*"]
    pub fn view(&mut self, val: Option<&Window>) -> &mut Self {
        self.set_view_shim(val);
        self
    }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn data(&mut self, val: Option<&str>) -> &mut Self {
        self.set_data_shim(val);
        self
    }
    #[cfg(feature = "DataTransfer")]
    #[doc = "Change the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `InputEventInit`*"]
    pub fn data_transfer(&mut self, val: Option<&DataTransfer>) -> &mut Self {
        self.set_data_transfer_shim(val);
        self
    }
    #[doc = "Change the `inputType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn input_type(&mut self, val: &str) -> &mut Self {
        self.set_input_type_shim(val);
        self
    }
    #[doc = "Change the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn is_composing(&mut self, val: bool) -> &mut Self {
        self.set_is_composing_shim(val);
        self
    }
    #[doc = "Change the `targetRanges` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
    pub fn target_ranges(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.set_target_ranges_shim(val);
        self
    }
}
impl Default for InputEventInit {
    fn default() -> Self {
        Self::new()
    }
}
