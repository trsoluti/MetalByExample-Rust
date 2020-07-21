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
use crate::MetalDepthStencilState;

// From System/Library/Frameworks/Metal.framework/Headers/MTLRenderCommandEncoder.h:
// typedef NS_ENUM(NSUInteger, MTLWinding) {
//     MTLWindingClockwise = 0,
//     MTLWindingCounterClockwise = 1,
// } API_AVAILABLE(macos(10.11), ios(8.0));
/// The vertex winding rule that determines a front-facing primitive.
pub type MTLWinding = NSUInteger;
/// Primitives whose vertices are specified in clockwise order are front-facing.
#[allow(non_upper_case_globals)]
pub const MTLWindingClockwise: MTLWinding = 0;
/// Primitives whose vertices are specified in counter-clockwise order are front-facing.
#[allow(non_upper_case_globals)]
pub const MTLWindingCounterClockwise: MTLWinding = 1;
//
// typedef NS_ENUM(NSUInteger, MTLCullMode) {
//     MTLCullModeNone = 0,
//     MTLCullModeFront = 1,
//     MTLCullModeBack = 2,
// } API_AVAILABLE(macos(10.11), ios(8.0));
/// The mode that determines whether to perform
/// culling and which type of primitive to cull.
pub type MTLCullMode = NSUInteger;
/// Does not cull any primitives.
#[allow(non_upper_case_globals)]
pub const MTLCullModeNone: MTLCullMode = 0;
/// Culls front-facing primitives.
#[allow(non_upper_case_globals)]
pub const MTLCullModeFront: MTLCullMode = 1;
/// Culls back-facing primitives.
#[allow(non_upper_case_globals)]
pub const MTLCullModeBack: MTLCullMode = 2;
//
// typedef NS_ENUM(NSUInteger, MTLPrimitiveType) {
//     MTLPrimitiveTypePoint = 0,
//     MTLPrimitiveTypeLine = 1,
//     MTLPrimitiveTypeLineStrip = 2,
//     MTLPrimitiveTypeTriangle = 3,
//     MTLPrimitiveTypeTriangleStrip = 4,
// } API_AVAILABLE(macos(10.11), ios(8.0));
/// The geometric primitive type for drawing commands.
pub type MTLPrimitiveType = NSUInteger;
/// For every separate set of three vertices, rasterize a triangle.
/// If the number of vertices is not a multiple of three,
/// either one or two vertices is ignored.
#[allow(non_upper_case_globals)]
pub const MTLPrimitiveTypeTriangle:MTLPrimitiveType = 3;

// From System/Library/Frameworks/Metal.framework/Headers/MTLStageInputOutputDescriptor.h:
// typedef NS_ENUM(NSUInteger, MTLIndexType) {
//     MTLIndexTypeUInt16 = 0,
//     MTLIndexTypeUInt32 = 1,
// } API_AVAILABLE(macos(10.11), ios(8.0));
/// The index type for an index buffer that references vertices of geometric primitives.
pub type MTLIndexType = NSUInteger;
/// A 16-bit unsigned integer used as a primitive index.
#[allow(non_upper_case_globals)]
pub const MTLIndexTypeUInt16:NSUInteger = 0;
/// A 32-bit unsigned integer used as a primitive index.
#[allow(non_upper_case_globals)]
pub const MTLIndexTypeUInt32:NSUInteger = 1;


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
    #[inline]
    pub fn set_render_pipeline_state(&mut self, pipeline: &MetalRenderPipelineState) {
        unsafe { msg_send![self.encoder, setRenderPipelineState:pipeline.to_objc()] }
    }
    /// Sets a buffer for the vertex function.
    #[inline]
    pub fn set_vertex_buffer(&mut self, vertex_buffer: &MetalBuffer, offset:NSUInteger, index: NSUInteger) {
        unsafe { msg_send![self.encoder, setVertexBuffer:vertex_buffer.to_objc() offset:offset atIndex:index] }
    }
    /// Sets the depth and stencil test state.
    #[inline]
    pub fn set_depth_stencil_state(&mut self, state: &MetalDepthStencilState) {
        unsafe { msg_send![self.encoder, setDepthStencilState:state.to_objc()] }
    }
    /// Sets the winding order of front-facing primitives.
    #[inline]
    pub fn set_front_facing_winding(&mut self, winding: MTLWinding) {
        unsafe { msg_send![self.encoder, setFrontFacingWinding:winding] }
    }
    /// Specifies whether to cull primitives when front- or back-facing.
    #[inline]
    pub fn set_cull_mode(&mut self, cull_mode: MTLCullMode) {
        unsafe { msg_send![self.encoder, setCullMode:cull_mode] }
    }
    /// Encodes a command to render one instance of primitives using vertex data in contiguous array elements.
    #[inline]
    pub fn draw_primitives(&mut self, primitive: NSUInteger, vertex_start: NSUInteger, vertex_count: NSUInteger) {
        unsafe { msg_send![self.encoder, drawPrimitives:primitive vertexStart:vertex_start vertexCount:vertex_count] }
    }
    /// Encodes a command to render one instance of primitives
    /// using an index list specified in a buffer.
    #[inline]
    pub fn draw_indexed_primitives_with_count_and_type_and_buffer_and_offset(
        &mut self,
        primitive_type: MTLPrimitiveType,
        index_count: NSUInteger,
        index_type: MTLIndexType,
        index_buffer: &MetalBuffer,
        index_buffer_offset: NSUInteger
    ) {
        unsafe { msg_send![self.encoder,
             drawIndexedPrimitives:primitive_type
                        indexCount:index_count
                         indexType:index_type
                       indexBuffer:index_buffer.to_objc()
                 indexBufferOffset:index_buffer_offset
        ] }
    }
    /// Declares that all command generation from the encoder is completed.
    #[inline]
    pub fn end_encoding(&mut self) {
        unsafe { msg_send![self.encoder, endEncoding] }
    }
}
