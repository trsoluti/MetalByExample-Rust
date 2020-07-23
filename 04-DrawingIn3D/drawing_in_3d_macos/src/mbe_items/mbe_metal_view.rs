//
//  mbe_metal_view.rs
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

use state::Storage;
use std::sync::{RwLock, LockResult, RwLockReadGuard, RwLockWriteGuard};
use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use objc::{Encode, Encoding};
use objc::declare::ClassDecl;
use cocoa::base::{id, nil};
use objc::runtime::{Object, Sel, Class, object_getClass, NO, BOOL, YES};
use core_animation::{CoreAnimMetalLayer, MTLPixelFormat, CoreAnimMetalDrawable, MTLPixelFormatBGRA8Unorm, CoreAnimDisplayLink};
use crate::mbe_items::RustMBERenderer;
use cocoa::foundation::{NSInteger, NSTimeInterval, NSUInteger, NSRect, NSSize};
use metal_kit::{MetalClearColor, MetalRenderPassDescriptor, MetalDevice, MetalTexture, MetalTextureDescriptor, MTLPixelFormatDepth32Float, MTLTextureUsageRenderTarget, MTLStorageModePrivate, MTLStoreActionStore, MTLLoadActionClear, MTLStoreActionDontCare};
use crate::{debug_log};
use cocoa::foundation::NSAutoreleasePool;
use core_graphics::geometry::CGSize;

// THIS PART IS NECESSARY BECAUSE CGRECT DOESN'T IMPLEMENT ENCODING
// AND DOESN'T IMPLEMENT A LOT OF STUFF FOR MAC OS
// #[derive(Clone, Copy, Default, PartialEq)]
// struct EncodableCGPoint {
//     point: CGPoint,
// }
// unsafe impl Encode for EncodableCGPoint {
//     fn encode() -> Encoding {
//         let encoding = format!("{{CGPoint={}{}}}",
//                                CGFloat::encode().as_str(),
//                                CGFloat::encode().as_str());
//         unsafe { objc::Encoding::from_str(&encoding) }
//     }
// }
// impl Debug for EncodableCGPoint {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let point:&CGPoint = self as _;
//     }
// }
// struct EncodableCGSize {
//     size: CGSize,
// }
// unsafe impl Encode for EncodableCGSize {
//     fn encode() -> Encoding {
//         let encoding = format!("{{CGSize={}{}}}",
//                                CGFloat::encode().as_str(),
//                                CGFloat::encode().as_str());
//         unsafe { objc::Encoding::from_str(&encoding) }
//     }
// }
// #[derive(Clone, Copy, Debug, Default)]
// struct EncodableCGRect {
//     rect: CGRect,
// }
// unsafe impl Encode for EncodableCGRect {
//     fn encode() -> Encoding {
//         let encoding = format!("{{CGRect={}{}}}",
//                                EncodableCGPoint::encode().as_str(),
//                                EncodableCGSize::encode().as_str());
//         unsafe { objc::Encoding::from_str(&encoding) }
//     }
// }
// impl From<CGRect> for EncodableCGRect {
//     fn from(rect: CGRect) -> Self { EncodableCGRect { rect } }
// }

