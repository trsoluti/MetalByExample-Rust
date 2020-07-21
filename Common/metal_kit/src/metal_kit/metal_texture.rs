//
//  metal_texture.rs
//
//  Created by TR Solutions on 2020-07-18.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for MTLTexture struct and associated data

use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use objc::runtime::{objc_release, objc_retain};
use cocoa::foundation::NSUInteger;

// From MTLTexture.h:
// typedef NS_OPTIONS(NSUInteger, MTLTextureUsage)
// {
//  ...
// MTLTextureUsageRenderTarget    = 0x0004,
// }
/// An option for rendering to the texture in a render pass.
///
/// Set this option if you use the given texture as a color,
/// depth, or stencil render target in any render pass.
/// This option allows you to assign the texture to the texture property
/// of a MTLRenderPassAttachmentDescriptor.
#[allow(non_upper_case_globals)]
pub const MTLTextureUsageRenderTarget:NSUInteger    = 0x0004;

/// Rust wrapper for a resource that holds formatted image data.
pub struct MetalTexture {
    texture: id,
}
impl Default for MetalTexture {
    fn default() -> Self {
        MetalTexture { texture: nil }
    }
}
impl From<id> for MetalTexture {
    fn from(texture: id) -> Self {
        let texture = unsafe { objc_retain(texture) };
        MetalTexture { texture }
    }
}
impl Drop for MetalTexture {
    fn drop(&mut self) { unsafe { objc_release(self.texture) } }
}

impl MetalTexture {
    /// Returns true if the underlying Objective C object is nil.
    pub fn is_null(&self) -> bool {
        self.texture.is_null()
    }
    /// Returns the underlying Objective C texture.
    pub fn to_objc(&self) -> id { self.texture }
    /// Gets the width of the texture image for the base level mipmap, in pixels.
    pub fn get_width(&self) -> NSUInteger {
        unsafe { msg_send![self.texture, width] }
    }
    /// Gets the height of the texture image for the base level mipmap, in pixels.
    pub fn get_height(&self) ->  NSUInteger {
        unsafe { msg_send![self.texture, height] }
    }
}
