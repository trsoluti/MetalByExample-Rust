//
//  display_link.rs
//
//  Created by TR Solutions on 2020-07-13.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//

//! A temporary Rust wrapper for CAVDisplayLink
//! Eventually it will be merged into the Rust object
//! CoreAnimDisplayLink

use objc::class;
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use objc::runtime::{objc_retain, objc_release, Object, Sel};
use std::fmt::{Display, Formatter};
use std::error::Error;
use cocoa::foundation::NSAutoreleasePool;

/// The error returned if the system failed
/// to create a render pipeline state
#[derive(Debug)]
pub enum CoreAnimVideoError {
    /// The system returned the given Objective C error
    /// when attempting to create a render pipeline state
    CoreAnimVideoCreationError(id),
}
impl Display for CoreAnimVideoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Core Anim Video Creation error")
    }
}
impl Error for CoreAnimVideoError {}

/// Rust wrapper for a display link that looks like iOS
/// CADisplayLink but uses MacOS CVDisplayLink underneath.
pub struct CoreAnimVideoDisplayLink {
    link: id
}
impl Default for CoreAnimVideoDisplayLink {
    fn default() -> Self { CoreAnimVideoDisplayLink { link: nil } }
}
impl From<id> for CoreAnimVideoDisplayLink {
    fn from(link: id) -> Self {
        let link = unsafe { objc_retain(link) };
        CoreAnimVideoDisplayLink { link }
    }
}
impl Drop for CoreAnimVideoDisplayLink {
    fn drop(&mut self) { unsafe { objc_release(self.link) } }
}
impl CoreAnimVideoDisplayLink {
    /// Returns a new display link wrapped in a Result.
    pub fn display_link_with_target_and_selector(_self: &Object, _sel: Sel) -> Result<CoreAnimVideoDisplayLink, CoreAnimVideoError> {
        let pool = unsafe { NSAutoreleasePool::new(nil) };
        let error: id = nil;
        let class = class!(CAVDisplayLink);
        let link:id = unsafe { msg_send![class, displayLinkWithTarget:_self selector:_sel didFailWithError:&error] };
        let result = if error != nil {
            Err(CoreAnimVideoError::CoreAnimVideoCreationError(error))
        } else {
            Ok(CoreAnimVideoDisplayLink::from(link))
        };
        unsafe { pool.drain() }
        result
    }
    /// Registers the display link with a run loop.
    pub fn add_to_run_loop_for_mode(&mut self, run_loop: id, mode: id) {
        unsafe { msg_send![self.link, addToRunLoop:run_loop forMode:mode] }
    }

    /// Removes the display link from all run loop modes.
    pub fn invalidate(&mut self) {
        unsafe { msg_send![self.link, invalidate] }
    }
}