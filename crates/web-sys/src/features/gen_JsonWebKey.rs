#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = JsonWebKey)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `JsonWebKey` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub type JsonWebKey;
}
#[doc = "The trait to access properties on the `JsonWebKey` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
pub trait JsonWebKeyGetters {
    #[doc = "Get the `alg` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn alg(&self) -> &str;
    #[doc = "Get the `crv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn crv(&self) -> &str;
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn d(&self) -> &str;
    #[doc = "Get the `dp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn dp(&self) -> &str;
    #[doc = "Get the `dq` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn dq(&self) -> &str;
    #[doc = "Get the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn e(&self) -> &str;
    #[doc = "Get the `ext` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn ext(&self) -> bool;
    #[doc = "Get the `k` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn k(&self) -> &str;
    #[doc = "Get the `key_ops` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn key_ops(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `kty` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn kty(&self) -> &str;
    #[doc = "Get the `n` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn n(&self) -> &str;
    #[doc = "Get the `oth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn oth(&self) -> &::wasm_bindgen::JsValue;
    #[doc = "Get the `p` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn p(&self) -> &str;
    #[doc = "Get the `q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn q(&self) -> &str;
    #[doc = "Get the `qi` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn qi(&self) -> &str;
    #[doc = "Get the `use` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn use_(&self) -> &str;
    #[doc = "Get the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn x(&self) -> &str;
    #[doc = "Get the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    fn y(&self) -> &str;
}
impl JsonWebKeyGetters for JsonWebKey {
    fn alg(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("alg"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn crv(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("crv"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn d(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("d"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn dp(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dp"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn dq(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("dq"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn e(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("e"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn ext(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("ext"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn k(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("k"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn key_ops(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("key_ops"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn kty(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("kty"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn n(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("n"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn oth(&self) -> &::wasm_bindgen::JsValue {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("oth"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn p(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("p"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn q(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("q"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn qi(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("qi"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn use_(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("use"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn x(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("x"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn y(&self) -> &str {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("y"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl JsonWebKey {
    #[doc = "Construct a new `JsonWebKey`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn new(kty: &str) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.kty(kty);
        ret
    }
    #[doc = "Change the `alg` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn alg(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("alg"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `crv` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn crv(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("crv"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn d(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("d"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `dp` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn dp(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("dp"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `dq` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn dq(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("dq"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `e` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn e(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("e"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `ext` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn ext(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("ext"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `k` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn k(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("k"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `key_ops` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn key_ops(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("key_ops"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `kty` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn kty(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("kty"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `n` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn n(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("n"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `oth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn oth(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("oth"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `p` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn p(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("p"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `q` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn q(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("q"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `qi` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn qi(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("qi"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `use` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn use_(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("use"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `x` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn x(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("x"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `y` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `JsonWebKey`*"]
    pub fn y(&mut self, val: &str) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("y"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
