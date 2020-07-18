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

use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use objc::runtime::{objc_retain, objc_release};

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
//
// typedef uint16_t MBEIndex;
// const MTLIndexType MBEIndexType = MTLIndexTypeUInt16;
//
// typedef struct
// {
//     vector_float4 position;
//     vector_float4 color;
// } MBEVertex;
//
// typedef struct
// {
//     matrix_float4x4 modelViewProjectionMatrix;
// } MBEUniforms;
//
// static inline uint64_t AlignUp(uint64_t n, uint32_t alignment) {
//     return ((n + alignment - 1) / alignment) * alignment;
// }
//
// static const uint32_t MBEBufferAlignment = 256;
//
// @interface MBERenderer ()
// @property (strong) id<MTLDevice> device;
// @property (strong) id<MTLBuffer> vertexBuffer;
// @property (strong) id<MTLBuffer> indexBuffer;
// @property (strong) id<MTLBuffer> uniformBuffer;
// @property (strong) id<MTLCommandQueue> commandQueue;
// @property (strong) id<MTLRenderPipelineState> renderPipelineState;
// @property (strong) id<MTLDepthStencilState> depthStencilState;
// @property (strong) dispatch_semaphore_t displaySemaphore;
// @property (assign) NSInteger bufferIndex;
// @property (assign) float rotationX, rotationY, time;
// @end

