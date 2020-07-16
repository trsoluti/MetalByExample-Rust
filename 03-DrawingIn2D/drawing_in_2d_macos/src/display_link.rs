//
//  display_link.rs
//
//  Created by TR Solutions on 2020-07-15.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//

//! A temporary Rust wrapper for CAVDisplayLink
//! Eventually it will be merged into the Rust object
//! CoreAnimDisplayLink

use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil, SEL, selector};
use objc::runtime::{objc_retain, objc_release, Object, Sel};
use std::fmt::{Display, Formatter};
use std::error::Error;
use cocoa::foundation::NSAutoreleasePool;
use std::os::raw::{c_void, c_ulong, c_int, c_ulonglong};
use std::ptr::null;
use crate::debug_log;
use cocoa::quartzcore::CVTimeStamp;
use std::pin::Pin;
use std::ops::Deref;

// Types from Core Video:
pub type CVDisplayLinkRef = *const c_void;
#[allow(non_camel_case_types)]
pub type dispatch_source_t = id;
#[allow(non_camel_case_types)]
pub type uintptr_t = *const c_void;
#[allow(non_camel_case_types)]
pub type dispatch_queue_t = id;
pub type CGDirectDisplayID = *const c_void;
pub type CVReturn = c_int;
pub type CVOptionFlags = c_ulonglong;
pub type CVDisplayLinkOutputCallback = *const extern "C" fn(display_link: CVDisplayLinkRef, now: *const CVTimeStamp, output_time: *const CVTimeStamp, flags_in: CVOptionFlags, flags_out: *mut CVOptionFlags, display_link_context: *const c_void) -> CVReturn;
#[allow(non_camel_case_types)]
pub type dispatch_object_t = id;

// CVReturn.h:    kCVReturnSuccess = 0
#[allow(non_upper_case_globals)]
static kCVReturnSuccess: c_int = 0;

#[link(name = "QuartzCore", kind = "framework")]
extern {
    #[allow(non_upper_case_globals)]
    static _dispatch_source_type_data_add: c_void;
    // dispatch_source_t dispatch_source_create(dispatch_source_type_t type, uintptr_t handle, unsigned long mask, dispatch_queue_t queue);
    fn dispatch_source_create(dispatch_type: &c_void, handle: uintptr_t, mask: c_ulong, queue: dispatch_queue_t) -> dispatch_source_t;
    // CGDirectDisplayID CGMainDisplayID(void);
    fn CGMainDisplayID() -> CGDirectDisplayID;
    // CVReturn CVDisplayLinkCreateWithCGDisplay(CGDirectDisplayID displayID, CVDisplayLinkRef  _Nullable *displayLinkOut);
    fn CVDisplayLinkCreateWithCGDisplay(display_id: CGDirectDisplayID, display_link_out: &mut CVDisplayLinkRef) -> CVReturn;
    // CV_EXPORT CVDisplayLinkRef CV_NULLABLE CVDisplayLinkRetain( CVDisplayLinkRef CV_NULLABLE displayLink ) AVAILABLE_MAC_OS_X_VERSION_10_4_AND_LATER;
    fn CVDisplayLinkRetain( display_link: CVDisplayLinkRef ) -> CVDisplayLinkRef;
    // CVReturn CVDisplayLinkSetOutputCallback(CVDisplayLinkRef displayLink, CVDisplayLinkOutputCallback callback, void *userInfo);
    fn CVDisplayLinkSetOutputCallback(display_link: CVDisplayLinkRef, callback: CVDisplayLinkOutputCallback, user_info: *const c_void) -> CVReturn;
    // void dispatch_resume(dispatch_object_t object);
    fn dispatch_resume(object: dispatch_object_t);
    // CVReturn CVDisplayLinkStart(CVDisplayLinkRef displayLink);
    fn CVDisplayLinkStart(display_link: CVDisplayLinkRef) -> CVReturn;
    // CVReturn CVDisplayLinkStop(CVDisplayLinkRef displayLink);
    fn CVDisplayLinkStop(display_link: CVDisplayLinkRef) -> CVReturn;
    // oid dispatch_suspend(dispatch_object_t object);
    fn dispatch_suspend(object: dispatch_object_t);
    fn dispatch_source_merge_data(source: dispatch_source_t, data: c_ulong);
}
#[link(name="System", kind="framework")]
extern {
    // oid dispatch_cancel(dispatch_object_t object);
    fn dispatch_source_cancel(object: dispatch_object_t);
}
// typedef void (*dispatch_function_with_user_data_t)(dispatch_source_t _Nonnull , void *_Nullable);
#[allow(non_camel_case_types)]
pub trait dispatch_function_with_user_data_trait: Fn(dispatch_source_t, *const c_void){}
#[allow(non_camel_case_types)]
pub type dispatch_function_with_user_data_t = extern "C" fn(source: dispatch_source_t, user_data: *const c_void);
#[link(name = "GlueLib", kind = "dylib")]
extern {
    // void dispatch_source_set_event_handler_f_with_user_data
    // (dispatch_source_t _Nonnull,
    //  dispatch_function_with_user_data_t _Nullable,
    //   void * _Nullable
    //   );
    fn dispatch_source_set_event_handler_f_with_user_data(
        source: dispatch_source_t,
        handler: dispatch_function_with_user_data_t,
        user_data: *const c_void,
    );
    // dispatch_queue_main_t _Nonnull dispatch_get_main_queue_not_inline(void);
    fn dispatch_get_main_queue_not_inline() -> dispatch_queue_t;
}


