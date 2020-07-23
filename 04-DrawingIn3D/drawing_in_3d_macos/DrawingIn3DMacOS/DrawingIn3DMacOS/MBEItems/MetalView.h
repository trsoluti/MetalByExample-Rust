// MetalView.h
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
@import Cocoa;
@import Metal;
@import QuartzCore.CAMetalLayer;

@protocol MetalViewDelegate;

@interface MetalView : NSView

/// The delegate of this view, responsible for drawing
@property (nonatomic, weak) id<MetalViewDelegate> delegate;

/// The Metal layer that backs this view
@property (nonatomic, readonly) CAMetalLayer *metalLayer;

/// The target frame rate (in Hz). For best results, this should
/// be a number that evenly divides 60 (e.g., 60, 30, 15).
@property (nonatomic) NSInteger preferredFramesPerSecond;

/// The desired pixel format of the color attachment
@property (nonatomic) MTLPixelFormat colorPixelFormat;

/// The color to which the color attachment should be cleared at the start of
/// a rendering pass
@property (nonatomic, assign) MTLClearColor clearColor;

/// The duration (in seconds) of the previous frame. This is valid only in the context
/// of a callback to the delegate's -drawInView: method.
@property (nonatomic, readonly) NSTimeInterval frameDuration;

/// The view's layer's current drawable. This is valid only in the context
/// of a callback to the delegate's -drawInView: method.
@property (nonatomic, readonly) id<CAMetalDrawable> currentDrawable;

/// A render pass descriptor configured to use the current drawable's texture
/// as its primary color attachment and an internal depth texture of the same
/// size as its depth attachment's texture
@property (nonatomic, readonly) MTLRenderPassDescriptor *currentRenderPassDescriptor;

@end

@protocol MetalViewDelegate <NSObject>
/// This method is called once per frame. Within the method, you may access
/// any of the properties of the view, and request the current render pass
/// descriptor to get a descriptor configured with renderable color and depth
/// textures.
- (void)drawInView:(MetalView *)view;
@end
