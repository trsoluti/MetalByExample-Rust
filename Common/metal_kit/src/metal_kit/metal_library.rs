//
//  metal_library.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for methods we use from MTLLibrary

use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id, nil};
use objc::runtime::{objc_retain, objc_release};
use cocoa::foundation::{NSAutoreleasePool, NSString};

/// Rust wrapper for a collection of Metal shader functions.
pub struct MetalLibrary {
    library: id,
}
impl Default for MetalLibrary {
    fn default() -> Self {
        MetalLibrary { library: nil }
    }
}
impl From<id> for MetalLibrary {
    fn from(library: id) -> Self {
        let library = unsafe { objc_retain(library) };
        MetalLibrary { library }
    }
}
impl Drop for MetalLibrary {
    fn drop(&mut self) { unsafe { objc_release(self.library) } }
}

impl MetalLibrary {
    /// Creates an object that represents a shader function in the library.
    pub fn new_function_with_name(&self, name: &str) -> id {
        // id<MTLFunction> vertexFunc = [library newFunctionWithName:@"vertex_main"];
        unsafe {
            let pool =  NSAutoreleasePool::new(nil);
            let function_name = NSString::alloc(nil).init_str(name);
            let function: id = objc_retain(msg_send![self.library, newFunctionWithName:function_name]);
            pool.drain();
            function
        }
    }
}
