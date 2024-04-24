#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = Transformer)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Transformer` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub type Transformer;
}
#[doc = "The trait to access properties on the `Transformer` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
pub trait TransformerGetters {
    #[doc = "Get the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    fn flush(&self) -> &::js_sys::Function;
    #[doc = "Get the `readableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    fn readable_type(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    fn start(&self) -> &::js_sys::Function;
    #[doc = "Get the `transform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    fn transform(&self) -> &::js_sys::Function;
    #[doc = "Get the `writableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    fn writable_type(&self) -> &::wasm_bindgen::JsValue;
}
impl TransformerGetters for Transformer {
    fn flush(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("flush"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn readable_type(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("readableType"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn start(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("start"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn transform(&self) -> &::js_sys::Function {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("transform"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn writable_type(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("writableType"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl Transformer {
    #[doc = "Construct a new `Transformer`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn flush(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("flush"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `readableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn readable_type(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("readableType"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("start"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `transform` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn transform(&mut self, val: &::js_sys::Function) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("transform"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `writableType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Transformer`*"]
    pub fn writable_type(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("writableType"),
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
impl Default for Transformer {
    fn default() -> Self {
        Self::new()
    }
}
