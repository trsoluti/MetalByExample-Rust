//
//  core_anim_metal_drawable.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for CAMetalDrawable
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::id;
use objc::runtime::{objc_release, objc_retain};

/// Thin wrapper for CAMetalDrawable
pub struct CoreAnimMetalDrawable {
    drawable: id,
}
impl From<id> for CoreAnimMetalDrawable {
    fn from(drawable: id) -> Self {
        let drawable = unsafe { objc_retain(drawable) };
        CoreAnimMetalDrawable { drawable }
    }
}
impl Drop for CoreAnimMetalDrawable {
    fn drop(&mut self) { unsafe { objc_release(self.drawable) } }
}

impl CoreAnimMetalDrawable {
    /// Gets the current texture value
    pub fn get_texture(&self) -> id {
        unsafe { msg_send![self.drawable, texture] }
    }
    /// Returns true if the drawable is not null
    pub fn is_not_null(&self) -> bool {
        !self.drawable.is_null()
    }
    /// Returns the underlying objective c drawable
    pub fn to_objc(&self) -> id { self.drawable }
}
