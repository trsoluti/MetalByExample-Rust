//
//  metal_render_pipeline_descriptor.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for methods we use from MTLRenderPipelineDescriptor

use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use objc::runtime::{objc_retain, objc_release};
use cocoa::foundation::{NSUInteger, NSAutoreleasePool};
use core_animation::MTLPixelFormat;

/// Rust wrapper for an argument of options
/// you pass to a device to get a render pipeline state object.
pub struct MetalRenderPipelineDescriptor {
    descriptor: id,
}
impl Default for MetalRenderPipelineDescriptor {
    fn default() -> Self {
        MetalRenderPipelineDescriptor { descriptor: nil }
    }
}
impl From<id> for MetalRenderPipelineDescriptor {
    fn from(descriptor: id) -> Self {
        let descriptor = unsafe { objc_retain(descriptor) };
        MetalRenderPipelineDescriptor { descriptor }
    }
}
impl Drop for MetalRenderPipelineDescriptor {
    fn drop(&mut self) { unsafe { objc_release(self.descriptor) } }
}

impl MetalRenderPipelineDescriptor {
    /// Creates a new MetalRenderPipelineDescriptor
    pub fn new() -> Self {
        let pool = unsafe { NSAutoreleasePool::new(nil) };
        let descriptor_class = class!(MTLRenderPipelineDescriptor);
        let descriptor: id = unsafe { msg_send![descriptor_class, new] };
        let descriptor = MetalRenderPipelineDescriptor::from(descriptor);
        unsafe { pool.drain() };
        descriptor
    }
    /// Sets, for the render target at the given index,
    /// the pixel format of the color attachment’s texture.
    pub fn set_color_attachment_pixel_format(&mut self, index: NSUInteger, pixel_format: MTLPixelFormat) {
        unsafe {
            let pool = NSAutoreleasePool::new(nil);
            let colour_attachments_array: id = msg_send![self.descriptor, colorAttachments];
            let colour_attachment_element: id = msg_send![colour_attachments_array, objectAtIndexedSubscript:index];
            let _:() = msg_send![colour_attachment_element, setPixelFormat:pixel_format];
            pool.drain();
        }
    }
    /// Sets the pixel format of the attachment that stores depth data.
    pub fn set_depth_attachment_pixel_format(&mut self, pixel_format: MTLPixelFormat) {
        unsafe { msg_send![self.descriptor, setDepthAttachmentPixelFormat:pixel_format] }
    }
    /// Sets a programmable function that processes individual vertices in a rendering pass.
    pub fn set_vertex_function(&mut self, vertex_function: id) {
        unsafe {
            let _:() = msg_send![self.descriptor, setVertexFunction:vertex_function];
        }
    }
    /// Sets a programmable function that processes individual fragments in a rendering pass.
    pub fn set_fragment_function(&mut self, fragment_function: id) {
        unsafe {
            let _:() = msg_send![self.descriptor, setFragmentFunction:fragment_function];
        }
    }
}
