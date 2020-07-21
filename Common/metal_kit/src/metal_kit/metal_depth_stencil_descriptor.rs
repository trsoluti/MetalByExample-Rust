//
//  metal_depth_stencil_descriptor.rs
//
//  Created by TR Solutions on 2020-07-18.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
/// A simple Rust wrapper around MTLDepthStencilDescriptor

use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use objc::runtime::{objc_retain, objc_release, BOOL};
use cocoa::foundation::NSUInteger;

// From MTLDepthStencil.h:
// typedef NS_ENUM(NSUInteger, MTLCompareFunction) {
// ...
//     MTLCompareFunctionLess = 1,
// ...
// }
/// A new value passes the comparison test if it is less than the existing value.
#[allow(non_upper_case_globals)]
pub const MTLCompareFunctionLess:NSUInteger = 1;

/// Rust wrapper around an object that configures new MTLDepthStencilState objects.
pub struct MetalDepthStencilDescriptor {
    descriptor: id,
}
impl Default for MetalDepthStencilDescriptor {
    fn default() -> Self { MetalDepthStencilDescriptor { descriptor: nil } }
}
impl From<id> for MetalDepthStencilDescriptor {
    fn from(descriptor: id) -> Self {
        let descriptor = unsafe { objc_retain(descriptor) };
        MetalDepthStencilDescriptor { descriptor }
    }
}
impl Drop for MetalDepthStencilDescriptor {
    fn drop(&mut self) { unsafe { objc_release(self.descriptor) } }
}
impl MetalDepthStencilDescriptor {
    /// Returns the underlying Objective C object.
    pub fn to_objc(&self) -> id { self.descriptor }
    /// Returns a new MetalDepthStencilDescriptor
    pub fn new() -> Self {
        let class = class!(MTLDepthStencilDescriptor);
        let descriptor:id = unsafe { msg_send![class, new] };
        MetalDepthStencilDescriptor::from(descriptor)
    }
    /// Sets the comparison that is performed
    /// between a fragment’s depth value and the depth value in the attachment,
    /// which determines whether to discard the fragment.
    pub fn set_depth_compare_function(&mut self, function: NSUInteger) {
        unsafe { msg_send![self.descriptor, setDepthCompareFunction:function] }
    }
    /// Sets a Boolean value that indicates whether depth values
    /// can be written to the depth attachment.
    pub fn set_depth_write_enabled(&mut self, enabled: BOOL) {
        unsafe { msg_send![self.descriptor, setDepthWriteEnabled:enabled] }
    }
}