#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = FileSystemCreateWritableOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemCreateWritableOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
    pub type FileSystemCreateWritableOptions;
}
#[doc = "The trait to access properties on the `FileSystemCreateWritableOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
pub trait FileSystemCreateWritableOptionsGetters {
    #[doc = "Get the `keepExistingData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
    fn keep_existing_data(&self) -> bool;
}
impl FileSystemCreateWritableOptionsGetters for FileSystemCreateWritableOptions {
    fn keep_existing_data(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("keepExistingData"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl FileSystemCreateWritableOptions {
    #[doc = "Construct a new `FileSystemCreateWritableOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `keepExistingData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemCreateWritableOptions`*"]
    pub fn keep_existing_data(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("keepExistingData"),
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
impl Default for FileSystemCreateWritableOptions {
    fn default() -> Self {
        Self::new()
    }
}