// /*
// // MetalView.h
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
// @import UIKit;
// @import Metal;
// @import QuartzCore.CAMetalLayer;
//
// @protocol MetalViewDelegate;
//
// @interface MetalView : UIView
//
/// An implementation of an UIView for Metal, in Rust
#[derive(Default)]
pub struct RustMetalView {
    // /// The delegate of this view, responsible for drawing
    // @property (nonatomic, weak) id<MetalViewDelegate> delegate;
    delegate: RustMBERenderer,
    //
    // /// The Metal layer that backs this view
    // @property (nonatomic, readonly) CAMetalLayer *metalLayer;
    metal_layer: CoreAnimMetalLayer,
    //
    // /// The target frame rate (in Hz). For best results, this should
    // /// be a number that evenly divides 60 (e.g., 60, 30, 15).
    // @property (nonatomic) NSInteger preferredFramesPerSecond;
    preferred_frames_per_second: NSInteger,
    //
    // /// The desired pixel format of the color attachment
    // @property (nonatomic) MTLPixelFormat colorPixelFormat;
    color_pixel_format: MTLPixelFormat,
    //
    // /// The color to which the color attachment should be cleared at the start of
    // /// a rendering pass
    // @property (nonatomic, assign) MTLClearColor clearColor;
    clear_color: MetalClearColor,
    //
    // /// The duration (in seconds) of the previous frame. This is valid only in the context
    // /// of a callback to the delegate's -drawInView: method.
    // @property (nonatomic, readonly) NSTimeInterval frameDuration;
    frame_duration: NSTimeInterval,
    //
    // /// The view's layer's current drawable. This is valid only in the context
    // /// of a callback to the delegate's -drawInView: method.
    // @property (nonatomic, readonly) id<CAMetalDrawable> currentDrawable;
    current_drawable: CoreAnimMetalDrawable,
    //
    // /// A render pass descriptor configured to use the current drawable's texture
    // /// as its primary color attachment and an internal depth texture of the same
    // /// size as its depth attachment's texture
    // @property (nonatomic, readonly) MTLRenderPassDescriptor *currentRenderPassDescriptor;
    // TOD: IS A COMPUTED PROPERTY
    // current_render_pass_descriptor: MetalRenderPassDescriptor,
    //
    // @end
    //
    // @interface MetalView ()
    // @property (strong) id<CAMetalDrawable> currentDrawable;
    // @property (assign) NSTimeInterval frameDuration;
    // @property (strong) id<MTLTexture> depthTexture;
    depth_texture: MetalTexture,
    // @property (strong) CADisplayLink *displayLink;
    display_link: CoreAnimDisplayLink,
    // @end
}
//
// @protocol MetalViewDelegate <NSObject>
/// Any rendering delegate must conform to this protocol.
pub trait RustMetalViewDelegate: Sized {
    // /// This method is called once per frame. Within the method, you may access
    // /// any of the properties of the view, and request the current render pass
    // /// descriptor to get a descriptor configured with renderable color and depth
    // /// textures.
    // - (void)drawInView:(MetalView *)view;
    /// This method is called once per frame. Within the method, you may access
    /// any of the properties of the view, and request the current render pass
    /// descriptor to get a descriptor configured with renderable color and depth
    /// textures.
    fn draw_in_view(&self, view: &RustMetalView);
    /// Updates the time and rotation based on the interval that has passed.
    fn update_time_and_rotation(&mut self, duration: NSTimeInterval);
}
// @end
unsafe impl Send for RustMetalView {}
unsafe impl Sync for RustMetalView {}
type RustMetalViewState = Storage<RwLock<RustMetalView>>;
#[derive(Copy, Clone)]
struct MetalViewPtr { metal_view_ptr: *mut RustMetalViewState, }
unsafe impl Sync for MetalViewPtr {}
impl MetalViewPtr {
    fn init(&self) {
        let storage = Box::into_raw(Box::new(RustMetalViewState::new()));
        let view_controller_ptr_ptr = &self.metal_view_ptr as *const _ as *mut *mut RustMetalViewState;
        unsafe { view_controller_ptr_ptr.write(storage); }
        unsafe { (*storage).set(RwLock::new(RustMetalView::default())); }
    }
    fn read(&self) -> LockResult<RwLockReadGuard<'_, RustMetalView>> {
        unsafe { (*self.metal_view_ptr).get().read() }
    }
    fn write(&mut self) -> LockResult<RwLockWriteGuard<'_, RustMetalView>> {
        unsafe { (*self.metal_view_ptr).get().write() }
    }
}
unsafe impl Encode for MetalViewPtr {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("^v") } // "^v" represents *mut c_void
    }
}