// from usr/include/dispatch/source.h:
// #define DISPATCH_SOURCE_TYPE_DATA_ADD (&_dispatch_source_type_data_add)
static DISPATCH_SOURCE_TYPE_DATA_ADD: &c_void = unsafe { &_dispatch_source_type_data_add };

// //
// //  CAVDisplayLink.h
// //  DrawingIn2DMacOS
// //
// //  Created by TR Solutions on 14/7/20.
// //  Copyright © 2020 TR Solutions.
// //
//
// #import <Foundation/Foundation.h>
//
// NS_ASSUME_NONNULL_BEGIN
//
// /// @abstract Error codes returned by the the display link displayLinkWithTarget: selector:didFailWithError: method
// /// @constant FAILED_TO_CREATE_DISPLAY_LINK Attempt to connect to the timer failed
// /// @constant FAILED_TO_CONNECT_DISPLAY Attempt to connect to the main display failed
// /// @discussion only valid if error is not nil on return from displayLinkWithTarget: selector:didFailWithError:
// enum DisplayLinkError {
//   FAILED_TO_CREATE_DISPLAY_LINK = -101,
//   FAILED_TO_LINK_OUTPUT_HANDLER = -102,
//   FAILED_TO_CONNECT_DISPLAY = -103
// };
/// The error returned if the system failed
/// to create a render pipeline state
#[derive(Debug)]
pub enum CoreAnimVideoError {
    /// The system returned the given Objective C error
    /// when attempting to create a render pipeline state
    CoreAnimVideoCreationError(id),
    /// Attempt to connect to the timer failed.
    FailedToCreateTimer(c_int),
    /// Attempt to link to the output handler failed.
    FailedToLinkOutputHandler(c_int),
    /// Attempt to connect to the main display failed.
    FailedToConnectToDisplay(c_int),
}
impl Display for CoreAnimVideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Core Anim Video Creation error")
    }
}
impl Error for CoreAnimVideoError {}
//
//
// @interface CAVDisplayLink : NSObject
//
// + (CAVDisplayLink *)displayLinkWithTarget:(id)target
//                                  selector:(SEL)sel
//                          didFailWithError: (NSError * _Nullable __autoreleasing *)error;
//
// - (void)addToRunLoop:(NSRunLoop *)runloop forMode:(NSRunLoopMode)mode;
//
// - (void)invalidate;
//
// @end
//
// NS_ASSUME_NONNULL_END
// ============================================================================================
// //
// //  CAMDisplayLink.m
// //  DrawingIn2DMacOS
// //
// //  Created by TR Solutions on 14/7/20.
// //  Copyright © 2020 TR Solutions. All rights reserved.
// //
//
// #import "CAVDisplayLink.h"
// @import CoreVideo;
// #import "GlueLib.h"
// @import ObjectiveC;
//
// @interface CAVDisplayLink ()
// @property CVDisplayLinkRef displayLinkRef;
// @property (nonatomic, strong) dispatch_source_t source;
// @property (nonatomic, strong) id target;
// @property SEL selector;
// @end
/// Rust wrapper for a display link that looks like iOS
/// CADisplayLink but uses MacOS CVDisplayLink underneath.
pub struct CoreAnimVideoDisplayLink {
    display_link_ref: CVDisplayLinkRef,
    source: dispatch_source_t,
    target: id,
    selector: SEL,
}
impl Default for CoreAnimVideoDisplayLink {
    fn default() -> Self {
        CoreAnimVideoDisplayLink {
           display_link_ref: null(),
            source: nil,
            target: nil,
            selector: selector("none"),
        }
    }
}
// impl Default for Pin<CoreAnimVideoDisplayLink> {
//     fn default() -> Self { Pin::new(CoreAnimVideoDisplayLink::default()) }
// }
impl Drop for CoreAnimVideoDisplayLink {
    fn drop(&mut self) {
        self.invalidate();
        unsafe {
            objc_release(self.source);
            objc_release(self.target);
        }
    }
}

