#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = UserProximityEventInit)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UserProximityEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub type UserProximityEventInit;
    #[wasm_bindgen(method, getter = "bubbles")]
    fn bubbles_shim(this: &UserProximityEventInit) -> bool;
    #[wasm_bindgen(method, setter = "bubbles")]
    fn set_bubbles_shim(this: &UserProximityEventInit, val: bool);
    #[wasm_bindgen(method, getter = "cancelable")]
    fn cancelable_shim(this: &UserProximityEventInit) -> bool;
    #[wasm_bindgen(method, setter = "cancelable")]
    fn set_cancelable_shim(this: &UserProximityEventInit, val: bool);
    #[wasm_bindgen(method, getter = "composed")]
    fn composed_shim(this: &UserProximityEventInit) -> bool;
    #[wasm_bindgen(method, setter = "composed")]
    fn set_composed_shim(this: &UserProximityEventInit, val: bool);
    #[wasm_bindgen(method, getter = "near")]
    fn near_shim(this: &UserProximityEventInit) -> bool;
    #[wasm_bindgen(method, setter = "near")]
    fn set_near_shim(this: &UserProximityEventInit, val: bool);
}
#[doc = "The trait to access properties on the `UserProximityEventInit` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
pub trait UserProximityEventInitGetters {
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    fn bubbles(&self) -> bool;
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    fn cancelable(&self) -> bool;
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    fn composed(&self) -> bool;
    #[doc = "Get the `near` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    fn near(&self) -> bool;
}
impl UserProximityEventInitGetters for UserProximityEventInit {
    fn bubbles(&self) -> bool {
        self.bubbles_shim()
    }
    fn cancelable(&self) -> bool {
        self.cancelable_shim()
    }
    fn composed(&self) -> bool {
        self.composed_shim()
    }
    fn near(&self) -> bool {
        self.near_shim()
    }
}
impl UserProximityEventInit {
    #[doc = "Construct a new `UserProximityEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub fn bubbles(&mut self, val: bool) -> &mut Self {
        self.set_bubbles_shim(val);
        self
    }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub fn cancelable(&mut self, val: bool) -> &mut Self {
        self.set_cancelable_shim(val);
        self
    }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub fn composed(&mut self, val: bool) -> &mut Self {
        self.set_composed_shim(val);
        self
    }
    #[doc = "Change the `near` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UserProximityEventInit`*"]
    pub fn near(&mut self, val: bool) -> &mut Self {
        self.set_near_shim(val);
        self
    }
}
impl Default for UserProximityEventInit {
    fn default() -> Self {
        Self::new()
    }
}
