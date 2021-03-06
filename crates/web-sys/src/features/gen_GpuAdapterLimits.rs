#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = GPUAdapterLimits , typescript_type = "GPUAdapterLimits")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `GpuAdapterLimits` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type GpuAdapterLimits;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxTextureDimension1D)]
    #[doc = "Getter for the `maxTextureDimension1D` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxTextureDimension1D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_texture_dimension_1d(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxTextureDimension2D)]
    #[doc = "Getter for the `maxTextureDimension2D` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxTextureDimension2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_texture_dimension_2d(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxTextureDimension3D)]
    #[doc = "Getter for the `maxTextureDimension3D` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxTextureDimension3D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_texture_dimension_3d(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxTextureArrayLayers)]
    #[doc = "Getter for the `maxTextureArrayLayers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxTextureArrayLayers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_texture_array_layers(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxBindGroups)]
    #[doc = "Getter for the `maxBindGroups` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxBindGroups)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_bind_groups(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxDynamicUniformBuffersPerPipelineLayout)]
    #[doc = "Getter for the `maxDynamicUniformBuffersPerPipelineLayout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxDynamicUniformBuffersPerPipelineLayout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_dynamic_uniform_buffers_per_pipeline_layout(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxDynamicStorageBuffersPerPipelineLayout)]
    #[doc = "Getter for the `maxDynamicStorageBuffersPerPipelineLayout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxDynamicStorageBuffersPerPipelineLayout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_dynamic_storage_buffers_per_pipeline_layout(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxSampledTexturesPerShaderStage)]
    #[doc = "Getter for the `maxSampledTexturesPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxSampledTexturesPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_sampled_textures_per_shader_stage(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxSamplersPerShaderStage)]
    #[doc = "Getter for the `maxSamplersPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxSamplersPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_samplers_per_shader_stage(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxStorageBuffersPerShaderStage)]
    #[doc = "Getter for the `maxStorageBuffersPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxStorageBuffersPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_storage_buffers_per_shader_stage(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxStorageTexturesPerShaderStage)]
    #[doc = "Getter for the `maxStorageTexturesPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxStorageTexturesPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_storage_textures_per_shader_stage(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxUniformBuffersPerShaderStage)]
    #[doc = "Getter for the `maxUniformBuffersPerShaderStage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxUniformBuffersPerShaderStage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_uniform_buffers_per_shader_stage(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxUniformBufferBindingSize)]
    #[doc = "Getter for the `maxUniformBufferBindingSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxUniformBufferBindingSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_uniform_buffer_binding_size(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxStorageBufferBindingSize)]
    #[doc = "Getter for the `maxStorageBufferBindingSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxStorageBufferBindingSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_storage_buffer_binding_size(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxVertexBuffers)]
    #[doc = "Getter for the `maxVertexBuffers` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxVertexBuffers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_vertex_buffers(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxVertexAttributes)]
    #[doc = "Getter for the `maxVertexAttributes` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxVertexAttributes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_vertex_attributes(this: &GpuAdapterLimits) -> u32;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "GPUAdapterLimits" , js_name = maxVertexBufferArrayStride)]
    #[doc = "Getter for the `maxVertexBufferArrayStride` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/GPUAdapterLimits/maxVertexBufferArrayStride)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `GpuAdapterLimits`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn max_vertex_buffer_array_stride(this: &GpuAdapterLimits) -> u32;
}
