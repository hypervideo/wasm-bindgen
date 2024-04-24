#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = WebGLContextAttributes)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlContextAttributes` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub type WebGlContextAttributes;
}
#[doc = "The trait to access properties on the `WebGlContextAttributes` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
pub trait WebGlContextAttributesGetters {
    #[doc = "Get the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn alpha(&self) -> bool;
    #[doc = "Get the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn antialias(&self) -> bool;
    #[doc = "Get the `depth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn depth(&self) -> bool;
    #[doc = "Get the `failIfMajorPerformanceCaveat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn fail_if_major_performance_caveat(&self) -> bool;
    #[cfg(feature = "WebGlPowerPreference")]
    #[doc = "Get the `powerPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlPowerPreference`*"]
    fn power_preference(&self) -> WebGlPowerPreference;
    #[doc = "Get the `premultipliedAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn premultiplied_alpha(&self) -> bool;
    #[doc = "Get the `preserveDrawingBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn preserve_drawing_buffer(&self) -> bool;
    #[doc = "Get the `stencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    fn stencil(&self) -> bool;
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Get the `xrCompatible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    fn xr_compatible(&self) -> bool;
}
impl WebGlContextAttributesGetters for WebGlContextAttributes {
    fn alpha(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("alpha"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn antialias(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("antialias"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn depth(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("depth"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn fail_if_major_performance_caveat(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(
            self.as_ref(),
            &JsValue::from("failIfMajorPerformanceCaveat"),
        );
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "WebGlPowerPreference")]
    fn power_preference(&self) -> WebGlPowerPreference {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("powerPreference"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn premultiplied_alpha(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("premultipliedAlpha"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn preserve_drawing_buffer(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("preserveDrawingBuffer"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn stencil(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("stencil"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(web_sys_unstable_apis)]
    fn xr_compatible(&self) -> bool {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("xrCompatible"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl WebGlContextAttributes {
    #[doc = "Construct a new `WebGlContextAttributes`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[doc = "Change the `alpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn alpha(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("alpha"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `antialias` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn antialias(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("antialias"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `depth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn depth(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(self.as_ref(), &JsValue::from("depth"), &JsValue::from(val));
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `failIfMajorPerformanceCaveat` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn fail_if_major_performance_caveat(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("failIfMajorPerformanceCaveat"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "WebGlPowerPreference")]
    #[doc = "Change the `powerPreference` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlPowerPreference`*"]
    pub fn power_preference(&mut self, val: WebGlPowerPreference) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("powerPreference"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `premultipliedAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn premultiplied_alpha(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("premultipliedAlpha"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `preserveDrawingBuffer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn preserve_drawing_buffer(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("preserveDrawingBuffer"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `stencil` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    pub fn stencil(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("stencil"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `xrCompatible` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn xr_compatible(&mut self, val: bool) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("xrCompatible"),
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
impl Default for WebGlContextAttributes {
    fn default() -> Self {
        Self::new()
    }
}
