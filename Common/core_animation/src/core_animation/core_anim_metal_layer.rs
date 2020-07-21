//
//  core_anim_metal_layer.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for CADisplayLink
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use cocoa::foundation::NSUInteger;
use crate::core_animation::core_anim_metal_drawable::CoreAnimMetalDrawable;
use objc::runtime::{objc_retain, objc_release};
use core_graphics::geometry::CGSize;

// From System/Library/Frameworks/Metal.framework/Versions/A/Headers/MTLPixelFormat.h:
// typedef NS_ENUM(NSUInteger, MTLPixelFormat) {...}
/// Pixel format enumeration
pub type MTLPixelFormat = NSUInteger;
// MTLPixelFormatBGRA8Unorm      = 80
/// Ordinary format with two 8-bit normalized unsigned integer components.
#[allow(non_upper_case_globals)]
pub static MTLPixelFormatBGRA8Unorm:MTLPixelFormat = 80;

/// Rust wrapper for CAMetalLayer
pub struct CoreAnimMetalLayer {
    layer: id,
}
impl Default for CoreAnimMetalLayer {
    fn default() -> Self {
        CoreAnimMetalLayer { layer: nil }
    }
}
impl From<id> for CoreAnimMetalLayer {
    fn from(layer: id) -> Self {
        let layer = unsafe { objc_retain(layer) };
        CoreAnimMetalLayer { layer }
    }
}
impl Drop for CoreAnimMetalLayer {
    fn drop(&mut self) { unsafe { objc_release(self.layer) } }
}

impl CoreAnimMetalLayer {
    /// Returns the underlying objective c metal layer.
    pub fn to_objc(&self) -> id { self.layer }
    // Note this has to be ID or we would have a circular dependency
    // between core_animation and metal_kit.
    /// Gets the Metal device for the layer.
    pub fn get_device(&self) -> id { unsafe { msg_send![self.layer, device] } }
    /// Sets the Metal device to the given value.
    pub fn set_device(&mut self, device: id) {
        unsafe { msg_send![self.layer, setDevice:device]}
    }
    /// Sets the pixel format of the layer’s textures
    /// to the given (underlying) value.
    pub fn set_pixel_format(&mut self, pixel_format: MTLPixelFormat) {
        unsafe { msg_send![self.layer, setPixelFormat:pixel_format] }
    }
    /// Waits until a Metal drawable is available, and then returns it.
    pub fn next_drawable(&self) -> CoreAnimMetalDrawable {
        let drawable:id = unsafe { msg_send![self.layer, nextDrawable] };
        CoreAnimMetalDrawable::from(drawable)
    }
    /// Gets the size, in pixels, of textures for rendering layer content.
    pub fn get_drawable_size(&self) -> CGSize {
        unsafe { msg_send![self.layer, drawableSize] }
    }
    /// Sets the size, in pixels, of textures for rendering layer content.
    pub fn set_drawable_size(&self, size: CGSize) {
        unsafe { msg_send![self.layer, setDrawableSize:size] }
    }
}
