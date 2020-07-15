//
//  MetalView.m
//  DrawingIn2DMacOS
//
//  Created by TR Solutions on 14/7/20.
//  Copyright © 2020 TR Solutions. All rights reserved.
//

#import "MetalView.h"
#import "CAVDisplayLink.h"
@import QuartzCore;
@import Metal;
@import simd;

typedef struct
{
  vector_float4 position;
  vector_float4 color;
} MBEVertex;

@interface MetalView ()
@property (nonatomic, strong) CAVDisplayLink *displayLink;
@property (nonatomic, strong) id<MTLDevice> device;
@property (nonatomic, strong) id<MTLRenderPipelineState> pipeline;
@property (nonatomic, strong) id<MTLCommandQueue> commandQueue;
@property (nonatomic, strong) id<MTLBuffer> vertexBuffer;
@end

@implementation MetalView

/// Returns a Metal-compatible layer.
+(Class) layerClass { return [CAMetalLayer class]; }

/// Since our view is layer-backed and updates itself by modifying its layer,
/// we override this property and change the return value to YES.
- (BOOL) wantsUpdateLayer { return YES; }

/// Makes a new instance of the class from the coder
- (instancetype)initWithCoder:(NSCoder *)aDecoder
{
  if ((self = [super initWithCoder:aDecoder]))
  {
    self.wantsLayer = true;
    [self makeDevice];
    [self makeBuffers];
    [self makePipeline];
    [self makeDisplayLink];
  }
  
  return self;
}

/// Safely deallocates the class
- (void)dealloc
{
  [_displayLink invalidate];
}



#pragma mark - items used by initWithCoder
/// Convenience routine to return the layer as a CAMetalLayer
- (CAMetalLayer *)metalLayer {
  return (CAMetalLayer *)self.layer;
}

/// Makes a new Metal device for drawing
- (void)makeDevice
{
  _device = MTLCreateSystemDefaultDevice();
  self.metalLayer.device = _device;
  self.metalLayer.pixelFormat = MTLPixelFormatBGRA8Unorm;
  //+ NSLog(@"metal layer device set to %@", self.metalLayer.device);
}

/// Makes a new Metal pipeline for drawing commands
- (void)makePipeline
{
  id<MTLLibrary> library = [_device newDefaultLibrary];
  
  id<MTLFunction> vertexFunc = [library newFunctionWithName:@"vertex_main"];
  id<MTLFunction> fragmentFunc = [library newFunctionWithName:@"fragment_main"];
  
  MTLRenderPipelineDescriptor *pipelineDescriptor = [MTLRenderPipelineDescriptor new];
  pipelineDescriptor.colorAttachments[0].pixelFormat = MTLPixelFormatBGRA8Unorm;
  pipelineDescriptor.vertexFunction = vertexFunc;
  pipelineDescriptor.fragmentFunction = fragmentFunc;
  
  NSError *error = nil;
  _pipeline = [_device newRenderPipelineStateWithDescriptor:pipelineDescriptor
                                                      error:&error];
  
  if (!_pipeline)
  {
    NSLog(@"Error occurred when creating render pipeline state: %@", error);
  }
  
  _commandQueue = [_device newCommandQueue];
}

- (void)makeBuffers
{
  static const MBEVertex vertices[] =
  {
    { .position = {  0.0,  0.5, 0, 1 }, .color = { 1, 0, 0, 1 } },
    { .position = { -0.5, -0.5, 0, 1 }, .color = { 0, 1, 0, 1 } },
    { .position = {  0.5, -0.5, 0, 1 }, .color = { 0, 0, 1, 1 } }
  };
  
  _vertexBuffer = [_device newBufferWithBytes:vertices
                                       length:sizeof(vertices)
                                      options:MTLResourceOptionCPUCacheModeDefault];
}

/// Makes a display link timer that will fire when the main display syncs
-(void)makeDisplayLink
{
  self.displayLink = [CAVDisplayLink displayLinkWithTarget:self selector:@selector(displayLinkDidFire:) didFailWithError:nil];
  [self.displayLink addToRunLoop:[NSRunLoop mainRunLoop] forMode:NSRunLoopCommonModes];
}

/** If the wantsLayer property is set to YES, this method will be invoked to return a layer instance. */
-(CALayer*) makeBackingLayer {
  CALayer* layer = [self.class.layerClass layer];
  CGSize viewScale = [self convertSizeToBacking: CGSizeMake(1.0, 1.0)];
  layer.contentsScale = MIN(viewScale.width, viewScale.height);
  return layer;
}

#pragma mark - Display Link Fire actions

/// Called by the notification system when the display link fires
- (void)displayLinkDidFire:(CAVDisplayLink *)displayLink
{
  //+ NSLog(@"In display link did fire.");
  [self redraw];
}

- (void)redraw
{
  id<CAMetalDrawable> drawable = [self.metalLayer nextDrawable];
  id<MTLTexture> framebufferTexture = drawable.texture;
  
  if (drawable)
  {
    MTLRenderPassDescriptor *passDescriptor = [MTLRenderPassDescriptor renderPassDescriptor];
    passDescriptor.colorAttachments[0].texture = framebufferTexture;
    passDescriptor.colorAttachments[0].clearColor = MTLClearColorMake(0.85, 0.85, 0.85, 1);
    passDescriptor.colorAttachments[0].storeAction = MTLStoreActionStore;
    passDescriptor.colorAttachments[0].loadAction = MTLLoadActionClear;
    
    id<MTLCommandBuffer> commandBuffer = [self.commandQueue commandBuffer];
    
    id<MTLRenderCommandEncoder> commandEncoder = [commandBuffer renderCommandEncoderWithDescriptor:passDescriptor];
    [commandEncoder setRenderPipelineState:self.pipeline];
    [commandEncoder setVertexBuffer:self.vertexBuffer offset:0 atIndex:0];
    [commandEncoder drawPrimitives:MTLPrimitiveTypeTriangle vertexStart:0 vertexCount:3];
    [commandEncoder endEncoding];
    
    [commandBuffer presentDrawable:drawable];
    [commandBuffer commit];
  }
}


@end
