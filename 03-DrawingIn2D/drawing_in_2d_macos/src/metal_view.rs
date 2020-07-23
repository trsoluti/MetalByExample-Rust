//
//  metal_view.rs
//
//  Created by TR Solutions on 2020-07-15.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Implements our application's MetalView in Rust

use crate::display_link::CoreAnimVideoDisplayLink;
use objc::{class, Encode, Encoding};
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, objc_retain, object_getClass, BOOL, YES};
use cocoa::base::{id, nil};
use state::Storage;
use std::sync::{RwLock, RwLockReadGuard, LockResult, RwLockWriteGuard};
use crate::{debug_log};
use matrix_kit::vector_float4;
use metal_kit::{MetalDevice, MTLResourceCPUCacheModeDefaultCache};
use metal_kit::MetalRenderPipelineDescriptor;
use cocoa::foundation::{NSString, NSAutoreleasePool, NSSize};
use metal_kit::MetalRenderPipelineState;
use metal_kit::MetalCommandQueue;
use core_animation::{CoreAnimMetalLayer, MTLPixelFormatBGRA8Unorm};
use metal_kit::MetalBuffer;
use metal_kit::{MetalRenderPassDescriptor, MTLStoreActionStore, MTLLoadActionClear};
use metal_kit::MetalClearColor;
use metal_kit::MTLPrimitiveTypeTriangle;
use std::pin::Pin;

extern {
    fn NSLogv(fmt: id, ...);
    #[allow(non_upper_case_globals)]
    static NSRunLoopCommonModes: id;
}

// //
// //  MetalView.h
// //  DrawingIn2DMacOS
// //
// //  Created by TR Solutions on 14/7/20.
// //  Copyright © 2020 TR Solutions.
// //
//
// #import <Cocoa/Cocoa.h>
//
// NS_ASSUME_NONNULL_BEGIN
//
// /// @interface MetalView
// /// @abstract A class to handle the interface to Metal while operating as an NSView.
// /// @discussion
// /// This class provides an interface similar to MetalKit's MTKView. It syncs with the display
// @interface MetalView: NSView
//
// @end
//
// NS_ASSUME_NONNULL_END
//
// ================================================================================================
// //
// //  MetalView.m
// //  DrawingIn2DMacOS
// //
// //  Created by TR Solutions on 14/7/20.
// //  Copyright © 2020 TR Solutions.
// //
//
// #import "MetalView.h"
// #import "CAVDisplayLink.h"
// @import QuartzCore;
// @import Metal;
// @import simd;
//
// typedef struct
// {
//   vector_float4 position;
//   vector_float4 color;
// } MBEVertex;
#[allow(dead_code)]
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
// @interface MetalView ()
// @property (nonatomic, strong) CAVDisplayLink *displayLink;
// @property (nonatomic, strong) id<MTLDevice> device;
// @property (nonatomic, strong) id<MTLRenderPipelineState> pipeline;
// @property (nonatomic, strong) id<MTLCommandQueue> commandQueue;
// @property (nonatomic, strong) id<MTLBuffer> vertexBuffer;
// @end
#[derive(Default)]
/// Rust implementation of our application's Metalview
pub struct MetalViewRust {
    #[allow(dead_code)]
    // delegate: Renderer,
    display_link: Option<Pin<Box<CoreAnimVideoDisplayLink>>>,
    layer: CoreAnimMetalLayer,
    device: MetalDevice,
    pipeline: MetalRenderPipelineState,
    command_queue: MetalCommandQueue,
    vertex_buffer: MetalBuffer,
}
unsafe impl Send for MetalViewRust {}
unsafe impl Sync for MetalViewRust {}
type MetalViewRustState = Storage<RwLock<MetalViewRust>>;
#[derive(Copy, Clone)]
struct MetalViewPtr { view_controller_ptr: *mut MetalViewRustState, }
unsafe impl Sync for MetalViewPtr {}
impl MetalViewPtr {
    fn init(&self) {
        let storage = Box::into_raw(Box::new(MetalViewRustState::new()));
        let view_controller_ptr_ptr = &self.view_controller_ptr as *const _ as *mut *mut MetalViewRustState;
        unsafe { view_controller_ptr_ptr.write(storage); }
        unsafe { (*storage).set(RwLock::new(MetalViewRust::default())); }
    }
    fn read(&self) -> LockResult<RwLockReadGuard<'_, MetalViewRust>> {
        unsafe { (*self.view_controller_ptr).get().read() }
    }
    fn write(&mut self) -> LockResult<RwLockWriteGuard<'_, MetalViewRust>> {
        unsafe { (*self.view_controller_ptr).get().write() }
    }
}
unsafe impl Encode for MetalViewPtr {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("^v") } // "^v" represents *mut c_void
    }
}

