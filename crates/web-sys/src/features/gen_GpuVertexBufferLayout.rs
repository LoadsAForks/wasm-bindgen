#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUVertexBufferLayout)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuVertexBufferLayout` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuVertexBufferLayout;
    #[wasm_bindgen(method, setter = "arrayStride")]
    fn array_stride_shim(this: &GpuVertexBufferLayout, val: f64);
    #[wasm_bindgen(method, setter = "attributes")]
    fn attributes_shim(this: &GpuVertexBufferLayout, val: &::wasm_bindgen::JsValue);
    #[cfg(feature = "GpuVertexStepMode")]
    #[wasm_bindgen(method, setter = "stepMode")]
    fn step_mode_shim(this: &GpuVertexBufferLayout, val: GpuVertexStepMode);
}
#[cfg(web_sys_unstable_apis)]
impl GpuVertexBufferLayout {
    #[doc = "Construct a new `GpuVertexBufferLayout`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new(array_stride: f64, attributes: &::wasm_bindgen::JsValue) -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new());
        ret.array_stride(array_stride);
        ret.attributes(attributes);
        ret
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `arrayStride` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn array_stride(&mut self, val: f64) -> &mut Self {
        self.array_stride_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Change the `attributes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn attributes(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self {
        self.attributes_shim(val);
        self
    }
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "GpuVertexStepMode")]
    #[doc = "Change the `stepMode` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuVertexBufferLayout`, `GpuVertexStepMode`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn step_mode(&mut self, val: GpuVertexStepMode) -> &mut Self {
        self.step_mode_shim(val);
        self
    }
}
