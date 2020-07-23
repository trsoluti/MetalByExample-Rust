//  MetalView.m
//  DrawingIn3DIOS
//
//  Original Copyright (c) 2015 Warren Moore
//  from https://github.com/metal-by-example/sample-code
//  Licensed under MIT.
//
//  Updates by TR Solutions on 18/7/23.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//
//  See appropriate LICENCE files for details.
//
#import "MetalView.h"
#import "CAVDisplayLink.h"

@interface MetalView ()
@property (strong) id<CAMetalDrawable> currentDrawable;
@property (assign) NSTimeInterval frameDuration;
@property (strong) id<MTLTexture> depthTexture;
@property (strong) CAVDisplayLink *displayLink;
@end

@implementation MetalView

/// Returns a Metal-compatible layer.
+ (Class)layerClass
{
  return [CAMetalLayer class];
}

/// Since our view is layer-backed and updates itself by modifying its layer,
/// we override this property and change the return value to YES.
- (BOOL) wantsUpdateLayer { return YES; }

/// If the wantsLayer property is set to YES, this method will be invoked to return a layer instance.
-(CALayer*) makeBackingLayer {
   CALayer* layer = [self.class.layerClass layer];
   CGSize viewScale = [self convertSizeToBacking: CGSizeMake(1.0, 1.0)];
   layer.contentsScale = MIN(viewScale.width, viewScale.height);
   return layer;
}


- (instancetype)initWithCoder:(NSCoder *)aDecoder
{
  if ((self = [super initWithCoder:aDecoder]))
  {
    [self setWantsLayer:true];
    [self commonInit];
    self.metalLayer.device = MTLCreateSystemDefaultDevice();
    [self makeDisplayLink];
  }
  
  return self;
}

#pragma mark - items used by initWithCoder
/// Convenience routine to return the layer as a CAMetalLayer
- (CAMetalLayer *)metalLayer
{
  return (CAMetalLayer *)self.layer;
}

- (void)commonInit
{
  _preferredFramesPerSecond = 60;
  _clearColor = MTLClearColorMake(1, 1, 1, 1);
  
  self.metalLayer.pixelFormat = MTLPixelFormatBGRA8Unorm;
}

- (void)setFrame:(CGRect)frame
{
  [super setFrame:frame];
  NSLog(@"In setFrame");
  
  CGSize viewScale = [self convertSizeToBacking:CGSizeMake(1.0, 1.0)];
  CGFloat scale = MIN(viewScale.width, viewScale.height);
  
  CGSize drawableSize = self.bounds.size;
  
  // Since drawable size is in pixels, we need to multiply by the scale to move from points to pixels
  drawableSize.width *= scale;
  drawableSize.height *= scale;
  
  self.metalLayer.drawableSize = drawableSize;
  
  [self makeDepthTexture];
}

- (void)setColorPixelFormat:(MTLPixelFormat)colorPixelFormat
{
  self.metalLayer.pixelFormat = colorPixelFormat;
}

- (MTLPixelFormat)colorPixelFormat
{
  return self.metalLayer.pixelFormat;
}

- (void)makeDisplayLink // (didMoveToWindow in iOS)
{
  //    const NSTimeInterval idealFrameDuration = (1.0 / 60);
  //    const NSTimeInterval targetFrameDuration = (1.0 / self.preferredFramesPerSecond);
  //    const NSInteger frameInterval = round(targetFrameDuration / idealFrameDuration);

//  if (self.window)
//  {
    [self.displayLink invalidate];
    self.displayLink = [CAVDisplayLink displayLinkWithTarget:self
                                                    selector:@selector(displayLinkDidFire:)
                                            didFailWithError:nil];
    self.displayLink.preferredFramesPerSecond = self.preferredFramesPerSecond;
    [self.displayLink addToRunLoop:[NSRunLoop mainRunLoop] forMode:NSRunLoopCommonModes];
//  }
//  else
//  {
//    [self.displayLink invalidate];
//    self.displayLink = nil;
//  }
}

- (void)displayLinkDidFire:(CAVDisplayLink *)displayLink
{
  self.currentDrawable = [self.metalLayer nextDrawable];
  self.frameDuration = self.displayLink.duration;
  
  if ([self.delegate respondsToSelector:@selector(drawInView:)])
  {
    [self.delegate drawInView:self];
  }
}

- (void)makeDepthTexture
{
  CGSize drawableSize = self.metalLayer.drawableSize;
  
  if ([self.depthTexture width] != drawableSize.width ||
      [self.depthTexture height] != drawableSize.height)
  {
    MTLTextureDescriptor *desc = [MTLTextureDescriptor texture2DDescriptorWithPixelFormat:MTLPixelFormatDepth32Float
                                                                                    width:drawableSize.width
                                                                                   height:drawableSize.height
                                                                                mipmapped:NO];
    desc.usage = MTLTextureUsageRenderTarget;
    desc.storageMode = MTLStorageModePrivate;
    
    self.depthTexture = [self.metalLayer.device newTextureWithDescriptor:desc];
  }
}

- (MTLRenderPassDescriptor *)currentRenderPassDescriptor
{
  MTLRenderPassDescriptor *passDescriptor = [MTLRenderPassDescriptor renderPassDescriptor];
  
  passDescriptor.colorAttachments[0].texture = [self.currentDrawable texture];
  passDescriptor.colorAttachments[0].clearColor = self.clearColor;
  passDescriptor.colorAttachments[0].storeAction = MTLStoreActionStore;
  passDescriptor.colorAttachments[0].loadAction = MTLLoadActionClear;
  
  passDescriptor.depthAttachment.texture = self.depthTexture;
  passDescriptor.depthAttachment.clearDepth = 1.0;
  passDescriptor.depthAttachment.loadAction = MTLLoadActionClear;
  passDescriptor.depthAttachment.storeAction = MTLStoreActionDontCare;
  
  return passDescriptor;
}

@end
