//
//  metal_command_buffer.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for methods we use from MTLCommandBuffer

use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::id;
use crate::metal_kit::metal_render_pass_descriptor::MetalRenderPassDescriptor;
use crate::metal_kit::metal_render_command_encoder::MetalRenderCommandEncoder;
use objc::runtime::{objc_retain, objc_release};
use core_animation::CoreAnimMetalDrawable;

/// Rust wrapper of a container
/// that stores encoded commands for the GPU to execute.
pub struct MetalCommandBuffer {
    buffer: id,
}
impl From<id> for MetalCommandBuffer {
    fn from(buffer:id) -> Self {
        let buffer = unsafe { objc_retain(buffer) };
        MetalCommandBuffer { buffer }
    }
}
impl Drop for MetalCommandBuffer {
    fn drop(&mut self) { unsafe { objc_release(self.buffer) } }
}

impl MetalCommandBuffer {
    /// Creates an object to encode a rendering pass into the command buffer.
    pub fn render_command_encoder_with_descriptor(&self, descriptor: &MetalRenderPassDescriptor) -> MetalRenderCommandEncoder {
        let descriptor_id = descriptor.objc_id();
        let encoder:id = unsafe { msg_send![self.buffer, renderCommandEncoderWithDescriptor:descriptor_id] };
        MetalRenderCommandEncoder::from(encoder)
    }
    /// Registers a drawable presentation to occur as soon as possible.
    pub fn present_drawable(&mut self, drawable: &CoreAnimMetalDrawable) {
        unsafe { msg_send![self.buffer, presentDrawable:drawable.to_objc()] }
    }
    /// Commits the command buffer for execution.
    pub fn commit(&mut self) {
        unsafe { msg_send![self.buffer, commit] }
    }
}
