//
//  lib.rs
//
//  Created by TR Solutions on 2020-07-15.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
#![allow(dead_code)]
#![deny(missing_docs)]
//! Library to implement Metal by Example 03 Drawing in 2D (Mac OS) in rust
//!
//! Compile this and then compile/link the Objective C portion.

mod app_delegate;
mod view_controller;
mod metal_view;
mod display_link;

use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};

// (Crate items are pub so that they get picked up by doc creator.)
pub use crate::app_delegate::AppDelegateRust;
pub use crate::view_controller::ViewControllerRust;
pub use crate::metal_view::MetalViewRust;
pub use crate::display_link::CoreAnimVideoDisplayLink;
pub use crate::display_link::CoreAnimVideoError;

extern {
    pub fn NSLog(format: id);
}
/// Calls Objective C's NSLog with one rust string argument
pub fn  debug_log(message: &str) {
    unsafe {
        let pool = NSAutoreleasePool::new(nil);
        let ns_message = NSString::alloc(nil).init_str(message);
        NSLog(ns_message);
        pool.drain();
    }
}

#[no_mangle]
/// Registers all the rust classes with the Objective C runtime
pub extern "C" fn register_rust_classes() {
    debug_log("In register_rust_classes");
    AppDelegateRust::register();
    ViewControllerRust::register();
    MetalViewRust::register();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
