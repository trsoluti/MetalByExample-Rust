//
//  core_anim_display_link.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for CADisplayLink
use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use objc::runtime::{Object, Sel, objc_retain, objc_release};

// System/Library/Frameworks/Foundation.framework/Headers/NSRunLoop.h:

/// Thin Rust wrapper for CADisplayLink
pub struct CoreAnimDisplayLink {
    display_link: id,
}
impl Default for CoreAnimDisplayLink {
    fn default() -> Self {
        CoreAnimDisplayLink { display_link: nil }
    }
}
impl From<id> for CoreAnimDisplayLink {
    fn from(display_link: id) -> Self {
        let display_link = unsafe { objc_retain(display_link) };
        CoreAnimDisplayLink { display_link }
    }
}
impl Drop for CoreAnimDisplayLink {
    fn drop(&mut self) {
        unsafe { objc_release(self.display_link) }
    }
}

impl CoreAnimDisplayLink {
    /// Returns a new display link.
    pub fn display_link_with_target_and_selector(object: &mut Object, selector: Sel) -> CoreAnimDisplayLink {
        let class = class!(CADisplayLink);
        let display_link = unsafe { msg_send![class, displayLinkWithTarget:object selector:selector] };
        CoreAnimDisplayLink { display_link }
    }
    /// Registers the display link with a run loop.
    pub fn add_to_run_loop_for_mode(&self, run_loop: id, mode: id) {
        unsafe { msg_send![self.display_link, addToRunLoop:run_loop forMode:mode] }
    }
    /// Removes the display link from all run loop modes.
    pub fn invalidate(&mut self) {
        unsafe { msg_send![self.display_link, invalidate] }
    }
}