impl MetalViewRust {
    /// Registers the MetalView class and its methods with the Objective C runtime.
    pub fn register() {
        // @interface MBEMetalView : NSView
        let base_class = class!(NSView);
        let mut class_decl = ClassDecl::new("MetalView", base_class).unwrap();

        unsafe {
            class_decl.add_ivar::<MetalViewPtr>("rustMetalView");
            //
            // @property (readonly) CAMetalLayer *metalLayer;
            // class_decl.add_method(
            //     sel!(metalLayer),
            //     get_metal_layer as extern "C" fn(&Object, Sel) -> id,
            // );
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
                sel!(dealloc),
                dealloc as extern "C" fn(&mut Object, Sel),
            );
            class_decl.add_method(
                sel!(redraw),
                redraw as extern "C" fn(&Object, Sel),
            );
            class_decl.add_method(
                sel!(displayLinkDidFire:),
                display_link_did_fire as extern "C" fn(&Object, Sel, id),
            );
            //
            // @end
        }
        class_decl.register();
    }
    // - (void)makeDevice
    // {
    fn make_device(&mut self) {
        // device = MTLCreateSystemDefaultDevice();
        self.device  = MetalDevice::create_system_default_device();
        // self.metalLayer.device = device;
        self.layer.set_device(self.device.to_objc());
        // self.metalLayer.pixelFormat = MTLPixelFormatBGRA8Unorm;
        self.layer.set_pixel_format(MTLPixelFormatBGRA8Unorm);
    }
    // }
    //do
    // - (void)makePipeline
    // {
    fn make_pipeline(&mut self) {
        let pool = unsafe {NSAutoreleasePool::new(nil) };
        // id<MTLLibrary> library = [device newDefaultLibrary];
        let library = self.device.new_default_library();
        //
        // id<MTLFunction> vertexFunc = [library newFunctionWithName:@"vertex_main"];
        // id<MTLFunction> fragmentFunc = [library newFunctionWithName:@"fragment_main"];
        let vertex_function = library.new_function_with_name("vertex_main");
        let fragment_function = library.new_function_with_name("fragment_main");
        //
        // MTLRenderPipelineDescriptor *pipelineDescriptor = [MTLRenderPipelineDescriptor new];
        let mut pipeline_descriptor = MetalRenderPipelineDescriptor::new();
        // pipelineDescriptor.colorAttachments[0].pixelFormat = MTLPixelFormatBGRA8Unorm;
        pipeline_descriptor.set_color_attachment_pixel_format(0, MTLPixelFormatBGRA8Unorm);
        // pipelineDescriptor.vertexFunction = vertexFunc;
        // pipelineDescriptor.fragmentFunction = fragmentFunc;
        pipeline_descriptor.set_vertex_function(vertex_function);
        pipeline_descriptor.set_fragment_function(fragment_function);
        //
        // NSError *error = nil;
        // _pipeline = [device newRenderPipelineStateWithDescriptor:pipelineDescriptor
        //                                                    error:&error];
        //
        // if (!_pipeline)
        // {
        //     NSLog(@"Error occurred when creating render pipeline state: %@", error);
        // }
        let pipeline_result = self.device.new_render_pipeline_state_with_descriptor(pipeline_descriptor);
        if let Err(error) = pipeline_result {
            let format = unsafe { NSString::alloc(nil).init_str("Error occurred when creating render pipeline state: %@") };
            let format = unsafe { objc_retain(format) };
            unsafe { NSLogv(format, error); }
        } else {
            self.pipeline = pipeline_result.unwrap();
            //
            // _commandQueue = [device newCommandQueue];
            self.command_queue = self.device.new_command_queue();
        }
        unsafe { pool.drain() };
    }
    // }
    //
    // - (void)makeBuffers
    // {
    fn make_buffers(&mut self) {
        // static const MBEVertex vertices[] =
        // {
        //     { .position = {  0.0,  0.5, 0, 1 }, .color = { 1, 0, 0, 1 } },
        //     { .position = { -0.5, -0.5, 0, 1 }, .color = { 0, 1, 0, 1 } },
        //     { .position = {  0.5, -0.5, 0, 1 }, .color = { 0, 0, 1, 1 } }
        // };
        let vertices = [
            //     { .position =   {  0.0,  0.5, 0,  1 }, .color = { 1,  0,  0,  1 } },
            MBEVertex::from([  0.0,  0.5, 0., 1.,             1., 0., 0., 1.]),
            //     { .position =   { -0.5, -0.5, 0,  1 }, .color = { 0,  1,  0,  1 } },
            MBEVertex::from([ -0.5, -0.5, 0., 1.,             0., 1., 0., 1.]),
            //     { .position =   {  0.5, -0.5, 0,  1 }, .color = { 0,  0,  1,  1 } }
            MBEVertex::from([  0.5, -0.5, 0., 1.,             0., 0., 1., 1.]),
        ];
        //
        // _vertexBuffer = [device newBufferWithBytes:vertices
        //                                     length:sizeof(vertices)
        //                                    options:MTLResourceOptionCPUCacheModeDefault];
        self.vertex_buffer = self.device.new_buffer_with_bytes_and_options(&vertices, MTLResourceCPUCacheModeDefaultCache)
    }
    // }
    // /// Makes a display link timer that will fire when the main display syncs
    // -(void)makeDisplayLink
    // {
    fn make_display_link(&mut self, _self: &mut Object) {
        // self.displayLink = [CAVDisplayLink displayLinkWithTarget:self selector:@selector(displayLinkDidFire:) didFailWithError:nil];
        let display_link = CoreAnimVideoDisplayLink::display_link_with_target_and_selector(_self, sel!(displayLinkDidFire:)).unwrap();
        // [self.displayLink addToRunLoop:[NSRunLoop mainRunLoop] forMode:NSRunLoopCommonModes];
        let main_loop:id = unsafe { msg_send![class!(NSRunLoop), mainRunLoop] };
        display_link.add_to_run_loop_for_mode(main_loop, unsafe { NSRunLoopCommonModes });
        self.display_link = Some(display_link);
    }
    // }
    //
    //
    // - (void)redraw
    // {
    fn redraw(&self) {
        let pool = unsafe { NSAutoreleasePool::new(nil) };
        //+ debug_log("In RustMetalView::redraw().");
        // id<CAMetalDrawable> drawable = [self.metalLayer nextDrawable];
        let drawable = self.layer.next_drawable();
        //
        // if (drawable)
        // {
        if drawable.is_not_null() {
            // id<MTLTexture> framebufferTexture = drawable.texture;
            let frame_buffer_texture = drawable.get_texture();
            // MTLRenderPassDescriptor *passDescriptor = [MTLRenderPassDescriptor renderPassDescriptor];
            let mut pass_descriptor = MetalRenderPassDescriptor::render_pass_descriptor();
            // passDescriptor.colorAttachments[0].texture = framebufferTexture;
            // passDescriptor.colorAttachments[0].clearColor = MTLClearColorMake(0.85, 0.85, 0.85, 1);
            // passDescriptor.colorAttachments[0].storeAction = MTLStoreActionStore;
            // passDescriptor.colorAttachments[0].loadAction = MTLLoadActionClear;
            pass_descriptor.set_color_attachments_texture(0, frame_buffer_texture);
            let clear_color = MetalClearColor::make(0.85, 0.85, 0.85, 1.);
            pass_descriptor.set_color_attachments_clear_color(0, clear_color);
            pass_descriptor.set_color_attachments_store_action(0, MTLStoreActionStore);
            pass_descriptor.set_color_attachments_load_action(0, MTLLoadActionClear);
            //
            // id<MTLCommandBuffer> commandBuffer = [self.commandQueue commandBuffer];
            let mut command_buffer = self.command_queue.command_buffer();
            //
            // id<MTLRenderCommandEncoder> commandEncoder = [commandBuffer renderCommandEncoderWithDescriptor:passDescriptor];
            let mut command_encoder = command_buffer.render_command_encoder_with_descriptor(&pass_descriptor);
            // [commandEncoder setRenderPipelineState:self.pipeline];
            // [commandEncoder setVertexBuffer:self.vertexBuffer offset:0 atIndex:0];
            // [commandEncoder drawPrimitives:MTLPrimitiveTypeTriangle vertexStart:0 vertexCount:3];
            // [commandEncoder endEncoding];
            command_encoder.set_render_pipeline_state(&self.pipeline);
            command_encoder.set_vertex_buffer(&self.vertex_buffer, 0, 0);
            command_encoder.draw_primitives(MTLPrimitiveTypeTriangle, 0, 3);
            command_encoder.end_encoding();
            //
            // [commandBuffer presentDrawable:drawable];
            command_buffer.present_drawable(&drawable);
            // [commandBuffer commit];
            command_buffer.commit();
        }
        // }
        unsafe { pool.drain() }
    }
    // }
    //

}

