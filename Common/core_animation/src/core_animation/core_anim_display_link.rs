//
//  core_anim_display_link.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for CADisplayLink
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use objc::runtime::{Object, Sel, objc_retain, objc_release};
use crate::CoreAnimError;

#[cfg(target_os = "ios")]
use objc::class;

#[cfg(target_os = "macos")]
use {
    std::os::raw::{c_void, c_int, c_ulonglong, c_ulong},
    cocoa::quartzcore::CVTimeStamp,
    cocoa::base::{SEL},
    cocoa::foundation::NSAutoreleasePool,
    std::ptr::null,
    std::pin::Pin,
    std::ops::Deref,
};
use cocoa::foundation::{NSInteger, NSTimeInterval};


// System/Library/Frameworks/Foundation.framework/Headers/NSRunLoop.h:

// Items needed by mac version:
// Types from Core Video:
#[cfg(target_os = "macos")]
pub type CVDisplayLinkRef = *const c_void;
#[cfg(target_os = "macos")]
#[allow(non_camel_case_types)]
pub type dispatch_source_t = id;
#[cfg(target_os = "macos")]
#[allow(non_camel_case_types)]
pub type uintptr_t = *const c_void;
#[cfg(target_os = "macos")]
#[allow(non_camel_case_types)]
pub type dispatch_queue_t = id;
#[cfg(target_os = "macos")]
pub type CGDirectDisplayID = *const c_void;
#[cfg(target_os = "macos")]
pub type CVReturn = c_int;
#[cfg(target_os = "macos")]
pub type CVOptionFlags = c_ulonglong;
#[cfg(target_os = "macos")]
pub type CVDisplayLinkOutputCallback = *const extern "C" fn(display_link: CVDisplayLinkRef, now: *const CVTimeStamp, output_time: *const CVTimeStamp, flags_in: CVOptionFlags, flags_out: *mut CVOptionFlags, display_link_context: *const c_void) -> CVReturn;
#[cfg(target_os = "macos")]
#[allow(non_camel_case_types)]
pub type dispatch_object_t = id;

// CVReturn.h:    kCVReturnSuccess = 0
#[cfg(target_os = "macos")]
#[allow(non_upper_case_globals)]
static kCVReturnSuccess: c_int = 0;

#[cfg(target_os = "macos")]
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
#[cfg(target_os = "macos")]
#[link(name="System", kind="framework")]
extern {
    // oid dispatch_cancel(dispatch_object_t object);
    fn dispatch_source_cancel(object: dispatch_object_t);
}
// typedef void (*dispatch_function_with_user_data_t)(dispatch_source_t _Nonnull , void *_Nullable);
#[cfg(target_os = "macos")]
#[allow(non_camel_case_types)]
pub trait dispatch_function_with_user_data_trait: Fn(dispatch_source_t, *const c_void){}
#[cfg(target_os = "macos")]
#[allow(non_camel_case_types)]
pub type dispatch_function_with_user_data_t = extern "C" fn(source: dispatch_source_t, user_data: *const c_void);
#[cfg(target_os = "macos")]
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
#[cfg(target_os = "ios")]
extern {
    #[allow(non_upper_case_globals)]
    static NSRunLoopCommonModes: id;
}


// from usr/include/dispatch/source.h:
// #define DISPATCH_SOURCE_TYPE_DATA_ADD (&_dispatch_source_type_data_add)
#[cfg(target_os = "macos")]
static DISPATCH_SOURCE_TYPE_DATA_ADD: &c_void = unsafe { &_dispatch_source_type_data_add };

#[cfg(target_os = "macos")]
pub struct _CoreAnimDisplayLink {
    display_link_ref: CVDisplayLinkRef,
    source: dispatch_source_t,
    target: id,
    selector: SEL,
}
/// Thin Rust wrapper for CADisplayLink
pub struct CoreAnimDisplayLink {
    #[cfg(target_os = "ios")]
    display_link: id,
    #[cfg(target_os = "macos")]
    display_link: Option<Pin<Box<_CoreAnimDisplayLink>>>,
}
impl Default for CoreAnimDisplayLink {
    #[cfg(target_os = "ios")]
    fn default() -> Self { CoreAnimDisplayLink { display_link: nil } }
    #[cfg(target_os="macos")]
    fn default() -> Self {  CoreAnimDisplayLink { display_link: None } }
}
#[cfg(target_os = "ios")]
impl From<id> for CoreAnimDisplayLink {
    fn from(display_link: id) -> Self {
        let display_link = unsafe { objc_retain(display_link) };
        CoreAnimDisplayLink { display_link }
    }
}
#[cfg(target_os = "macos")]
impl From<Pin<Box<_CoreAnimDisplayLink>>> for CoreAnimDisplayLink {
    fn from(display_link: Pin<Box<_CoreAnimDisplayLink>>) -> Self {
        CoreAnimDisplayLink { display_link: Some(display_link) }
    }
}
#[cfg(target_os = "macos")]
impl Deref for CoreAnimDisplayLink {
    type Target = _CoreAnimDisplayLink;

    fn deref(&self) -> &Self::Target {
       self.display_link.as_ref().unwrap().deref()
    }
}
impl Drop for CoreAnimDisplayLink {
    #[cfg(target_os = "ios")]
    fn drop(&mut self) {
        unsafe { objc_release(self.display_link) }
    }
    #[cfg(target_os = "macos")]
    fn drop(&mut self) {
        self.invalidate();
        unsafe {
            if let Some(display_link) = &self.display_link {
                objc_release(display_link.source);
                objc_release(display_link.target);
            }
        }
    }
}

