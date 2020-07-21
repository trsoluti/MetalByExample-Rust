//
//  metal_texture_descriptor.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for methods we use from MTLTextureDescriptor

use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use objc::runtime::{objc_retain, objc_release, BOOL};
use core_animation::MTLPixelFormat;
use cocoa::foundation::NSUInteger;

// From MTLPixelFormat.h:
// typedef NS_ENUM(NSUInteger, MTLPixelFormat)
// {
#[allow(non_upper_case_globals)]
/// A 32-bit depth pixel format with one 32-bit floating-point component,
/// typically used for a depth render target.
pub const MTLPixelFormatDepth32Float:MTLPixelFormat  = 252;
//
// From MTLResource.h:
// typedef NS_ENUM(NSUInteger, MTLStorageMode)
// {
// ...
//   MTLStorageModePrivate = 2,
// ...
// }
#[allow(non_upper_case_globals)]
/// This mode allows the texture resource data
/// to be kept entirely to GPU (or driver) private memory
/// that will never be accessed by the CPU directly, so no
///  coherency of any kind must be maintained.
pub const MTLStorageModePrivate:NSUInteger = 2;

/// Rust wrapper for an object that you use
/// to configure new Metal texture objects.
pub struct MetalTextureDescriptor {
    descriptor: id,
}
impl Default for MetalTextureDescriptor {
    fn default() -> Self {
        MetalTextureDescriptor { descriptor: nil }
    }
}
impl From<id> for MetalTextureDescriptor {
    fn from(descriptor: id) -> Self {
        let descriptor = unsafe { objc_retain(descriptor) };
        MetalTextureDescriptor { descriptor }
    }
}
impl Drop for MetalTextureDescriptor {
    fn drop(&mut self) { unsafe { objc_release(self.descriptor) } }
}

impl MetalTextureDescriptor {
    // + (MTLTextureDescriptor *)texture2DDescriptorWithPixelFormat:(MTLPixelFormat)pixelFormat
    //                                                        width:(NSUInteger)width
    //                                                       height:(NSUInteger)height
    //                                                    mipmapped:(BOOL)mipmapped;
    /// Creates a texture descriptor object for a 2D texture.
    pub fn texture_2d_descriptor_with_pixel_format_and_width_and_height_and_mipmapped(
        pixel_format: MTLPixelFormat,
        width: NSUInteger,
        height: NSUInteger,
        mipmapped: BOOL
    ) -> Self {
        let class = class!(MTLTextureDescriptor);
        let descriptor:id = unsafe { msg_send![class, texture2DDescriptorWithPixelFormat:pixel_format
                                                                                    width:width
                                                                                   height:height
                                                                                mipmapped:mipmapped] };
        MetalTextureDescriptor::from(descriptor)
    }
    /// Set options that determine how you can use the texture.
    pub fn set_usage(&mut self, usage: NSUInteger) {
        unsafe { msg_send![self.descriptor, setUsage:usage] }
    }
    /// Set the location and access permissions of the texture.
    pub fn set_storage_mode(&mut self, storage_mode: NSUInteger) {
        unsafe { msg_send![self.descriptor, setStorageMode:storage_mode] }
    }
}
