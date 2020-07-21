//
//  mbe_renderer.rs
//
//  Original objc code Copyright (c) 2015 Warren Moore
//  from https://github.com/metal-by-example/sample-code
//  Licensed under MIT.
//
//  Translated to Rust by TR Solutions on 18/7/20.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//
//  See appropriate LICENCE files for details.
//

use crate::mbe_items::mbe_metal_view::{RustMetalViewDelegate, RustMetalView};
use metal_kit::{MetalDevice, MetalBuffer, MetalCommandQueue, MetalRenderPipelineState, MetalDeviceError, MetalRenderPipelineDescriptor, MTLPixelFormatDepth32Float, MetalDepthStencilDescriptor, MTLCompareFunctionLess, MetalDepthStencilState, MTLResourceCPUCacheModeDefaultCache, MetalClearColor, MetalRenderPassDescriptor, MTLStoreActionStore, MTLLoadActionClear, MTLStoreActionDontCare, MTLWindingCounterClockwise, MTLCullModeBack, MTLPrimitiveTypeTriangle, MTLIndexTypeUInt16, MTLIndexType};
use cocoa::foundation::{NSInteger, NSTimeInterval};
use std::os::raw::{c_float,c_void};
use core_animation::{DispatchSemaphore, MTLPixelFormatBGRA8Unorm, DISPATCH_TIME_FOREVER};
use objc::runtime::YES;
use crate::{vector_float3, debug_log};
use crate::vector_float4;
use crate::matrix_types::matrix_float4x4;
use std::mem::size_of;
use cocoa::base::id;

// Use the Apple versions rather than linking in the Rust stdlib for them.
extern {
    // usr/include/math.h:extern float sinf(float);
    fn sinf(value: c_float) -> c_float;
}
#[inline]
fn sinf32(value: f32) -> f32 { unsafe { sinf(value) } }

// #import "MetalView.h"
//
// @interface MBERenderer : NSObject <MetalViewDelegate>
//
// @end
// ================================================================================================
// //  MBERenderer.m
// //  DrawingIn3DIOS
// //
// //  Original Copyright (c) 2015 Warren Moore
// //  from https://github.com/metal-by-example/sample-code
// //  Licensed under MIT.
// //
// //  Updates by TR Solutions on 18/7/20.
// //  Copyright © 2020 TR Solutions Pte. Ltd.
// //  Licensed under Apache 2.0 and MIT
// //
// //  See appropriate LICENCE files for details.
// //
// #import "MBERenderer.h"
// #import "MBEMathUtilities.h"
//
// @import Metal;
// @import QuartzCore.CAMetalLayer;
// @import simd;
//
// static const NSInteger MBEInFlightBufferCount = 3;
const MBE_IN_FLIGHT_BUFFER_COUNT:NSInteger = 3;
//
// typedef uint16_t MBEIndex;
// const MTLIndexType MBEIndexType = MTLIndexTypeUInt16;
type MBEIndex = u16;
const MBE_INDEX_TYPE: MTLIndexType = MTLIndexTypeUInt16;
//
// typedef struct
// {
//     vector_float4 position;
//     vector_float4 color;
// } MBEVertex;
struct MBEVertex {
    position: vector_float4,
    color: vector_float4,
}
impl From<[f32;8]> for MBEVertex {
    fn from(values: [f32; 8]) -> Self {
        MBEVertex {
            position: vector_float4::new (values[0], values[1], values[2], values[3]),
            color:    vector_float4::new (values[4], values[5], values[6], values[7]),
        }
    }
}
//
// typedef struct
// {
//     matrix_float4x4 modelViewProjectionMatrix;
// } MBEUniforms;
struct MBEUniforms {
    model_view_projection_matrix: matrix_float4x4,
}
//
// static inline uint64_t AlignUp(uint64_t n, uint32_t alignment) {
//     return ((n + alignment - 1) / alignment) * alignment;
// }
/// Aligns the given address to the given alignment
#[inline]
fn align_up(n: u64, alignment: u32) -> u64 {
    let alignment = alignment as u64;
    ((n + alignment - 1) / alignment) * alignment
}
//
// static const uint32_t MBEBufferAlignment = 256;
const MBE_BUFFER_ALIGNMENT: u32 = 256;
//
// @interface MBERenderer ()
/// The renderer for our 3D cube
#[derive(Default)]
pub struct RustMBERenderer {
    // @property (strong) id<MTLDevice> device;
    device: MetalDevice,
    // @property (strong) id<MTLBuffer> vertexBuffer;
    vertex_buffer: MetalBuffer,
    // @property (strong) id<MTLBuffer> indexBuffer;
    index_buffer: MetalBuffer,
    // @property (strong) id<MTLBuffer> uniformBuffer;
    uniform_buffer: MetalBuffer,
    // @property (strong) id<MTLCommandQueue> commandQueue;
    command_queue: MetalCommandQueue,
    // @property (strong) id<MTLRenderPipelineState> renderPipelineState;
    render_pipeline_state: MetalRenderPipelineState,
    // @property (strong) id<MTLDepthStencilState> depthStencilState;
    depth_stencil_state: MetalDepthStencilState,
    // @property (strong) dispatch_semaphore_t displaySemaphore;
    display_semaphore: DispatchSemaphore,
    // @property (assign) NSInteger bufferIndex;
    buffer_index: NSInteger,
    // @property (assign) float rotationX, rotationY, time;
    rotation_x: c_float,
    rotation_y: c_float,
    time: c_float,
}
// @end

