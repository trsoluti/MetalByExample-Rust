//
//  metal_command_queue.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for methods we use from MTLCommandQueue

use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use crate::metal_kit::metal_command_buffer::MetalCommandBuffer;
use objc::runtime::{objc_retain, objc_release};

/// Rust wrapper for a queue that organizes command buffers
/// to be executed by a GPU.
pub struct MetalCommandQueue {
    command_queue: id,
}
impl Default for MetalCommandQueue {
    fn default() -> Self {
        MetalCommandQueue {
            command_queue: nil,
        }
    }
}
impl From<id> for MetalCommandQueue {
    fn from(command_queue: id) -> Self {
        let command_queue = unsafe { objc_retain(command_queue) };
        MetalCommandQueue { command_queue }
    }
}
impl Drop for MetalCommandQueue {
    fn drop(&mut self) { unsafe { objc_release(self.command_queue) } }
}
impl MetalCommandQueue {
    /// Creates a command buffer.
    pub fn command_buffer(&self) -> MetalCommandBuffer {
        let command_buffer:id = unsafe { msg_send![self.command_queue, commandBuffer] };
        MetalCommandBuffer::from(command_buffer)
    }
}