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
use std::os::raw::c_void;

// From GlueLib.h:
// /// @typedef command_buffer_completion_function_with_user_data_t
// ///
// /// @abstract
// /// The signature of the callback which the command buffer will call
// /// each time the command buffer has finished the command.
// typedef void (*command_buffer_completion_function_with_user_data_t)(id<MTLCommandBuffer> _Nonnull, void *_Nullable);
/// The signature of the callback which the command buffer will call
/// each time the command buffer has finished the command.
#[allow(non_camel_case_types)]
pub type command_buffer_completion_function_with_user_data_t = extern "C" fn(command_buffer: id, user_data: *const c_void);
#[link(name="GlueLib", kind="dylib")]
extern {
    // /// @function command_buffer_add_completed_handler_f_with_user_data
    // ///
    // /// @abstract
    // /// This function allows the caller to pass in arbitrary user data which is passed back
    // /// when the command buffer completes operation. The data can, for example, allow the callback to be connected to
    // /// the calling object's id.
    // ///
    // /// This function is a kludge  to allow Rust classes to set up dispatch callbacks.
    // void command_buffer_add_completed_handler_f_with_user_data
    // (id<MTLCommandBuffer> _Nonnull,
    //  command_buffer_completion_function_with_user_data_t _Nullable,
    //  void *_Nullable
    //  );
    fn command_buffer_add_completed_handler_f_with_user_data(
        command_buffer: id,
        completion_function: command_buffer_completion_function_with_user_data_t,
        user_data: *const c_void
    );
}

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
    #[inline]
    pub fn render_command_encoder_with_descriptor(&self, descriptor: &MetalRenderPassDescriptor) -> MetalRenderCommandEncoder {
        let descriptor_id = descriptor.objc_id();
        let encoder:id = unsafe { msg_send![self.buffer, renderCommandEncoderWithDescriptor:descriptor_id] };
        MetalRenderCommandEncoder::from(encoder)
    }
    /// Registers a drawable presentation to occur as soon as possible.
    #[inline]
    pub fn present_drawable(&mut self, drawable: &CoreAnimMetalDrawable) {
        unsafe { msg_send![self.buffer, presentDrawable:drawable.to_objc()] }
    }
    /// Commits the command buffer for execution.
    #[inline]
    pub fn commit(&mut self) {
        unsafe { msg_send![self.buffer, commit] }
    }
    /// Attaches a command to be executed
    /// when the buffer completes operation
    #[inline]
    pub fn attach_completion_function(
        &self,
        completion_function: command_buffer_completion_function_with_user_data_t,
        user_data: *const c_void
    ) {
        unsafe { command_buffer_add_completed_handler_f_with_user_data(
            self.buffer,
            completion_function,
            user_data
        )}
    }

}