// pub struct RustMBERenderer {
//     renderer: id,
// }
// impl Default for RustMBERenderer {
//     fn default() -> Self { RustMBERenderer { renderer : nil } }
// }
// impl From<id> for RustMBERenderer {
//     fn from(renderer: id) -> Self {
//         let renderer = unsafe {objc_retain(renderer) };
//         RustMBERenderer { renderer }
//     }
// }
// impl Drop for RustMBERenderer {
//     fn drop(&mut self) { unsafe { objc_release(self.renderer) } }
// }
//
// @implementation MBERenderer
impl RustMBERenderer {
    /// Creates a new MBE Renderer
    pub fn new() -> Result<Self, MetalDeviceError> {
        // if ((self = [super init]))
        // {
        //     _device = MTLCreateSystemDefaultDevice();
        //     _displaySemaphore = dispatch_semaphore_create(MBEInFlightBufferCount);
        //     [self makePipeline];
        //     [self makeBuffers];
        // }
        //
        // return self;
        let mut device = MetalDevice::create_system_default_device();
        let display_semaphore = DispatchSemaphore::create(MBE_IN_FLIGHT_BUFFER_COUNT);
        let (
            command_queue,
            depth_stencil_state,
            render_pipeline_state
        ) = Self::make_pipeline(&mut device)?;
        let (
            vertex_buffer,
            index_buffer,
            uniform_buffer
        ) = Self::make_buffers(&mut device);
        Ok(RustMBERenderer {
            device,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            command_queue,
            render_pipeline_state,
            depth_stencil_state,
            display_semaphore,
            buffer_index: 0,
            rotation_x: 0.0,
            rotation_y: 0.0,
            time: 0.0
        })
    }
    //
    // - (instancetype)init
    // {
    //     if ((self = [super init]))
    //     {
    //         _device = MTLCreateSystemDefaultDevice();
    //         _displaySemaphore = dispatch_semaphore_create(MBEInFlightBufferCount);
    //         [self makePipeline];
    //         [self makeBuffers];
    //     }
    //
    //     return self;
    // }
    //
    // - (void)makePipeline
    // {
    fn make_pipeline(device: &mut MetalDevice) -> Result<(MetalCommandQueue, MetalDepthStencilState, MetalRenderPipelineState), MetalDeviceError> {
        // self.commandQueue = [self.device newCommandQueue];
        //
        // id<MTLLibrary> library = [self.device newDefaultLibrary];
        let library = device.new_default_library();
        //
        // MTLRenderPipelineDescriptor *pipelineDescriptor = [MTLRenderPipelineDescriptor new];
        let mut pipeline_descriptor = MetalRenderPipelineDescriptor::new();
        // pipelineDescriptor.vertexFunction = [library newFunctionWithName:@"vertex_project"];
        // pipelineDescriptor.fragmentFunction = [library newFunctionWithName:@"fragment_flatcolor"];
        pipeline_descriptor.set_vertex_function(library.new_function_with_name("vertex_project"));
        pipeline_descriptor.set_fragment_function(library.new_function_with_name("fragment_flatcolor"));
        // pipelineDescriptor.colorAttachments[0].pixelFormat = MTLPixelFormatBGRA8Unorm;
        // pipelineDescriptor.depthAttachmentPixelFormat = MTLPixelFormatDepth32Float;
        pipeline_descriptor.set_color_attachment_pixel_format(0, MTLPixelFormatBGRA8Unorm);
        pipeline_descriptor.set_depth_attachment_pixel_format(MTLPixelFormatDepth32Float);
        //
        // MTLDepthStencilDescriptor *depthStencilDescriptor = [MTLDepthStencilDescriptor new];
        let mut depth_stencil_descriptor = MetalDepthStencilDescriptor::new();

        // depthStencilDescriptor.depthCompareFunction = MTLCompareFunctionLess;
        // depthStencilDescriptor.depthWriteEnabled = YES;
        // self.depthStencilState = [self.device newDepthStencilStateWithDescriptor:depthStencilDescriptor];
        depth_stencil_descriptor.set_depth_compare_function(MTLCompareFunctionLess);
        depth_stencil_descriptor.set_depth_write_enabled(YES);
        let depth_stencil_state = device.new_depth_stencil_state_with_descriptor(depth_stencil_descriptor);
        //
        // NSError *error = nil;
        // self.renderPipelineState = [self.device newRenderPipelineStateWithDescriptor:pipelineDescriptor
        //                                                                        error:&error];
        //
        // if (!self.renderPipelineState)
        // {
        //     NSLog(@"Error occurred when creating render pipeline state: %@", error);
        // }
        let render_pipeline_state = device.new_render_pipeline_state_with_descriptor(pipeline_descriptor)?;
        //
        // self.commandQueue = [self.device newCommandQueue];
        Ok((device.new_command_queue(), depth_stencil_state, render_pipeline_state))
    }
    // }
    //
    // - (void)makeBuffers
    // {
    fn make_buffers(device: &mut MetalDevice) -> (MetalBuffer, MetalBuffer, MetalBuffer) {
        // static const MBEVertex vertices[] =
        // {
        //     { .position = { -1,  1,  1, 1 }, .color = { 0, 1, 1, 1 } },
        //     { .position = { -1, -1,  1, 1 }, .color = { 0, 0, 1, 1 } },
        //     { .position = {  1, -1,  1, 1 }, .color = { 1, 0, 1, 1 } },
        //     { .position = {  1,  1,  1, 1 }, .color = { 1, 1, 1, 1 } },
        //     { .position = { -1,  1, -1, 1 }, .color = { 0, 1, 0, 1 } },
        //     { .position = { -1, -1, -1, 1 }, .color = { 0, 0, 0, 1 } },
        //     { .position = {  1, -1, -1, 1 }, .color = { 1, 0, 0, 1 } },
        //     { .position = {  1,  1, -1, 1 }, .color = { 1, 1, 0, 1 } }
        // };
        let vertices:[MBEVertex;8] = [
            //     { .position = {  -1,  1,  1,  1 }, .color = { 0,  1,  1,  1 } },
            MBEVertex::from([-1., 1., 1., 1.,             0., 1., 1., 1.]),
            //     { .position = {  -1, -1,  1,  1 }, .color = { 0,  0,  1,  1 } },
            MBEVertex::from([-1.,-1., 1., 1.,             0., 0., 1., 1.]),
            //     { .position = {   1, -1,  1,  1 }, .color = { 1,  0,  1,  1 } },
            MBEVertex::from([ 1.,-1., 1., 1.,             1., 0., 1., 1.]),
            //     { .position = {   1,  1,  1,  1 }, .color = { 1,  1,  1,  1 } },
            MBEVertex::from([ 1., 1., 1., 1.,             1., 1., 1., 1.]),
            //     { .position = {  -1,  1, -1,  1 }, .color = { 0,  1,  0,  1 } },
            MBEVertex::from([-1., 1.,-1., 1.,             0., 1., 0., 1.]),
            //     { .position = {  -1, -1, -1,  1 }, .color = { 0,  0,  0,  1 } },
            MBEVertex::from([-1.,-1.,-1., 1.,             0., 0., 0., 1.]),
            //     { .position = {   1, -1, -1,  1 }, .color = { 1,  0,  0,  1 } },
            MBEVertex::from([ 1.,-1.,-1., 1.,             1., 0., 0., 1.]),
            //     { .position = {   1,  1, -1,  1 }, .color = { 1,  1,  0,  1 } }
            MBEVertex::from([ 1., 1.,-1., 1.,             1., 1., 0., 1.]),
        ];
        //
        // static const MBEIndex indices[] =
        // {
        //     3, 2, 6, 6, 7, 3,
        //     4, 5, 1, 1, 0, 4,
        //     4, 0, 3, 3, 7, 4,
        //     1, 5, 6, 6, 2, 1,
        //     0, 1, 2, 2, 3, 0,
        //     7, 6, 5, 5, 4, 7
        // };
        let indices:[MBEIndex;6*6] = [
                3, 2, 6, 6, 7, 3,
                4, 5, 1, 1, 0, 4,
                4, 0, 3, 3, 7, 4,
                1, 5, 6, 6, 2, 1,
                0, 1, 2, 2, 3, 0,
                7, 6, 5, 5, 4, 7
        ];
        //
        // _vertexBuffer = [self.device newBufferWithBytes:vertices
        //                                          length:sizeof(vertices)
        //                                         options:MTLResourceOptionCPUCacheModeDefault];
        // [_vertexBuffer setLabel:@"Vertices"];
        let mut vertex_buffer = device.new_buffer_with_bytes_and_options(
            &vertices,
            MTLResourceCPUCacheModeDefaultCache // same as MTLResourceOptionCPUCacheModeDefault
        );
        vertex_buffer.set_label("Vertices");
        //
        // _indexBuffer = [self.device newBufferWithBytes:indices
        //                                         length:sizeof(indices)
        //                                        options:MTLResourceOptionCPUCacheModeDefault];
        // [_indexBuffer setLabel:@"Indices"];
        let mut index_buffer = device.new_buffer_with_bytes_and_options(
            &indices,
            MTLResourceCPUCacheModeDefaultCache,
        );
        index_buffer.set_label("Indices");
        //
        // _uniformBuffer = [self.device newBufferWithLength:AlignUp(sizeof(MBEUniforms), MBEBufferAlignment) * MBEInFlightBufferCount
        //                                           options:MTLResourceOptionCPUCacheModeDefault];
        // [_uniformBuffer setLabel:@"Uniforms"];
        let length = size_of::<MBEUniforms>();
        let length = align_up(length as _, MBE_BUFFER_ALIGNMENT) * (MBE_IN_FLIGHT_BUFFER_COUNT as u64);
        let length_str = format!("uniform buffer length = {}", length);
        debug_log(length_str.as_str());
        let mut uniform_buffer = device.new_buffer_with_length_and_options(
            length as _,
            MTLResourceCPUCacheModeDefaultCache
        );
        uniform_buffer.set_label("Uniforms");
        (vertex_buffer, index_buffer, uniform_buffer)
    }
    // }
    //
    // - (void)updateUniformsForView:(MetalView *)view duration:(NSTimeInterval)duration
    // {
    fn update_uniforms_for_view(&self, view: &RustMetalView, _duration: NSTimeInterval) {
        // float scaleFactor = sinf(5 * self.time) * 0.25 + 1;
        let scale_factor = sinf32(self.time * 5.) * 0.25 + 1.;

        // const vector_float3 xAxis = { 1, 0, 0 };
        // const vector_float3 yAxis = { 0, 1, 0 };
        let x_axis = vector_float3::from([1., 0., 0.] );
        let y_axis = vector_float3::from([0., 1., 0.] );
        // const matrix_float4x4 xRot = matrix_float4x4_rotation(xAxis, self.rotationX);
        let x_rot = matrix_float4x4::rotation(x_axis, self.rotation_x);
        // const matrix_float4x4 yRot = matrix_float4x4_rotation(yAxis, self.rotationY);
        let y_rot = matrix_float4x4::rotation(y_axis, self.rotation_y);
        // const matrix_float4x4 scale = matrix_float4x4_uniform_scale(scaleFactor);
        let scale = matrix_float4x4::uniform_scale(scale_factor);
        // const matrix_float4x4 modelMatrix = matrix_multiply(matrix_multiply(xRot, yRot), scale);
        let model_matrix = (x_rot * y_rot) * scale;
        // let model_matrix_msg = format!("Model Matrix: {:?}", model_matrix);
        // debug_log(model_matrix_msg.as_str());
        //
        // const vector_float3 cameraTranslation = { 0, 0, -5 };
        // const matrix_float4x4 viewMatrix = matrix_float4x4_translation(cameraTranslation);
        let camera_translation = vector_float3::from([0., 0., -5.]);
        let view_matrix = matrix_float4x4::translation(camera_translation);
        //
        // const CGSize drawableSize = view.metalLayer.drawableSize;
        let drawable_size = view.get_metal_layer().get_drawable_size();
        // const float aspect = drawableSize.width / drawableSize.height;
        let aspect = (drawable_size.width / drawable_size.height) as f32;
        // const float fov = (2 * M_PI) / 5;
        // const float near = 1;
        // const float far = 100;
        let fov = (2. * std::f32::consts::PI) / 5.;
        let near = 1.;
        let far = 100.;
        // const matrix_float4x4 projectionMatrix = matrix_float4x4_perspective(aspect, fov, near, far);
        let projection_matrix = matrix_float4x4::perspective(aspect, fov, near, far);
        //
        // MBEUniforms uniforms;
        // uniforms.modelViewProjectionMatrix = matrix_multiply(projectionMatrix, matrix_multiply(viewMatrix, modelMatrix));
        let uniforms = MBEUniforms {
            model_view_projection_matrix: projection_matrix * (view_matrix * model_matrix),
        };
        //
        // const NSUInteger uniformBufferOffset = AlignUp(sizeof(MBEUniforms), MBEBufferAlignment) * self.bufferIndex;
        let uniform_buffer_offset = self.uniform_buffer_offset();
        // memcpy([self.uniformBuffer contents] + uniformBufferOffset, &uniforms, sizeof(uniforms));
        let contents = self.uniform_buffer.get_contents() as *mut u8;
        //+ let print_string = format!("Contents = {:?}", contents);
        //+ debug_log(print_string.as_str());
        unsafe {
            let write_position = contents.offset(uniform_buffer_offset as isize) as *mut MBEUniforms;
            write_position.write(uniforms);
        }
    }
    // }
    //
    // TOD: this function was residing in RustMetal view, but is only used by the renderer.
    // to support the function we needed to update the metal view, but we are owned by it
    // so we can't in Rust. So, it was moved to here.
    fn get_current_render_pass_descriptor_for_view_with_clear_color(&self, view: &RustMetalView, clear_color: MetalClearColor) -> MetalRenderPassDescriptor {
        // MTLRenderPassDescriptor *passDescriptor = [MTLRenderPassDescriptor renderPassDescriptor];
        let mut pass_descriptor = MetalRenderPassDescriptor::render_pass_descriptor();
        //
        // passDescriptor.colorAttachments[0].texture = [self.currentDrawable texture];
        // passDescriptor.colorAttachments[0].clearColor = self.clearColor;
        // passDescriptor.colorAttachments[0].storeAction = MTLStoreActionStore;
        // passDescriptor.colorAttachments[0].loadAction = MTLLoadActionClear;
        pass_descriptor.set_color_attachments_texture(0, view.get_current_drawable().get_texture());
        pass_descriptor.set_color_attachments_clear_color(0, clear_color);
        pass_descriptor.set_color_attachments_store_action(0, MTLStoreActionStore);
        pass_descriptor.set_color_attachments_load_action(0, MTLLoadActionClear);
        //
        // passDescriptor.depthAttachment.texture = self.depthTexture;
        // passDescriptor.depthAttachment.clearDepth = 1.0;
        // passDescriptor.depthAttachment.loadAction = MTLLoadActionClear;
        // passDescriptor.depthAttachment.storeAction = MTLStoreActionDontCare;
        let mut depth_attachment = pass_descriptor.get_depth_attachment();
        depth_attachment.set_texture(view.get_depth_texture());
        depth_attachment.set_clear_depth(1.);
        depth_attachment.set_load_action(MTLLoadActionClear);
        depth_attachment.set_store_action(MTLStoreActionDontCare);
        //
        // return passDescriptor;
        pass_descriptor
    }
    /// We use this a lot
    #[inline]
    fn uniform_buffer_offset(&self) -> u64 {
        align_up(
            size_of::<MBEUniforms>() as u64,
            MBE_BUFFER_ALIGNMENT
        ) * (self.buffer_index as u64)
    }
}
// }
//
// @end
//
impl RustMetalViewDelegate for RustMBERenderer {
    // - (void)drawInView:(MetalView *)view
    // {
    fn draw_in_view(&self, view: &RustMetalView) {
        // dispatch_semaphore_wait(self.displaySemaphore, DISPATCH_TIME_FOREVER);
        self.display_semaphore.wait(DISPATCH_TIME_FOREVER);
        //+ let message = format!("display semaphore: {:?}", self.display_semaphore);
        //+ debug_log(message.as_str());
        //
        // TOD: Rust won't let us update, because the view owns this delegate
        // It's only needed for pass descriptor, anyway, so we moved the code
        // down into the renderer.
        // view.clearColor = MTLClearColorMake(0.95, 0.95, 0.95, 1);
        let clear_color = MetalClearColor::make(
            0.95, 0.95, 0.95, 1.
        );
        //
        // [self updateUniformsForView:view duration:view.frameDuration];
        self.update_uniforms_for_view(view, view.get_frame_duration());
        //
        // id<MTLCommandBuffer> commandBuffer = [self.commandQueue commandBuffer];
        let mut command_buffer = self.command_queue.command_buffer();
        //
        // MTLRenderPassDescriptor *passDescriptor = [view currentRenderPassDescriptor];
        let pass_descriptor = self.get_current_render_pass_descriptor_for_view_with_clear_color(view, clear_color);
        //
        // id<MTLRenderCommandEncoder> renderPass = [commandBuffer renderCommandEncoderWithDescriptor:passDescriptor];
        let mut render_pass = command_buffer.render_command_encoder_with_descriptor(&pass_descriptor);
        // [renderPass setRenderPipelineState:self.renderPipelineState];
        // [renderPass setDepthStencilState:self.depthStencilState];
        // [renderPass setFrontFacingWinding:MTLWindingCounterClockwise];
        // [renderPass setCullMode:MTLCullModeBack];
        render_pass.set_render_pipeline_state(&self.render_pipeline_state);
        render_pass.set_depth_stencil_state(&self.depth_stencil_state);
        render_pass.set_front_facing_winding(MTLWindingCounterClockwise);
        render_pass.set_cull_mode(MTLCullModeBack);
        //
        // const NSUInteger uniformBufferOffset = AlignUp(sizeof(MBEUniforms), MBEBufferAlignment) * self.bufferIndex;
        let uniform_buffer_offset = self.uniform_buffer_offset();
        //
        // [renderPass setVertexBuffer:self.vertexBuffer offset:0 atIndex:0];
        // [renderPass setVertexBuffer:self.uniformBuffer offset:uniformBufferOffset atIndex:1];
        render_pass.set_vertex_buffer(&self.vertex_buffer, 0, 0);
        render_pass.set_vertex_buffer(&self.uniform_buffer, uniform_buffer_offset, 1);
        //
        // [renderPass drawIndexedPrimitives:MTLPrimitiveTypeTriangle
        //                        indexCount:[self.indexBuffer length] / sizeof(MBEIndex)
        //                         indexType:MBEIndexType
        //                       indexBuffer:self.indexBuffer
        //                 indexBufferOffset:0];
        render_pass.draw_indexed_primitives_with_count_and_type_and_buffer_and_offset (
            MTLPrimitiveTypeTriangle,
            self.index_buffer.get_length() / (size_of::<MBEIndex>() as u64),
            MBE_INDEX_TYPE,
            &self.index_buffer,
            0
        );
        //
        // [renderPass endEncoding];
        render_pass.end_encoding();
        //
        // [commandBuffer presentDrawable:view.currentDrawable];
        command_buffer.present_drawable(view.get_current_drawable());
        //
        // [commandBuffer addCompletedHandler:^(id<MTLCommandBuffer> commandBuffer) {
        //     self.bufferIndex = (self.bufferIndex + 1) % MBEInFlightBufferCount;
        //     dispatch_semaphore_signal(self.displaySemaphore);
        // }];
        let self_addr = self as *const _ as *const c_void;
        //+ let self_addr_description = format!("self address: {:?}", self_addr);
        //+ debug_log(self_addr_description.as_str());
        command_buffer.attach_completion_function(
            command_buffer_completion_handler_f,
            self_addr
        );
        //
        // [commandBuffer commit];
        command_buffer.commit()
    }
    // }
    // TOD: moved update code to this fn as we can't update
    // the delegate and pass a pointer to the metal view in Rust.
    fn update_time_and_rotation(&mut self, duration: NSTimeInterval) {
        // self.time += duration;
        // self.rotationX += duration * (M_PI / 2);
        // self.rotationY += duration * (M_PI / 3);
        let duration = duration as f32;
        self.time += duration;
        self.rotation_x += duration * (std::f32::consts::PI / 2.);
        self.rotation_y += duration * (std::f32::consts::PI / 3.);
    }

}