//
// @implementation CAVDisplayLink
impl CoreAnimVideoDisplayLink {
    //
    // + (CAVDisplayLink *)displayLinkWithTarget:(id)target
    //                                  selector:(SEL)sel
    //                          didFailWithError: (NSError * _Nullable __autoreleasing *)error
    // {
    /// Returns a new display link wrapped in a Result.
    pub fn display_link_with_target_and_selector(target: &mut Object, selector: Sel) -> Result<Pin<Box<CoreAnimVideoDisplayLink>>, CoreAnimVideoError> {
        //   CAVDisplayLink * displayLink = [[CAVDisplayLink alloc] initWithTarget:(id)target
        //                                                                selector:(SEL)sel
        //                                                        didFailWithError:(NSError * _Nullable __autoreleasing *)error];
        //   return displayLink;
    // }
    //
    // - (instancetype) initWithTarget:(id)target
    //                        selector:(SEL)sel
    //                didFailWithError: (NSError * _Nullable __autoreleasing *)error {
        // //+ NSLog(@"In CAVDisplayLink initWithTarget");
        debug_log("In CoreAnimVideoDisplayLink display link with target and selector");
        let pool = unsafe { NSAutoreleasePool::new(nil) };
        // if (self = [super init]) {
        //   _target = target;
        //   _selector = sel;
        //   _source = dispatch_source_create(DISPATCH_SOURCE_TYPE_DATA_ADD, 0, 0, dispatch_get_main_queue());
        let queue = unsafe { dispatch_get_main_queue_not_inline() };
        let source = unsafe { dispatch_source_create(
            DISPATCH_SOURCE_TYPE_DATA_ADD,
            null(),
            0,
            queue
        ) };
        //
        // // Get a display link pointer
        // CVDisplayLinkRef displayLinkRef;
        // CGDirectDisplayID mainDisplay = CGMainDisplayID();
        // CVDisplayLinkCreateWithCGDisplay(mainDisplay, &displayLinkRef);
        // if (displayLinkRef == nil) {
        //   if (error != nil) {
        //     NSString *domain = @"com.trsolutions.DisplayLink.InitializationErrors";
        //        NSString *desc = NSLocalizedString(@"Unable to create display link", @"");
        //        NSDictionary *userInfo = @{ NSLocalizedDescriptionKey : desc };
        //
        //       *error = [NSError errorWithDomain:domain
        //                                    code: FAILED_TO_CREATE_DISPLAY_LINK
        //                                userInfo:userInfo];
        //
        //   }
        //   return nil;
        // }
        let mut display_link_ref: CVDisplayLinkRef = null();
        unsafe {
            let return_code = CVDisplayLinkCreateWithCGDisplay(
                CGMainDisplayID(),
                &mut display_link_ref
            );
            if display_link_ref == null() {
                return Err(CoreAnimVideoError::FailedToConnectToDisplay(return_code));
            }
        }
        // _displayLinkRef = CVDisplayLinkRetain(displayLinkRef);
        let display_link_ref = unsafe { CVDisplayLinkRetain(display_link_ref) };
        //
        // CVReturn returnCode = CVDisplayLinkSetOutputCallback(displayLinkRef, displayLinkCallback, (__bridge void * _Nullable)(_source));
        //
        // if (returnCode != kCVReturnSuccess) {
        //   if (error != nil) {
        //     NSString *domain = @"com.trsolutions.DisplayLink.InitializationErrors";
        //        NSString *desc = NSLocalizedString(@"Unable to link output handler", @"");
        //        NSDictionary *userInfo = @{ NSLocalizedDescriptionKey : desc };
        //
        //       *error = [NSError errorWithDomain:domain
        //                                    code:FAILED_TO_LINK_OUTPUT_HANDLER
        //                                userInfo:userInfo];
        //   }
        //   return nil;
        // }
        let return_code = unsafe { CVDisplayLinkSetOutputCallback(
            display_link_ref,
            display_link_callback as CVDisplayLinkOutputCallback,
            source as *const c_void,
        ) };
        if return_code != kCVReturnSuccess {
            return Err(CoreAnimVideoError::FailedToCreateTimer(return_code));
        }
        //
        // dispatch_source_set_event_handler_f_with_user_data(_source, &dispatch_event_handler_f, (__bridge void * _Nullable)(self));
        //
        // if (error != nil) {
        //   *error = nil;
        // }
        // //+NSLog(@"Display Link successfully initialized.");
        // }
        // return self;
        let display_link = Box::pin(CoreAnimVideoDisplayLink {
            display_link_ref,
            source,
            target: unsafe { objc_retain(target) },
            selector
        });
        unsafe { dispatch_source_set_event_handler_f_with_user_data(
            source,
            dispatch_event_handler_f::<fn(id)>,
            display_link.deref() as *const _ as _,
        ) };
        unsafe { pool.drain() }
        Ok(display_link)
    }
    // }
    //
    // - (void)addToRunLoop:(NSRunLoop *)runloop forMode:(NSRunLoopMode)mode {
    /// Registers the display link with a run loop.
    pub fn add_to_run_loop_for_mode(&self, _run_loop: id, _mode: id) {
        // CVDisplayLinkStart(_displayLinkRef);
        // dispatch_resume(_source);
        unsafe {
            CVDisplayLinkStart(self.display_link_ref);
            dispatch_resume(self.source)
        }
    }
    // }
    //
    //
    // - (void)invalidate {
    /// Removes the display link from all run loop modes.
    pub fn invalidate(&mut self) {
        // CVDisplayLinkStop(_displayLinkRef);
        // dispatch_suspend(_source);
        unsafe {
            CVDisplayLinkStop(self.display_link_ref);
            dispatch_suspend(self.source)
        }
    }
    // }

