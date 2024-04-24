#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ConnStatusDict)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConnStatusDict` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    pub type ConnStatusDict;
}
#[doc = "The trait to access properties on the `ConnStatusDict` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
pub trait ConnStatusDictGetters {
    #[doc = "Get the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    fn status(&self) -> &str;
}
impl ConnStatusDictGetters for ConnStatusDict {
    fn status(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("status"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ConnStatusDict {
    #[doc = "Construct a new `ConnStatusDict`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `status` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConnStatusDict`*"]
    pub fn status(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r =
            ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("status"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
impl Default for ConnStatusDict {
    fn default() -> Self {
        Self::new()
    }
}