//
// @implementation MetalView
//
// /// Returns a Metal-compatible layer.
// +(Class) layerClass { return [CAMetalLayer class]; }
extern "C" fn get_layer_class(_self: &Class, _sel: Sel) -> id {
    debug_log("In get layer class");
    class!(CAMetalLayer) as * const _ as _
}

//
// /// Since our view is layer-backed and updates itself by modifying its layer,
// /// we override this property and change the return value to YES.
// - (BOOL) wantsUpdateLayer { return YES; }
extern "C" fn wants_update_layer(_self: &Object, _sel: Sel) -> BOOL { YES }
//
// /// Makes a new instance of the class from the coder
// - (instancetype)initWithCoder:(NSCoder *)aDecoder
// {
extern "C" fn init_with_coder(_self: &mut Object, _sel: Sel, coder: id) -> id {
    // if ((self = [super initWithCoder:aDecoder]))
    let _self:id = unsafe {
        let class = object_getClass(_self);
        let superclass: &Class = msg_send![class, superclass];
        msg_send![super(_self, superclass), initWithCoder:coder]
    };
    debug_log("In metal view init with coder");
    // {
    if _self != nil {
        let mut rust_metal_view = unsafe {
            let p = _self.as_mut().unwrap();
            let rust_metal_view_ptr = p.get_mut_ivar::<MetalViewPtr>("rustMetalView");
            rust_metal_view_ptr.init();
            rust_metal_view_ptr.write().unwrap()
        };
        // self.wantsLayer = true;
        // [self makeDevice];
        // [self makeBuffers];
        // [self makePipeline];
        // [self makeDisplayLink];
        // Layer is a field in the parent level, so need to set it first
        let _:() = unsafe { msg_send![_self, setWantsLayer:YES] };
        let layer:id = unsafe { msg_send![_self, layer] };
        rust_metal_view.layer = CoreAnimMetalLayer::from(layer);
        rust_metal_view.make_device();
        rust_metal_view.make_buffers();
        rust_metal_view.make_pipeline();
        rust_metal_view.make_display_link(unsafe { _self.as_mut().unwrap() } );
    }
    // }
    //
    // return self;
    _self
}
// }
//
// /// Safely deallocates the class
// - (void)dealloc
// {
extern "C" fn dealloc(_self: &mut Object, _sel: Sel) {
    let mut rust_metal_view = get_mut_rust_metal_view(_self).unwrap();
    // [_displayLink invalidate];
    match &mut rust_metal_view.display_link {
        Some(display_link) => display_link.invalidate(),
        None => ()
    }
}
// }
//
//
//
// #pragma mark - items used by initWithCoder
// /// Convenience routine to return the layer as a CAMetalLayer
// - (CAMetalLayer *)metalLayer {
//   return (CAMetalLayer *)self.layer;
// }
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
// #pragma mark - Display Link Fire actions
//
// /// Called by the notification system when the display link fires
// - (void)displayLinkDidFire:(CAVDisplayLink *)displayLink
// {
extern "C" fn display_link_did_fire(_self: &Object, _sel: Sel, _display_link: id) {
    //   //+ NSLog(@"In display link did fire.");
    //   [self redraw];
    let rust_metal_view = get_rust_metal_view(_self).unwrap();
    rust_metal_view.redraw()
}
// }
//
// - (void)redraw
// {
extern "C" fn redraw(_self: &Object, _sel: Sel) {
    let rust_metal_view = get_rust_metal_view(_self).unwrap();
    rust_metal_view.redraw()
}
// }
//
//
// @end
//

// ============================================================================
// private functions for operations
fn get_rust_metal_view(_self: &Object) -> LockResult<RwLockReadGuard<'_, MetalViewRust>> {
    let rust_metal_view_ptr = unsafe { _self.get_ivar::<MetalViewPtr>("rustMetalView") };
    rust_metal_view_ptr.read()
}
fn get_mut_rust_metal_view(_self: &mut Object) -> LockResult<RwLockWriteGuard<'_, MetalViewRust>> {
    let rust_metal_view_ptr = unsafe { _self.get_mut_ivar::<MetalViewPtr>("rustMetalView") };
    rust_metal_view_ptr.write()
}
