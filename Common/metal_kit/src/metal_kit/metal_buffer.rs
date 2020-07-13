//
//  metal_buffer.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for methods we use from MTLBuffer

use cocoa::base::{id, nil};
use objc::runtime::{objc_release, objc_retain};

/// Rust wrapper for MTLBuffer
pub struct MetalBuffer {
    buffer: id,
}
impl Default for MetalBuffer {
    fn default() -> Self {
        MetalBuffer { buffer: nil }
    }
}
impl From<id> for MetalBuffer {
    fn from(buffer: id) -> Self {
        let buffer = unsafe { objc_retain(buffer) };
        MetalBuffer { buffer }
    }
}
impl Drop for MetalBuffer {
    fn drop(&mut self) { unsafe { objc_release(self.buffer) } }
}
impl MetalBuffer {
    /// Returns the underlying objective c buffer
    pub fn to_objc(&self) -> id { self.buffer }
}