extern "C" fn command_buffer_completion_handler_f(_source: id, user_data: *const c_void)
    /*where F:Fn(id)*/
{
    // [commandBuffer addCompletedHandler:^(id<MTLCommandBuffer> commandBuffer) {
    //     self.bufferIndex = (self.bufferIndex + 1) % MBEInFlightBufferCount;
    //     dispatch_semaphore_signal(self.displaySemaphore);
    // }];
    //+ let message = format!("In command buffer completion handler f, user_data = {:?}", user_data);
    //+ debug_log(message.as_str());
    let user_data_link_ptr = user_data as *mut RustMBERenderer;
    match unsafe { user_data_link_ptr.as_mut() } {
        Some(renderer_p) => {
            // Note: this really shouldn't at this point. should be just
            // after you pushed the command onto the queue.
            renderer_p.buffer_index = (renderer_p.buffer_index + 1) % MBE_IN_FLIGHT_BUFFER_COUNT;
            //+ let message = format!("display semaphore: {:?}", renderer_p.display_semaphore);
            //+ debug_log(message.as_str());
            renderer_p.display_semaphore.signal();
        }
        None => ()
    }
}

/*
// Rust:
rust: x rot: matrix_float4x4 { _private: [vector_float4 { _private: [1.000000, 0.000000, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 0.998629, -0.052344, 0.000000] }, vector_float4 { _private: [0.000000, 0.052345, 0.998629, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
objc: x rot: matrix_float4x4 { _private: [vector_float4 { _private: [1.000000, 0.000000, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 0.998629, -0.052345, 0.000000] }, vector_float4 { _private: [0.000000, 0.052345, 0.998629, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }

rust: y rot: matrix_float4x4 { _private: [vector_float4 { _private: [0.999391, 0.000000, 0.034906, 0.000000] }, vector_float4 { _private: [0.000000, 1.000000, 0.000000, 0.000000] }, vector_float4 { _private: [-0.034905, 0.000000, 0.999391, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
objc: y rot: matrix_float4x4 { _private: [vector_float4 { _private: [0.999391, 0.000000, 0.034906, 0.000000] }, vector_float4 { _private: [0.000000, 1.000000, 0.000000, 0.000000] }, vector_float4 { _private: [-0.034906, 0.000000, 0.999391, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }

rust: scale: matrix_float4x4 { _private: [vector_float4 { _private: [1.041481, 0.000000, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 1.041481, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 1.041481, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
objc: scale: matrix_float4x4 { _private: [vector_float4 { _private: [1.041481, 0.000000, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 1.041481, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 1.041481, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }

rust: Rot Matrix: matrix_float4x4 { _private: [vector_float4 { _private: [0.999391, 0.000000, 0.03490543, 0.0] }, vector_float4 { _private: [0.0018271194, 0.9986291, -0.052312948, 0.0] }, vector_float4 { _private: [-0.03485758, 0.052344847, 0.9980205, 0.0] }, vector_float4 { _private: [0.0, 0.0, 0.0, 1.0] }] }
objc: Rot Matrix: matrix_float4x4 { _private: [vector_float4 { _private: [0.999391, 0.001827, 0.034859, 0.000000] }, vector_float4 { _private: [0.000000, 0.998629, -0.052347, 0.000000] }, vector_float4 { _private: [-0.034907, 0.052315, 0.998020, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }

rust: Model Matrix: matrix_float4x4 { _private: [vector_float4 { _private: [1.0408459, 0.0, 0.036352914, 0.0] }, vector_float4 { _private: [0.0019028664, 1.0400528, -0.054482307, 0.0] }, vector_float4 { _private: [-0.036303077, 0.054515526, 1.039419, 0.0] }, vector_float4 { _private: [0.0, 0.0, 0.0, 1.0] }] }
objc: Model Matrix: matrix_float4x4 { _private: [vector_float4 { _private: [1.040847, 0.001903, 0.036304, 0.000000] }, vector_float4 { _private: [0.000000, 1.040054, -0.054517, 0.000000] }, vector_float4 { _private: [-0.036354, 0.054484, 1.039420, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }

rust: x rot: matrix_float4x4 { _private: [vector_float4 { _private: [1.0, 0.0, 0.0, 0.0] }, vector_float4 { _private: [0.0, 0.9945206, -0.10454064, 0.0] }, vector_float4 { _private: [0.0, 0.10454064, 0.9945206, 0.0] }, vector_float4 { _private: [0.0, 0.0, 0.0, 1.0] }] }
rust: y rot: matrix_float4x4 { _private: [vector_float4 { _private: [0.9975635, 0.0, 0.069764614, 0.0] }, vector_float4 { _private: [0.0, 1.0, 0.0, 0.0] }, vector_float4 { _private: [-0.069764614, 0.0, 0.9975635, 0.0] }, vector_float4 { _private: [0.0, 0.0, 0.0, 1.0] }] }
rust: scale: matrix_float4x4 { _private: [vector_float4 { _private: [1.0818079, 0.0, 0.0, 0.0] }, vector_float4 { _private: [0.0, 1.0818079, 0.0, 0.0] }, vector_float4 { _private: [0.0, 0.0, 1.0818079, 0.0] }, vector_float4 { _private: [0.0, 0.0, 0.0, 1.0] }] }
rust: Model Matrix: matrix_float4x4 { _private: [vector_float4 { _private: [1.079172, 0.0, 0.07547191, 0.0] }, vector_float4 { _private: [0.007889882, 1.0758802, -0.11281733, 0.0] }, vector_float4 { _private: [-0.07505837, 0.113092884, 1.0732588, 0.0] }, vector_float4 { _private: [0.0, 0.0, 0.0, 1.0] }] }

// objc:
objc: x rot: matrix_float4x4 { _private: [vector_float4 { _private: [1.000000, 0.000000, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 0.998629, -0.052345, 0.000000] }, vector_float4 { _private: [0.000000, 0.052345, 0.998629, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
objc: y rot: matrix_float4x4 { _private: [vector_float4 { _private: [0.999391, 0.000000, 0.034906, 0.000000] }, vector_float4 { _private: [0.000000, 1.000000, 0.000000, 0.000000] }, vector_float4 { _private: [-0.034906, 0.000000, 0.999391, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
objc: scale: matrix_float4x4 { _private: [vector_float4 { _private: [1.041481, 0.000000, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 1.041481, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 1.041481, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
objc: Model Matrix: matrix_float4x4 { _private: [vector_float4 { _private: [1.040847, 0.001903, 0.036304, 0.000000] }, vector_float4 { _private: [0.000000, 1.040054, -0.054517, 0.000000] }, vector_float4 { _private: [-0.036354, 0.054484, 1.039420, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
objc: x rot: matrix_float4x4 { _private: [vector_float4 { _private: [1.000000, 0.000000, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 0.994521, -0.104541, 0.000000] }, vector_float4 { _private: [0.000000, 0.104541, 0.994521, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
objc: y rot: matrix_float4x4 { _private: [vector_float4 { _private: [0.997563, 0.000000, 0.069765, 0.000000] }, vector_float4 { _private: [0.000000, 1.000000, 0.000000, 0.000000] }, vector_float4 { _private: [-0.069765, 0.000000, 0.997563, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
objc: scale: matrix_float4x4 { _private: [vector_float4 { _private: [1.081808, 0.000000, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 1.081808, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 1.081808, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
objc: Model Matrix: matrix_float4x4 { _private: [vector_float4 { _private: [1.079172, 0.007890, 0.075058, 0.000000] }, vector_float4 { _private: [0.000000, 1.075880, -0.113093, 0.000000] }, vector_float4 { _private: [-0.075472, 0.112817, 1.073259, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }


 */