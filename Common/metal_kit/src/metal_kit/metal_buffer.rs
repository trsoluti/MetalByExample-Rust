//
//  metal_buffer.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for methods we use from MTLBuffer

use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use objc::runtime::{objc_release, objc_retain};
use cocoa::foundation::{NSString, NSUInteger};
use std::os::raw::c_void;

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
    #[inline]
    pub fn to_objc(&self) -> id { self.buffer }
    /// Sets the buffer label
    #[inline]
    pub fn set_label(&mut self, label: &str) {
        let label = unsafe { NSString::alloc(nil).init_str(label) };
        unsafe { msg_send![self.buffer, setLabel:label] }
    }
    /// Gets the system address of the buffer’s storage allocation.
    #[inline]
    pub fn get_contents(&self) -> *const c_void {
        unsafe { msg_send![self.buffer, contents] }
    }
    /// Gets the logical size of the buffer, in bytes.
    #[inline]
    pub fn get_length(&self) -> NSUInteger {
        unsafe { msg_send![self.buffer, length] }
    }
}
