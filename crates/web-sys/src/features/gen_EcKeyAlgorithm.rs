#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = EcKeyAlgorithm)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `EcKeyAlgorithm` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"]
    pub type EcKeyAlgorithm;
    #[wasm_bindgen(method, getter = "name")]
    fn name_shim(this: &EcKeyAlgorithm) -> String;
    #[wasm_bindgen(method, setter = "name")]
    fn set_name_shim(this: &EcKeyAlgorithm, val: &str);
    #[wasm_bindgen(method, getter = "namedCurve")]
    fn named_curve_shim(this: &EcKeyAlgorithm) -> String;
    #[wasm_bindgen(method, setter = "namedCurve")]
    fn set_named_curve_shim(this: &EcKeyAlgorithm, val: &str);
}
#[doc = "The trait to access properties on the `EcKeyAlgorithm` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"]
pub trait EcKeyAlgorithmGetters {
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"]
    fn name(&self) -> String;
    #[doc = "Get the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"]
    fn named_curve(&self) -> String;
}
impl EcKeyAlgorithmGetters for EcKeyAlgorithm {
    fn name(&self) -> String {
        self.name_shim()
    }
    fn named_curve(&self) -> String {
        self.named_curve_shim()
    }
}
impl EcKeyAlgorithm {
    #[doc = "Construct a new `EcKeyAlgorithm`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"]
    pub fn new(name: &str, named_curve: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        Self::name(&mut ret, name);
        Self::named_curve(&mut ret, named_curve);
        ret
    }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"]
    pub fn name(&mut self, val: &str) -> &mut Self {
        self.set_name_shim(val);
        self
    }
    #[doc = "Change the `namedCurve` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EcKeyAlgorithm`*"]
    pub fn named_curve(&mut self, val: &str) -> &mut Self {
        self.set_named_curve_shim(val);
        self
    }
}