impl RustMetalView {
    /// Registers the MetalView class and its methods with the Objective C runtime.
    pub fn register() {
        // @interface MBEMetalView : UIView
        let base_class = class!(NSView);
        let mut class_decl = ClassDecl::new("MetalView", base_class).unwrap();

        unsafe {
            class_decl.add_ivar::<MetalViewPtr>("rustMetalView");
            //
            // /// The Metal layer that backs this view
            // @property (nonatomic, readonly) CAMetalLayer *metalLayer;
            class_decl.add_method(
                sel!(metalLayer),
                get_metal_layer as extern "C" fn(&Object, Sel) -> id,
            );
            //
            // /// The target frame rate (in Hz). For best results, this should
            // /// be a number that evenly divides 60 (e.g., 60, 30, 15).
            // @property (nonatomic) NSInteger preferredFramesPerSecond;
            class_decl.add_method(
                sel!(preferredFramesPerSecond),
                    get_preferred_frames_per_second as extern "C" fn(&Object, Sel) -> NSInteger,
            );
            class_decl.add_method(
                sel!(setPreferredFramesPerSecond:),
                set_preferred_frames_per_second as extern "C" fn(&mut Object, Sel, NSInteger),
            );
            //
            // /// The desired pixel format of the color attachment
            // @property (nonatomic) MTLPixelFormat colorPixelFormat;
            class_decl.add_method(
                sel!(colorPixelFormat),
                get_color_pixel_format as extern "C" fn(&Object, Sel) -> MTLPixelFormat,
            );
            class_decl.add_method(
                sel!(setColorPixelFormat:),
                set_color_pixel_format as extern "C" fn(&mut Object, Sel, MTLPixelFormat)
            );
            //
            // /// The color to which the color attachment should be cleared at the start of
            // /// a rendering pass
            // @property (nonatomic, assign) MTLClearColor clearColor;
            class_decl.add_method(
                sel!(clearColor),
                get_clear_color as extern "C" fn(&Object, Sel) -> MetalClearColor,
            );
            class_decl.add_method(
                sel!(setClearColor:),
                set_clear_color as extern "C" fn(&mut Object, Sel, MetalClearColor),
            );
            //
            // /// The duration (in seconds) of the previous frame. This is valid only in the context
            // /// of a callback to the delegate's -drawInView: method.
            // @property (nonatomic, readonly) NSTimeInterval frameDuration;
            class_decl.add_method(
                sel!(frameDuration),
                get_frame_duration as extern "C" fn(&Object, Sel) -> NSTimeInterval,
            );
            //
            // /// The view's layer's current drawable. This is valid only in the context
            // /// of a callback to the delegate's -drawInView: method.
            // @property (nonatomic, readonly) id<CAMetalDrawable> currentDrawable;
            class_decl.add_method(
                sel!(currentDrawable),
                get_current_drawable as extern "C" fn(&Object, Sel) -> id,
            );
            //
            // /// A render pass descriptor configured to use the current drawable's texture
            // /// as its primary color attachment and an internal depth texture of the same
            // /// size as its depth attachment's texture
            // @property (nonatomic, readonly) MTLRenderPassDescriptor *currentRenderPassDescriptor;
            class_decl.add_method(
                sel!(currentRenderPassDescriptor),
                get_current_render_pass_descriptor as extern "C" fn(&Object, Sel) -> id,
            );

            // Now the methods that are overrides or local
            class_decl.add_class_method(
                sel!(layerClass),
                get_layer_class as extern "C" fn(&Class, Sel) -> id,
            );
            class_decl.add_method(
                sel!(wantsUpdateLayer),
                wants_update_layer as extern "C" fn (&Object, Sel) -> BOOL,
            );
            class_decl.add_method(
                sel!(makeBackingLayer),
                make_backing_layer as extern "C" fn (&Object, Sel) -> id,
            );
            class_decl.add_method(
                sel!(initWithCoder:),
                init_with_coder as extern "C" fn(&mut Object, Sel, id) -> id,
            );
            class_decl.add_method(
                sel!(setFrame:),
                set_frame as extern "C" fn(&mut Object, Sel, NSRect),
            );
            // TOD: not in mac os
            // class_decl.add_method(
            //     sel!(didMoveToWindow),
            //     did_move_to_window as extern "C" fn(&mut Object, Sel),
            // );
            class_decl.add_method(
                sel!(displayLinkDidFire:),
                display_link_did_fire as extern "C" fn(&mut Object, Sel, id),
            );
            class_decl.register();
        }
    }
    // - (void)makeDepthTexture
    // {
    fn make_depth_texture(&mut self) {
        debug_log("In make_depth_texture()");
        // CGSize drawableSize = self.metalLayer.drawableSize;
        let drawable_size = self.metal_layer.get_drawable_size();
        //
        // if ([self.depthTexture width] != drawableSize.width ||
        //     [self.depthTexture height] != drawableSize.height)
        // {
        if self.depth_texture.is_null()
            || (self.depth_texture.get_width() != drawable_size.width as NSUInteger)
            || (self.depth_texture.get_height() != drawable_size.height as NSUInteger)
        {
            // MTLTextureDescriptor *desc = [MTLTextureDescriptor texture2DDescriptorWithPixelFormat:MTLPixelFormatDepth32Float
            //                                                                                 width:drawableSize.width
            //                                                                                height:drawableSize.height
            //                                                                             mipmapped:NO];
            let mut descriptor = MetalTextureDescriptor::texture_2d_descriptor_with_pixel_format_and_width_and_height_and_mipmapped(
                MTLPixelFormatDepth32Float,
                drawable_size.width as _,
                drawable_size.height as _,
                NO
            );
            // desc.usage = MTLTextureUsageRenderTarget;
            // desc.storageMode = MTLStorageModePrivate;
            descriptor.set_usage(MTLTextureUsageRenderTarget);
            descriptor.set_storage_mode(MTLStorageModePrivate);
            //
            // self.depthTexture = [self.metalLayer.device newTextureWithDescriptor:desc];
            let device = MetalDevice::from(self.metal_layer.get_device());
            let texture = device.new_texture_with_descriptor(descriptor);
            self.depth_texture = texture;
            let debug_message = format!("view depth texture set to {:?}", &self.depth_texture.to_objc());
            debug_log(debug_message.as_str());
        }
        // }
    }
    // }
    /// Gets the metal layer.
    #[inline]
    pub fn get_metal_layer(&self) -> &CoreAnimMetalLayer {
        &self.metal_layer
    }
    /// Gets a pointer to the current drawable.
    #[inline]
    pub fn get_current_drawable(&self) -> &CoreAnimMetalDrawable { &self.current_drawable }
    /// Gets a pointer to the depth texture.
    #[inline]
    pub fn get_depth_texture(&self) -> &MetalTexture { &self.depth_texture }
    /// Gets the frame duration.
    #[inline]
    pub fn get_frame_duration(&self) -> NSTimeInterval { self.frame_duration }
    // - (MTLRenderPassDescriptor *)currentRenderPassDescriptor
    // {
    fn current_render_pass_descriptor(&self) -> id {
        // MTLRenderPassDescriptor *passDescriptor = [MTLRenderPassDescriptor renderPassDescriptor];
        let mut pass_descriptor = MetalRenderPassDescriptor::render_pass_descriptor();
        //
        // passDescriptor.colorAttachments[0].texture = [self.currentDrawable texture];
        // passDescriptor.colorAttachments[0].clearColor = self.clearColor;
        // passDescriptor.colorAttachments[0].storeAction = MTLStoreActionStore;
        // passDescriptor.colorAttachments[0].loadAction = MTLLoadActionClear;
        pass_descriptor.set_color_attachments_texture(0, self.current_drawable.get_texture());
        pass_descriptor.set_color_attachments_clear_color(0, self.clear_color);
        pass_descriptor.set_color_attachments_store_action(0, MTLStoreActionStore);
        pass_descriptor.set_color_attachments_load_action(0, MTLLoadActionClear);
        //
        // passDescriptor.depthAttachment.texture = self.depthTexture;
        // passDescriptor.depthAttachment.clearDepth = 1.0;
        // passDescriptor.depthAttachment.loadAction = MTLLoadActionClear;
        // passDescriptor.depthAttachment.storeAction = MTLStoreActionDontCare;
        let mut depth_attachment = pass_descriptor.get_depth_attachment();
        depth_attachment.set_texture(&self.depth_texture);
        depth_attachment.set_clear_depth(1.);
        depth_attachment.set_load_action(MTLLoadActionClear);
        depth_attachment.set_store_action(MTLStoreActionDontCare);
        //
        // return passDescriptor;
        pass_descriptor.to_objc()
    }

// }
}

