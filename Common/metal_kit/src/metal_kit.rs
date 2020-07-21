//
//  metal_kit.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for the classes and methods we use from the Metal framework

mod metal_buffer;
mod metal_clear_colors;
mod metal_command_buffer;
mod metal_device;
mod metal_library;
mod metal_render_command_encoder;
mod metal_render_pass_descriptor;
mod metal_render_pipeline_descriptor;
mod metal_render_pipeline_state;
mod metal_command_queue;
mod metal_texture;
mod metal_texture_descriptor;
mod metal_render_pass_depth_attachment_descriptor;
mod metal_depth_stencil_descriptor;
mod metal_depth_stencil_state;

pub use metal_buffer::MetalBuffer;
pub use metal_clear_colors::MetalClearColor;
pub use metal_command_buffer::MetalCommandBuffer;
pub use metal_device::MetalDevice;
pub use metal_device::MetalDeviceError;
pub use metal_device::MTLResourceCPUCacheModeDefaultCache;
pub use metal_device::MTLResourceOptions;
pub use metal_library::MetalLibrary;
pub use metal_render_command_encoder::MetalRenderCommandEncoder;
pub use metal_render_command_encoder::MTLWinding;
pub use metal_render_command_encoder::MTLWindingClockwise;
pub use metal_render_command_encoder::MTLWindingCounterClockwise;
pub use metal_render_command_encoder::MTLCullMode;
pub use metal_render_command_encoder::MTLCullModeNone;
pub use metal_render_command_encoder::MTLCullModeFront;
pub use metal_render_command_encoder::MTLCullModeBack;
pub use metal_render_command_encoder::MTLPrimitiveType;
pub use metal_render_command_encoder::MTLPrimitiveTypeTriangle;
pub use metal_render_command_encoder::MTLIndexType;
pub use metal_render_command_encoder::MTLIndexTypeUInt16;
pub use metal_render_command_encoder::MTLIndexTypeUInt32;
pub use metal_render_pass_descriptor::MetalRenderPassDescriptor;
pub use metal_render_pass_descriptor::MTLLoadActionClear;
pub use metal_render_pass_descriptor::MTLStoreActionDontCare;
pub use metal_render_pass_descriptor::MTLStoreActionStore;
pub use metal_render_pipeline_descriptor::MetalRenderPipelineDescriptor;
pub use metal_render_pipeline_state::MetalRenderPipelineState;
pub use metal_command_queue::MetalCommandQueue;
pub use metal_texture::MetalTexture;
pub use metal_texture::MTLTextureUsageRenderTarget;
pub use metal_texture_descriptor::MetalTextureDescriptor;
pub use metal_texture_descriptor::MTLPixelFormatDepth32Float;
pub use metal_texture_descriptor::MTLStorageModePrivate;
pub use metal_render_pass_depth_attachment_descriptor::MetalRenderPassDepthAttachment;
pub use metal_depth_stencil_descriptor::MetalDepthStencilDescriptor;
pub use metal_depth_stencil_descriptor::MTLCompareFunctionLess;
pub use metal_depth_stencil_state::MetalDepthStencilState;