/// Temporary Rust wrapper for the objc renderer
pub struct RustMBERenderer {
    renderer: id,
}
impl Default for RustMBERenderer {
    fn default() -> Self { RustMBERenderer { renderer : nil } }
}
impl From<id> for RustMBERenderer {
    fn from(renderer: id) -> Self {
        let renderer = unsafe {objc_retain(renderer) };
        RustMBERenderer { renderer }
    }
}
impl Drop for RustMBERenderer {
    fn drop(&mut self) { unsafe { objc_release(self.renderer) } }
}
//
// @implementation MBERenderer
impl RustMBERenderer {
    /// Creates a new MBE Renderer and wraps it in Rust
    pub fn new() -> Self {
        let class = class!(MBERenderer);
        let renderer:id = unsafe { msg_send![class, new] };
        RustMBERenderer::from(renderer)
    }
    /// Returns the underlying objective c renderer
    pub fn to_objc(&self) -> id { self.renderer }
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
    //     self.commandQueue = [self.device newCommandQueue];
    //
    //     id<MTLLibrary> library = [self.device newDefaultLibrary];
    //
    //     MTLRenderPipelineDescriptor *pipelineDescriptor = [MTLRenderPipelineDescriptor new];
    //     pipelineDescriptor.vertexFunction = [library newFunctionWithName:@"vertex_project"];
    //     pipelineDescriptor.fragmentFunction = [library newFunctionWithName:@"fragment_flatcolor"];
    //     pipelineDescriptor.colorAttachments[0].pixelFormat = MTLPixelFormatBGRA8Unorm;
    //     pipelineDescriptor.depthAttachmentPixelFormat = MTLPixelFormatDepth32Float;
    //
    //     MTLDepthStencilDescriptor *depthStencilDescriptor = [MTLDepthStencilDescriptor new];
    //     depthStencilDescriptor.depthCompareFunction = MTLCompareFunctionLess;
    //     depthStencilDescriptor.depthWriteEnabled = YES;
    //     self.depthStencilState = [self.device newDepthStencilStateWithDescriptor:depthStencilDescriptor];
    //
    //     NSError *error = nil;
    //     self.renderPipelineState = [self.device newRenderPipelineStateWithDescriptor:pipelineDescriptor
    //                                                                            error:&error];
    //
    //     if (!self.renderPipelineState)
    //     {
    //         NSLog(@"Error occurred when creating render pipeline state: %@", error);
    //     }
    //
    //     self.commandQueue = [self.device newCommandQueue];
    // }
    //
    // - (void)makeBuffers
    // {
    //     static const MBEVertex vertices[] =
    //     {
    //         { .position = { -1,  1,  1, 1 }, .color = { 0, 1, 1, 1 } },
    //         { .position = { -1, -1,  1, 1 }, .color = { 0, 0, 1, 1 } },
    //         { .position = {  1, -1,  1, 1 }, .color = { 1, 0, 1, 1 } },
    //         { .position = {  1,  1,  1, 1 }, .color = { 1, 1, 1, 1 } },
    //         { .position = { -1,  1, -1, 1 }, .color = { 0, 1, 0, 1 } },
    //         { .position = { -1, -1, -1, 1 }, .color = { 0, 0, 0, 1 } },
    //         { .position = {  1, -1, -1, 1 }, .color = { 1, 0, 0, 1 } },
    //         { .position = {  1,  1, -1, 1 }, .color = { 1, 1, 0, 1 } }
    //     };
    //
    //     static const MBEIndex indices[] =
    //     {
    //         3, 2, 6, 6, 7, 3,
    //         4, 5, 1, 1, 0, 4,
    //         4, 0, 3, 3, 7, 4,
    //         1, 5, 6, 6, 2, 1,
    //         0, 1, 2, 2, 3, 0,
    //         7, 6, 5, 5, 4, 7
    //     };
    //
    //     _vertexBuffer = [self.device newBufferWithBytes:vertices
    //                                              length:sizeof(vertices)
    //                                             options:MTLResourceOptionCPUCacheModeDefault];
    //     [_vertexBuffer setLabel:@"Vertices"];
    //
    //     _indexBuffer = [self.device newBufferWithBytes:indices
    //                                             length:sizeof(indices)
    //                                            options:MTLResourceOptionCPUCacheModeDefault];
    //     [_indexBuffer setLabel:@"Indices"];
    //
    //     _uniformBuffer = [self.device newBufferWithLength:AlignUp(sizeof(MBEUniforms), MBEBufferAlignment) * MBEInFlightBufferCount
    //                                               options:MTLResourceOptionCPUCacheModeDefault];
    //     [_uniformBuffer setLabel:@"Uniforms"];
    // }
    //
    // - (void)updateUniformsForView:(MetalView *)view duration:(NSTimeInterval)duration
    // {
    //     self.time += duration;
    //     self.rotationX += duration * (M_PI / 2);
    //     self.rotationY += duration * (M_PI / 3);
    //     float scaleFactor = sinf(5 * self.time) * 0.25 + 1;
    //     const vector_float3 xAxis = { 1, 0, 0 };
    //     const vector_float3 yAxis = { 0, 1, 0 };
    //     const matrix_float4x4 xRot = matrix_float4x4_rotation(xAxis, self.rotationX);
    //     const matrix_float4x4 yRot = matrix_float4x4_rotation(yAxis, self.rotationY);
    //     const matrix_float4x4 scale = matrix_float4x4_uniform_scale(scaleFactor);
    //     const matrix_float4x4 modelMatrix = matrix_multiply(matrix_multiply(xRot, yRot), scale);
    //
    //     const vector_float3 cameraTranslation = { 0, 0, -5 };
    //     const matrix_float4x4 viewMatrix = matrix_float4x4_translation(cameraTranslation);
    //
    //     const CGSize drawableSize = view.metalLayer.drawableSize;
    //     const float aspect = drawableSize.width / drawableSize.height;
    //     const float fov = (2 * M_PI) / 5;
    //     const float near = 1;
    //     const float far = 100;
    //     const matrix_float4x4 projectionMatrix = matrix_float4x4_perspective(aspect, fov, near, far);
    //
    //     MBEUniforms uniforms;
    //     uniforms.modelViewProjectionMatrix = matrix_multiply(projectionMatrix, matrix_multiply(viewMatrix, modelMatrix));
    //
    //     const NSUInteger uniformBufferOffset = AlignUp(sizeof(MBEUniforms), MBEBufferAlignment) * self.bufferIndex;
    //     memcpy([self.uniformBuffer contents] + uniformBufferOffset, &uniforms, sizeof(uniforms));
    // }
    //
    // - (void)drawInView:(MetalView *)view
    // {
    //     dispatch_semaphore_wait(self.displaySemaphore, DISPATCH_TIME_FOREVER);
    //
    //     view.clearColor = MTLClearColorMake(0.95, 0.95, 0.95, 1);
    //
    //     [self updateUniformsForView:view duration:view.frameDuration];
    //
    //     id<MTLCommandBuffer> commandBuffer = [self.commandQueue commandBuffer];
    //
    //     MTLRenderPassDescriptor *passDescriptor = [view currentRenderPassDescriptor];
    //
    //     id<MTLRenderCommandEncoder> renderPass = [commandBuffer renderCommandEncoderWithDescriptor:passDescriptor];
    //     [renderPass setRenderPipelineState:self.renderPipelineState];
    //     [renderPass setDepthStencilState:self.depthStencilState];
    //     [renderPass setFrontFacingWinding:MTLWindingCounterClockwise];
    //     [renderPass setCullMode:MTLCullModeBack];
    //
    //     const NSUInteger uniformBufferOffset = AlignUp(sizeof(MBEUniforms), MBEBufferAlignment) * self.bufferIndex;
    //
    //     [renderPass setVertexBuffer:self.vertexBuffer offset:0 atIndex:0];
    //     [renderPass setVertexBuffer:self.uniformBuffer offset:uniformBufferOffset atIndex:1];
    //
    //     [renderPass drawIndexedPrimitives:MTLPrimitiveTypeTriangle
    //                            indexCount:[self.indexBuffer length] / sizeof(MBEIndex)
    //                             indexType:MBEIndexType
    //                           indexBuffer:self.indexBuffer
    //                     indexBufferOffset:0];
    //
    //     [renderPass endEncoding];
    //
    //     [commandBuffer presentDrawable:view.currentDrawable];
    //
    //     [commandBuffer addCompletedHandler:^(id<MTLCommandBuffer> commandBuffer) {
    //         self.bufferIndex = (self.bufferIndex + 1) % MBEInFlightBufferCount;
    //         dispatch_semaphore_signal(self.displaySemaphore);
    //     }];
    //
    //     [commandBuffer commit];
}
// }
//
// @end
//
