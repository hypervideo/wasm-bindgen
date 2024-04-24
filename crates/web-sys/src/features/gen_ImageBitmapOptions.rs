#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = ImageBitmapOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ImageBitmapOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub type ImageBitmapOptions;
}
#[doc = "The trait to access properties on the `ImageBitmapOptions` dictionary."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
pub trait ImageBitmapOptionsGetters {
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Get the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageBitmapOptions`*"]
    fn color_space_conversion(&self) -> ColorSpaceConversion;
    #[cfg(feature = "ImageOrientation")]
    #[doc = "Get the `imageOrientation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageOrientation`*"]
    fn image_orientation(&self) -> ImageOrientation;
    #[cfg(feature = "PremultiplyAlpha")]
    #[doc = "Get the `premultiplyAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `PremultiplyAlpha`*"]
    fn premultiply_alpha(&self) -> PremultiplyAlpha;
    #[doc = "Get the `resizeHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    fn resize_height(&self) -> u32;
    #[cfg(feature = "ResizeQuality")]
    #[doc = "Get the `resizeQuality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ResizeQuality`*"]
    fn resize_quality(&self) -> ResizeQuality;
    #[doc = "Get the `resizeWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    fn resize_width(&self) -> u32;
}
impl ImageBitmapOptionsGetters for ImageBitmapOptions {
    #[cfg(feature = "ColorSpaceConversion")]
    fn color_space_conversion(&self) -> ColorSpaceConversion {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("colorSpaceConversion"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "ImageOrientation")]
    fn image_orientation(&self) -> ImageOrientation {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("imageOrientation"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "PremultiplyAlpha")]
    fn premultiply_alpha(&self) -> PremultiplyAlpha {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("premultiplyAlpha"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn resize_height(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("resizeHeight"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    #[cfg(feature = "ResizeQuality")]
    fn resize_quality(&self) -> ResizeQuality {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("resizeQuality"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
    fn resize_width(&self) -> u32 {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::get(self.as_ref(), &JsValue::from("resizeWidth"));
        let r = r.expect("getting properties should never fail on our dictionary objects");
        ::wasm_bindgen::JsCast::unchecked_into(r)
    }
}
impl ImageBitmapOptions {
    #[doc = "Construct a new `ImageBitmapOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret
    }
    #[cfg(feature = "ColorSpaceConversion")]
    #[doc = "Change the `colorSpaceConversion` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ColorSpaceConversion`, `ImageBitmapOptions`*"]
    pub fn color_space_conversion(&mut self, val: ColorSpaceConversion) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("colorSpaceConversion"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "ImageOrientation")]
    #[doc = "Change the `imageOrientation` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageOrientation`*"]
    pub fn image_orientation(&mut self, val: ImageOrientation) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("imageOrientation"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "PremultiplyAlpha")]
    #[doc = "Change the `premultiplyAlpha` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `PremultiplyAlpha`*"]
    pub fn premultiply_alpha(&mut self, val: PremultiplyAlpha) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("premultiplyAlpha"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `resizeHeight` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub fn resize_height(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("resizeHeight"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[cfg(feature = "ResizeQuality")]
    #[doc = "Change the `resizeQuality` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ResizeQuality`*"]
    pub fn resize_quality(&mut self, val: ResizeQuality) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("resizeQuality"),
            &JsValue::from(val),
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
    #[doc = "Change the `resizeWidth` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`*"]
    pub fn resize_width(&mut self, val: u32) -> &mut Self {
        use wasm_bindgen::JsValue;
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from("resizeWidth"),
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
impl Default for ImageBitmapOptions {
    fn default() -> Self {
        Self::new()
    }
}