    //
    //
}
// @end
//
// #pragma mark - Callback
//
// static CVReturn displayLinkCallback
//  (
//     CVDisplayLinkRef displayLink,
//     const CVTimeStamp* now,
//     const CVTimeStamp* outputTime,
//     CVOptionFlags flagsIn,
//     CVOptionFlags* flagsOut,
//     void* displayLinkContext
//     )
// {
extern "C" fn display_link_callback(
    _display_link: CVDisplayLinkRef,
    _now: &CVTimeStamp,
    _output_time: &CVTimeStamp,
    _flags_in: CVOptionFlags,
    _flags_out: &mut CVOptionFlags,
    _display_link_context: *const c_void,
) -> CVReturn {
    // //+ NSLog(@"in displayLinkCallback");
    // NSObject<OS_dispatch_source> * source = (__bridge NSObject<OS_dispatch_source> *)displayLinkContext;
    // //+ NSLog(@"displayLinkContext = %@", displayLinkContext);
    // dispatch_source_merge_data(source, 1);
    // return kCVReturnSuccess;
    let source = _display_link_context as dispatch_source_t;
    unsafe { dispatch_source_merge_data(source, 1) };
    kCVReturnSuccess

}
// }
//
// static void dispatch_event_handler_f(dispatch_source_t _Nonnull source, void* _Nullable user_data) {
extern "C" fn dispatch_event_handler_f<F>(_source: dispatch_source_t, user_data: *const c_void)
    where F: Fn(id)
{
    // //+ NSLog(@"In dispatch event handler f, user_data = %@", user_data);
    // CAVDisplayLink *displayLink = (__bridge CAVDisplayLink *) user_data;
    // if (displayLink != nil && displayLink.target != nil) {
    //   IMP imp = [displayLink.target methodForSelector:displayLink.selector];
    //   void (*func)(id, SEL) = (void*) imp;
    //   func(displayLink.target, displayLink.selector);
    // }
    let user_display_link_ptr = user_data as *const CoreAnimVideoDisplayLink;
    match unsafe { user_display_link_ptr.as_ref() } {
        Some(display_link_p) => {
            let func: extern "C" fn(&Object, SEL, id) = unsafe { msg_send![display_link_p.target, methodForSelector:display_link_p.selector] };
            let target = unsafe { display_link_p.target.as_ref().unwrap() };
            func(target, display_link_p.selector, display_link_p as *const _ as _);
        },
        None => ()
    }
}
// }
