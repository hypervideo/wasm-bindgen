#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = AssignedNodesOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AssignedNodesOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    pub type AssignedNodesOptions;
}
#[doc = "The trait to access properties on the `AssignedNodesOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
pub trait AssignedNodesOptionsGetters {
    #[doc = "Get the `flatten` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    fn flatten(&self) -> bool;
}
impl AssignedNodesOptionsGetters for AssignedNodesOptions {
    fn flatten(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("flatten"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl AssignedNodesOptions {
    #[doc = "Construct a new `AssignedNodesOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `flatten` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AssignedNodesOptions`*"]
    pub fn flatten(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("flatten"),
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
impl Default for AssignedNodesOptions {
    fn default() -> Self {
        Self::new()
    }
}