impl CoreAnimDisplayLink {
    #[cfg(target_os = "ios")]
    /// Returns a new display link.
    pub fn display_link_with_target_and_selector(target: &mut Object, selector: Sel) -> Result<CoreAnimDisplayLink, CoreAnimError> {
        let class = class!(CADisplayLink);
        let display_link = unsafe { msg_send![class, displayLinkWithTarget:target selector:selector] };
        Ok(CoreAnimDisplayLink { display_link })
    }
    #[cfg(target_os = "macos")]
    /// Returns a new display link.
    pub fn display_link_with_target_and_selector(target: &mut Object, selector: Sel) -> Result<CoreAnimDisplayLink, CoreAnimError> {
        //+ debug_log("In CoreAnimVideoDisplayLink display link with target and selector");
        let pool = unsafe { NSAutoreleasePool::new(nil) };
        let queue = unsafe { dispatch_get_main_queue_not_inline() };
        let source = unsafe { dispatch_source_create(
            DISPATCH_SOURCE_TYPE_DATA_ADD,
            null(),
            0,
            queue
        ) };

        let mut display_link_ref: CVDisplayLinkRef = null();
        unsafe {
            let return_code = CVDisplayLinkCreateWithCGDisplay(
                CGMainDisplayID(),
                &mut display_link_ref
            );
            if display_link_ref == null() {
                return Err(CoreAnimError::FailedToConnectToDisplay(return_code));
            }
        }
        let display_link_ref = unsafe { CVDisplayLinkRetain(display_link_ref) };

        let return_code = unsafe { CVDisplayLinkSetOutputCallback(
            display_link_ref,
            display_link_callback as CVDisplayLinkOutputCallback,
            source as *const c_void,
        ) };
        if return_code != kCVReturnSuccess {
            return Err(CoreAnimError::FailedToCreateTimer(return_code));
        }

        let display_link = Box::pin(_CoreAnimDisplayLink {
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
        Ok(CoreAnimDisplayLink::from(display_link))
    }
    /// Sets the preferred frame rate for the display link callback.
    #[allow(unused)]
    pub fn set_preferred_frames_per_second(&self, preferred_frames_per_second: NSInteger) {
        #[cfg(target_os = "ios")]
        unsafe { msg_send![self.display_link, setPreferredFramesPerSecond:preferred_frames_per_second] }
    }
    /// Registers the display link with a run loop.
    pub fn add_to_run_loop_for_mode(&self, _run_loop: id, _mode: id) {
        #[cfg(target_os = "ios")]
        unsafe { msg_send![self.display_link, addToRunLoop:_run_loop forMode:_mode] }
        #[cfg(target_os = "macos")]
            unsafe {
            CVDisplayLinkStart(self.display_link_ref);
            dispatch_resume(self.source)
        }
    }
    /// Registers the display link with the standard loop.
    pub fn add_to_main_run_loop(&self) {
        #[cfg(target_os = "ios")]
        let (run_loop, mode) = self.get_main_run_loop();
        #[cfg(target_os = "macos")]
        let (run_loop, mode) = (nil, nil);

        self.add_to_run_loop_for_mode(run_loop, mode)
    }
    #[cfg(target_os = "ios")]
    #[inline]
    fn get_main_run_loop(&self) -> (id, id) {
        let run_loop_class = class!(NSRunLoop);
        let run_loop = unsafe { msg_send![run_loop_class, mainRunLoop] };
        let mode = unsafe { NSRunLoopCommonModes };
        (run_loop, mode)
    }
    /// Gets the duration component of a type checking result.
    pub fn get_duration(&self) -> NSTimeInterval {
        #[cfg(target_os = "ios")]
        unsafe { msg_send![self.display_link, duration] }
        #[cfg(target_os = "macos")]
        return 1. / 60.
    }
    /// Removes the display link from all run loop modes.
    pub fn invalidate(&mut self) {
        #[cfg(target_os = "ios")]
        unsafe { msg_send![self.display_link, invalidate] }
        #[cfg(target_os = "macos")]
        unsafe {
            if self.display_link.is_some()
            {
                CVDisplayLinkStop(self.display_link_ref);
                dispatch_suspend(self.source)
            }
        }
    }
}
#[cfg(target_os = "macos")]
extern "C" fn display_link_callback(
    _display_link: CVDisplayLinkRef,
    _now: &CVTimeStamp,
    _output_time: &CVTimeStamp,
    _flags_in: CVOptionFlags,
    _flags_out: &mut CVOptionFlags,
    _display_link_context: *const c_void,
) -> CVReturn {
    let source = _display_link_context as dispatch_source_t;
    unsafe { dispatch_source_merge_data(source, 1) };
    kCVReturnSuccess
}
#[cfg(target_os = "macos")]
extern "C" fn dispatch_event_handler_f<F>(_source: dispatch_source_t, user_data: *const c_void)
    where F: Fn(id)
{
    let user_display_link_ptr = user_data as *const _CoreAnimDisplayLink;
    match unsafe { user_display_link_ptr.as_ref() } {
        Some(display_link_p) => {
            let func: extern "C" fn(&Object, SEL, id) = unsafe { msg_send![display_link_p.target, methodForSelector:display_link_p.selector] };
            let target = unsafe { display_link_p.target.as_ref().unwrap() };
            func(target, display_link_p.selector, display_link_p as *const _ as _);
        },
        None => ()
    }
}
