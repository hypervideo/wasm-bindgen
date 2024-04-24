#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = MutationObserverInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MutationObserverInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub type MutationObserverInit;
}
#[doc = "The trait to access properties on the `MutationObserverInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
pub trait MutationObserverInitGetters {
    #[doc = "Get the `animations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn animations(&self) -> bool;
    #[doc = "Get the `attributeFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn attribute_filter(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `attributeOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn attribute_old_value(&self) -> bool;
    #[doc = "Get the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn attributes(&self) -> bool;
    #[doc = "Get the `characterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn character_data(&self) -> bool;
    #[doc = "Get the `characterDataOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn character_data_old_value(&self) -> bool;
    #[doc = "Get the `childList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn child_list(&self) -> bool;
    #[doc = "Get the `nativeAnonymousChildList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn native_anonymous_child_list(&self) -> bool;
    #[doc = "Get the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    fn subtree(&self) -> bool;
}
impl MutationObserverInitGetters for MutationObserverInit {
    fn animations(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("animations"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn attribute_filter(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("attributeFilter"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn attribute_old_value(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("attributeOldValue"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn attributes(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("attributes"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn character_data(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("characterData"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn character_data_old_value(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("characterDataOldValue"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn child_list(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("childList"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn native_anonymous_child_list(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("nativeAnonymousChildList"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn subtree(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("subtree"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl MutationObserverInit {
    #[doc = "Construct a new `MutationObserverInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `animations` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn animations(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("animations"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `attributeFilter` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn attribute_filter(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("attributeFilter"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `attributeOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn attribute_old_value(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("attributeOldValue"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn attributes(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("attributes"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `characterData` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn character_data(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("characterData"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `characterDataOldValue` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn character_data_old_value(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("characterDataOldValue"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `childList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn child_list(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("childList"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `nativeAnonymousChildList` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn native_anonymous_child_list(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("nativeAnonymousChildList"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `subtree` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationObserverInit`*"]
    pub fn subtree(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("subtree"),
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
impl Default for MutationObserverInit {
    fn default() -> Self {
        Self::new()
    }
}