//
// ================================================================================================
// //  MetalView.m
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
// #import "MetalView.h"
//
// @interface MetalView ()
// @property (strong) id<CAMetalDrawable> currentDrawable;
// @property (assign) NSTimeInterval frameDuration;
// @property (strong) id<MTLTexture> depthTexture;
// @property (strong) CADisplayLink *displayLink;
// @end
//
// @implementation MetalView
// (Synthesized methods):
//
// /// The target frame rate (in Hz). For best results, this should
// /// be a number that evenly divides 60 (e.g., 60, 30, 15).
// @property (nonatomic) NSInteger preferredFramesPerSecond;
extern "C" fn get_preferred_frames_per_second(_self: &Object, _sel:Sel) -> NSInteger {
    let rust_metal_view = get_rust_metal_view(_self).unwrap();
    rust_metal_view.preferred_frames_per_second
}
extern "C" fn set_preferred_frames_per_second(_self: &mut Object, _sel: Sel, preferred_frames_per_second: NSInteger) {
    let mut rust_metal_view = get_mut_rust_metal_view(_self).unwrap();
    rust_metal_view.preferred_frames_per_second = preferred_frames_per_second
}
//
// /// The color to which the color attachment should be cleared at the start of
// /// a rendering pass
// @property (nonatomic, assign) MTLClearColor clearColor;
extern "C" fn get_clear_color(_self: &Object, _sel: Sel) -> MetalClearColor {
    let rust_metal_view = get_rust_metal_view(_self).unwrap();
    rust_metal_view.clear_color
}
extern "C" fn set_clear_color(_self: &mut Object, _sel: Sel, clear_color: MetalClearColor) {
    let mut rust_metal_view = get_mut_rust_metal_view(_self).unwrap();
    rust_metal_view.clear_color = clear_color
}
//
// /// The duration (in seconds) of the previous frame. This is valid only in the context
// /// of a callback to the delegate's -drawInView: method.
// @property (nonatomic, readonly) NSTimeInterval frameDuration;
extern "C" fn get_frame_duration(_self: &Object, _sel: Sel) -> NSTimeInterval {
    let rust_metal_view = get_rust_metal_view(_self).unwrap();
    rust_metal_view.frame_duration
}
//
// /// The view's layer's current drawable. This is valid only in the context
// /// of a callback to the delegate's -drawInView: method.
// @property (nonatomic, readonly) id<CAMetalDrawable> currentDrawable;
extern "C" fn get_current_drawable(_self: &Object, _sel: Sel) -> id {
    let rust_metal_view = get_rust_metal_view(_self).unwrap();
    rust_metal_view.current_drawable.to_objc()
}
//
// + (Class)layerClass
// {
//     return [CAMetalLayer class];
// }
extern "C" fn get_layer_class(_self: &Class, _sel: Sel) -> id {
    class!(CAMetalLayer) as * const _ as _
}
//
// /// Since our view is layer-backed and updates itself by modifying its layer,
// /// we override this property and change the return value to YES.
// - (BOOL) wantsUpdateLayer { return YES; }
extern "C" fn wants_update_layer(_self: &Object, _sel: Sel) -> BOOL { YES }
//
// /** If the wantsLayer property is set to YES, this method will be invoked to return a layer instance. */
// -(CALayer*) makeBackingLayer {
extern "C" fn make_backing_layer(_self: &Object, _sel: Sel) -> id {
    debug_log("In make backing layer.");
    // CALayer* layer = [self.class.layerClass layer];
    let self_class = unsafe { object_getClass(_self).as_ref().unwrap() };
    let layer_class = get_layer_class( self_class, sel!(layerClass) );
    let layer:id = unsafe { msg_send![layer_class, layer] };
    // CGSize viewScale = [self convertSizeToBacking: CGSizeMake(1.0, 1.0)];
    let unit_box = NSSize { width: 1., height: 1. };
    let view_scale: NSSize = unsafe { msg_send![_self, convertSizeToBacking:unit_box] };
    // layer.contentsScale = MIN(viewScale.width, viewScale.height);
    let contents_scale = f64::min(view_scale.width, view_scale.height);
    let _:() = unsafe { msg_send![layer, setContentsScale:contents_scale] };
    // return layer;
    layer
}
// }
//
// - (CAMetalLayer *)metalLayer
// {
//     return (CAMetalLayer *)self.layer;
// }
extern "C" fn get_metal_layer(_self: &Object, _sel:Sel) -> id {
    let rust_metal_view = get_rust_metal_view(_self).unwrap();
    rust_metal_view.metal_layer.to_objc()
}
//
// - (instancetype)initWithCoder:(NSCoder *)aDecoder
// {
extern "C" fn init_with_coder(_self: &mut Object, _sel: Sel, _coder: id) -> id {
    // if ((self = [super initWithCoder:aDecoder]))
    // {
    let _self: id = unsafe {
        let class = object_getClass(_self);
        let superclass: &Class = msg_send![class, superclass];
        msg_send![super(_self, superclass), initWithCoder: _coder]
    };
    debug_log("In metal view init with coder");
    if _self != nil {
        let mut rust_metal_view = get_mut_rust_metal_view(unsafe {_self.as_mut().unwrap()}).unwrap();
        // [self commonInit]; // TOD: brought lines here
        // _preferredFramesPerSecond = 60;
        rust_metal_view.preferred_frames_per_second = 60;
        // _clearColor = MTLClearColorMake(1, 1, 1, 1);
        rust_metal_view.clear_color = MetalClearColor::make(1., 1., 1., 1.);
        //
        // Layer is a field in the superclass, so need to set it first
        let _:() = unsafe { msg_send![_self, setWantsLayer:YES] };
        let layer:id = unsafe { msg_send![_self, layer] };
        let mut layer = CoreAnimMetalLayer::from(layer);
        // self.metalLayer.pixelFormat = MTLPixelFormatBGRA8Unorm;
        layer.set_pixel_format(MTLPixelFormatBGRA8Unorm);
        // self.metalLayer.device = MTLCreateSystemDefaultDevice();
        let system_default_device = MetalDevice::create_system_default_device();
        layer.set_device(system_default_device.to_objc());
        rust_metal_view.metal_layer = layer;
        rust_metal_view.delegate = RustMBERenderer::new().expect("Failed to make renderer");
    }
    if _self != nil {
        let _self = unsafe {_self.as_mut()}.unwrap();
        // TOD: following code was in did_move_to_window, which is not called in mac os:
        {
            let mut rust_metal_view = get_mut_rust_metal_view(_self).unwrap();
            // [self.displayLink invalidate];
            rust_metal_view.display_link.invalidate();
        }
        // self.displayLink = [CADisplayLink displayLinkWithTarget:self selector:@selector(displayLinkDidFire:)];
        let display_link = {
            CoreAnimDisplayLink::display_link_with_target_and_selector(_self, sel!(displayLinkDidFire:))
                .expect("Unable to create display link")
        };
        // self.displayLink.preferredFramesPerSecond = self.preferredFramesPerSecond;
        display_link.set_preferred_frames_per_second(get_rust_metal_view(_self).unwrap().preferred_frames_per_second);
        // [self.displayLink addToRunLoop:[NSRunLoop mainRunLoop] forMode:NSRunLoopCommonModes];
        display_link.add_to_main_run_loop();

        let mut rust_metal_view = get_mut_rust_metal_view(_self).unwrap();
        rust_metal_view.display_link = display_link;
    }
    // }
    //
    // return self;
    _self
}
// }
//
// - (instancetype)initWithFrame:(CGRect)frame device:(id<MTLDevice>)device
// {
// TOD: INIT-WITH-FRAME NEVER CALLED SO WE DON'T IMPLEMENT IT
//     if ((self = [super initWithFrame:frame]))
//     {
//         [self commonInit];
//         self.metalLayer.device = device;
//     }
//
//     return self;
// }
//
// - (void)commonInit
// {
// TOD: COMMON-INIT MOVED INTO INIT-WITH-CODER
//     _preferredFramesPerSecond = 60;
//     _clearColor = MTLClearColorMake(1, 1, 1, 1);
//
//     self.metalLayer.pixelFormat = MTLPixelFormatBGRA8Unorm;
// }
//
// - (void)setFrame:(CGRect)frame
// {
extern "C" fn set_frame(_self: &mut Object, _sel: Sel, frame: NSRect) {
    let pool = unsafe { NSAutoreleasePool::new(nil) };
    // [super setFrame:frame];
    unsafe {
        let class = object_getClass(_self);
        let superclass: &Class = msg_send![class, superclass];
        msg_send![super(_self, superclass), setFrame:frame]
    }
    //
    //
    // // If we've moved to a window by the time our frame is being set, we can take its scale as our own
    // if (self.window)
    // {
    // let window:id = unsafe { msg_send![_self, window] };
    // let scale = if window != nil {
    //     //     scale = self.window.screen.scale;
    //     let screen:id = unsafe { msg_send![window, screen] };
    //     let scale: CGFloat = unsafe { msg_send![screen, scale] };
    //     scale
    // } else {
    //     // // During the first layout pass, we will not be in a view hierarchy, so we guess our scale
    //     // CGFloat scale = [UIScreen mainScreen].scale;
    //     let ui_screen_class = class!(UIScreen);
    //     let main_screen:id = unsafe { msg_send![ui_screen_class, mainScreen] };
    //     let scale:CGFloat = unsafe { msg_send![main_screen, scale] };
    //     scale
    // };
    // }
    // TOD: on the mac, we put the scaling factor in the layer
    // when we created it.
    let unit_box = CGSize::new(1.0, 1.0);
    let view_scale: CGSize = unsafe { msg_send![_self, convertSizeToBacking:unit_box] };
    let scale = f64::min(view_scale.width, view_scale.height);
    //
    // CGSize drawableSize = self.bounds.size;
    let bounds:NSRect = unsafe { msg_send![_self, bounds] };
    let drawable_size = bounds.size;
    //
    // // Since drawable size is in pixels, we need to multiply by the scale to move from points to pixels
    // drawableSize.width *= scale;
    // drawableSize.height *= scale;
    let drawable_size = CGSize { // has to be CGSize as rust doesn't know they're the same
        width: drawable_size.width * scale,
        height: drawable_size.height * scale,
    };
    //
    // self.metalLayer.drawableSize = drawableSize;
    let mut rust_metal_view = get_mut_rust_metal_view(_self).unwrap();
    rust_metal_view.metal_layer.set_drawable_size(drawable_size);
    //
    // [self makeDepthTexture];
    rust_metal_view.make_depth_texture();
    unsafe { pool.drain() }
}
// }
//
// - (void)setColorPixelFormat:(MTLPixelFormat)colorPixelFormat
// {
//     self.metalLayer.pixelFormat = colorPixelFormat;
// }
extern "C" fn set_color_pixel_format(_self: &mut Object, _sel: Sel, color_pixel_format: MTLPixelFormat) {
    let mut rust_metal_view = get_mut_rust_metal_view(_self).unwrap();
    rust_metal_view.color_pixel_format = color_pixel_format
}
//
// - (MTLPixelFormat)colorPixelFormat
// {
//     return self.metalLayer.pixelFormat;
// }
extern "C" fn get_color_pixel_format(_self: &Object, _sel: Sel) -> MTLPixelFormat {
    let rust_metal_view = get_rust_metal_view(_self).unwrap();
    rust_metal_view.color_pixel_format
}
//
// - (void)displayLinkDidFire:(CADisplayLink *)displayLink
// {
extern "C" fn display_link_did_fire(_self: &mut Object, _sel: Sel, _display_link: id) {
    //+ debug_log("In display link did fire");
    // self.currentDrawable = [self.metalLayer nextDrawable];
    // self.frameDuration = displayLink.duration;
    let mut rust_metal_view = get_mut_rust_metal_view(_self).unwrap();
    rust_metal_view.current_drawable = rust_metal_view.metal_layer.next_drawable();
    let frame_duration = rust_metal_view.display_link.get_duration();
    // Delegate has to be read-only except for very specific circumstances
    // (i.e. can't pass in any pointers from rust_metal_view)
    // so we have to break out the update parts of draw_in_view
    rust_metal_view.delegate.update_time_and_rotation(frame_duration);
    rust_metal_view.frame_duration = frame_duration;
    //
    // if ([self.delegate respondsToSelector:@selector(drawInView:)])
    // {
    //     [self.delegate drawInView:self];
    // }
    rust_metal_view.delegate.draw_in_view(&rust_metal_view);
}
// }
//
//
// - (MTLRenderPassDescriptor *)currentRenderPassDescriptor
// {
extern "C" fn get_current_render_pass_descriptor(_self: &Object, _sel: Sel) -> id {
    let rust_metal_view = get_rust_metal_view(_self).unwrap();
    rust_metal_view.current_render_pass_descriptor()
}

// }
//
// @end
//

// ============================================================================
// private functions for operations
fn get_rust_metal_view(_self: &Object) -> LockResult<RwLockReadGuard<'_, RustMetalView>> {
    let rust_metal_view_ptr = unsafe { _self.get_ivar::<MetalViewPtr>("rustMetalView") };
    rust_metal_view_ptr.read()
}
fn get_mut_rust_metal_view(_self: &mut Object) -> LockResult<RwLockWriteGuard<'_, RustMetalView>> {
    let rust_metal_view_ptr = unsafe { _self.get_mut_ivar::<MetalViewPtr>("rustMetalView") };
    if rust_metal_view_ptr.metal_view_ptr.is_null() {
        rust_metal_view_ptr.init() // will point a pointer in.
    }
    rust_metal_view_ptr.write()
}
