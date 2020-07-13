//
//  metal_render_command_encoder.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for methods we use from MTLRenderCommandEncoder

use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use crate::metal_kit::metal_render_pipeline_state::MetalRenderPipelineState;
use cocoa::foundation::NSUInteger;
use crate::metal_kit::metal_buffer::MetalBuffer;
use objc::runtime::{objc_release, objc_retain};

// From System/Library/Frameworks/Metal.framework/Versions/A/Headers/MTLRenderCommandEncoder.h
/// For every separate set of three vertices, rasterize a triangle.
/// If the number of vertices is not a multiple of three,
/// either one or two vertices is ignored.
#[allow(non_upper_case_globals)]
pub static MTLPrimitiveTypeTriangle:NSUInteger = 3;

/// Rust wrapper for the object to use for encoding commands for a render pass.
pub struct MetalRenderCommandEncoder {
    encoder: id,
}
impl Default for MetalRenderCommandEncoder {
    fn default() -> Self {
        MetalRenderCommandEncoder { encoder: nil }
    }
}
impl From<id> for MetalRenderCommandEncoder {
    fn from(encoder: id) -> Self {
        let encoder = unsafe { objc_retain(encoder) };
        MetalRenderCommandEncoder { encoder }
    }
}
impl Drop for MetalRenderCommandEncoder {
    fn drop(&mut self) { unsafe { objc_release(self.encoder) } }
}
impl MetalRenderCommandEncoder {
    /// Sets the current render pipeline state object.
    pub fn set_render_pipeline_state(&mut self, pipeline: &MetalRenderPipelineState) {
        unsafe { msg_send![self.encoder, setRenderPipelineState:pipeline.to_objc()] }
    }
    /// Sets a buffer for the vertex function.
    pub fn set_vertex_buffer(&mut self, vertex_buffer: &MetalBuffer, offset:NSUInteger, index: NSUInteger) {
        unsafe { msg_send![self.encoder, setVertexBuffer:vertex_buffer.to_objc() offset:offset atIndex:index] }
    }
    /// Encodes a command to render one instance of primitives using vertex data in contiguous array elements.
    pub fn draw_primitives(&mut self, primitive: NSUInteger, vertex_start: NSUInteger, vertex_count: NSUInteger) {
        unsafe { msg_send![self.encoder, drawPrimitives:primitive vertexStart:vertex_start vertexCount:vertex_count] }
    }
    /// Declares that all command generation from the encoder is completed.
    pub fn end_encoding(&mut self) {
        unsafe { msg_send![self.encoder, endEncoding] }
    }
}
