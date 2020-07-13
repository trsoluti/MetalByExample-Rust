//
//  metal_device.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for methods we use from MTLDevice
// Shim for Objc Metal Device
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use objc::runtime::{objc_retain, objc_release};
use crate::metal_kit::metal_library::MetalLibrary;
use crate::metal_kit::metal_render_pipeline_descriptor::MetalRenderPipelineDescriptor;
use crate::metal_kit::metal_render_pipeline_state::MetalRenderPipelineState;
use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::metal_kit::metal_command_queue::MetalCommandQueue;
use crate::metal_kit::metal_buffer::MetalBuffer;
use cocoa::foundation::NSUInteger;

#[link(name="Metal", kind="framework")]
extern {
    // From System/Library/Frameworks/Metal.framework/Versions/A/Headers/MTLDevice.h:
    // MTL_EXTERN id <MTLDevice> __nullable MTLCreateSystemDefaultDevice(void) API_AVAILABLE(macos(10.11), ios(8.0)) NS_RETURNS_RETAINED;
    fn MTLCreateSystemDefaultDevice() -> id;
}
//
// typedef enum MTLResourceOptions : NSUInteger {
//     ...
// } MTLResourceOptions;
//
/// Optional arguments used to set the behavior of a resource.
pub type MTLResourceOptions = NSUInteger;
// System/Library/Frameworks/Metal.framework/Headers/MTLResource.h:
//     MTLCPUCacheModeDefaultCache = 0,
//
// MTLResourceCPUCacheModeDefaultCache  = MTLCPUCacheModeDefaultCache  << MTLResourceCPUCacheModeShift,
// #define MTLResourceCPUCacheModeShift            0
/// The default CPU cache mode for the resource,
/// which guarantees that read and write operations
/// are executed in the expected order.
#[allow(non_upper_case_globals)]
pub const MTLResourceCPUCacheModeDefaultCache:NSUInteger = 0; // I think we've got a good chance of this always remaining 0.

/// The error returned if the system failed
/// to create a render pipeline state
#[derive(Debug)]
pub enum MetalDeviceError {
    /// The system returned the given Objective C error
    /// when attempting to create a render pipeline state
    RenderPipelineStateCreationError(id),
}
impl Display for MetalDeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Render pipeline state creation error")
    }
}
impl Error for MetalDeviceError {}

/// Rust wrapper for the Metal interface to a GPU
/// that you use to draw graphics or do parallel computation.
pub struct MetalDevice {
    device: id,
}
impl Default for MetalDevice {
    fn default() -> Self {
        MetalDevice { device: nil }
    }
}
impl From<id> for MetalDevice {
    fn from(device: id) -> Self {
        let device = unsafe { objc_retain(device) };
        MetalDevice { device }
    }
}
impl Drop for MetalDevice {
    fn drop(&mut self) { unsafe { objc_release(self.device) } }
}

impl MetalDevice {
    /// Returns a reference to the preferred default Metal device object.
    pub fn create_system_default_device() -> Self {
        // TODO: call MTLCreateSystemDefaultDevice to get default device
        let device:id = unsafe { MTLCreateSystemDefaultDevice() };
        MetalDevice::from(device)
    }
    /// Returns the underlying objective c device
    pub fn to_objc(&self) -> id { self.device }
    /// Creates a library object containing the functions in the app’s default Metal library.
    pub fn new_default_library(&self) -> MetalLibrary {
        let library: id = unsafe { msg_send![self.device, newDefaultLibrary] };
        MetalLibrary::from(library)
    }
    /// Synchronously creates a render pipeline state object and associated reflection information.
    pub fn new_render_pipeline_state_with_descriptor(&mut self, descriptor: MetalRenderPipelineDescriptor) -> Result<MetalRenderPipelineState, MetalDeviceError> {
        //
        // NSError *error = nil;
        // _pipeline = [device newRenderPipelineStateWithDescriptor:pipelineDescriptor
        //                                                    error:&error];
        //
        // if (!_pipeline)
        // {
        //     NSLog(@"Error occurred when creating render pipeline state: %@", error);
        // }
        let error:id = nil;
        let pipeline_state:id = unsafe { msg_send![self.device, newRenderPipelineStateWithDescriptor:descriptor error:&error] };
        if pipeline_state == nil {
            Err(MetalDeviceError::RenderPipelineStateCreationError(error))
        } else {
            Ok(MetalRenderPipelineState::from(pipeline_state))
        }
    }
    /// Creates a command submission queue.
    pub fn new_command_queue(&mut self ) -> MetalCommandQueue {
        let command_queue:id = unsafe { msg_send![self.device, newCommandQueue] };
        MetalCommandQueue::from(command_queue)
    }
    // - (id<MTLBuffer>)newBufferWithBytes:(const void *)pointer length:(NSUInteger)length options:(MTLResourceOptions)options;
    /// Allocates a new buffer of a given length and initializes its contents by copying existing data into it.
    pub fn new_buffer_with_bytes_and_options<T>(&mut self, pointer: &[T], options: MTLResourceOptions) -> MetalBuffer {
        let length = std::mem::size_of_val(pointer) as NSUInteger;
        //+ debug_log(&format!("Length of buffer is {} bytes", length));
        let data_ptr = pointer.first().unwrap() as *const _;
        let buffer:id = unsafe { msg_send![self.device, newBufferWithBytes:data_ptr length:length options:options] };
        MetalBuffer::from(buffer)
    }
}
