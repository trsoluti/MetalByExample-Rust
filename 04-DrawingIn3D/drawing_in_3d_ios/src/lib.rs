//
//  lib.rs
//
//  Created by TR Solutions on 2020-07-18.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
#![allow(dead_code)]
#![deny(missing_docs)]
//! Library to implement Metal by Example 04 Drawing in 3D in Rust
//!
//! Compile this and then compile/link the Objective C portion.
mod app_delegate;
mod scene_delegate;
mod view_controller;
mod mbe_items;
mod generic_id;

use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};
// (Crate items are pub so that they get picked up by doc creator.)
pub use crate::app_delegate::AppDelegateRust;
pub use crate::scene_delegate::SceneDelegateRust;
pub use crate::view_controller::ViewControllerRust;
pub use crate::mbe_items::RustMetalView;
pub use crate::mbe_items::RustMetalViewDelegate;
pub use crate::mbe_items::RustMBERenderer;


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
    SceneDelegateRust::register();
    ViewControllerRust::register();
    RustMetalView::register();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
