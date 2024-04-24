#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = TreeCellInfo)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeCellInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    pub type TreeCellInfo;
}
#[doc = "The trait to access properties on the `TreeCellInfo` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
pub trait TreeCellInfoGetters {
    #[doc = "Get the `childElt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    fn child_elt(&self) -> &str;
    #[doc = "Get the `row` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    fn row(&self) -> i32;
}
impl TreeCellInfoGetters for TreeCellInfo {
    fn child_elt(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("childElt"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn row(&self) -> i32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("row"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl TreeCellInfo {
    #[doc = "Construct a new `TreeCellInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `childElt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    pub fn child_elt(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("childElt"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `row` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
    pub fn row(&mut self, val: i32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("row"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for TreeCellInfo {
    fn default() -> Self {
        Self::new()
    }
}
