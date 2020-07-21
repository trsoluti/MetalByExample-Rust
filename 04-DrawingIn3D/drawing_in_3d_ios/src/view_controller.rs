//
//  view_controller.rs
//
//  Created by TR Solutions on 2020-07-18.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//

//! Implements our application's ViewController in Rust
use objc::{class, Encode, Encoding};
use objc::sel;
use objc::sel_impl;
use objc::msg_send;
use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel, object_getClass, Class, BOOL, YES};
use state::Storage;
use std::sync::{RwLock, RwLockReadGuard, LockResult, RwLockWriteGuard};
use std::ptr::null_mut;
use crate::debug_log;

// //
// //  ViewController.h
// //  SimpleIOSViewController
// //
// //  Created by TR Solutions on 5/7/20.
// //  Copyright © 2020 TR Solutions. All rights reserved.
// //
//
// @import UIKit
//
// @interface ViewController : UIViewController
/// Rust implementation of our application's view controller
#[derive(Default)]
pub struct ViewControllerRust {
    // renderer: RustMBERenderer,
}

unsafe impl Send for ViewControllerRust {}
unsafe impl Sync for ViewControllerRust {}
type ViewControllerRustState = Storage<RwLock<ViewControllerRust>>;
#[derive(Copy, Clone)]
struct ViewControllerPtr { view_controller_ptr: *mut ViewControllerRustState, }
unsafe impl Sync for ViewControllerPtr {}
impl ViewControllerPtr {
    fn init(&self) {
        let storage = Box::into_raw(Box::new(ViewControllerRustState::new()));
        let view_controller_ptr_ptr = &self.view_controller_ptr as *const _ as *mut *mut ViewControllerRustState;
        unsafe { view_controller_ptr_ptr.write(storage); }
        unsafe { (*storage).set(RwLock::new(ViewControllerRust::default())); }
    }
    fn read(&self) -> LockResult<RwLockReadGuard<'_, ViewControllerRust>> {
        unsafe { (*self.view_controller_ptr).get().read() }
    }
    fn write(&mut self) -> LockResult<RwLockWriteGuard<'_, ViewControllerRust>> {
        unsafe { (*self.view_controller_ptr).get().write() }
    }
    fn drop(&mut self) {
        unsafe {
            let _p = Box::from_raw(self.view_controller_ptr);
            // dropping p should drop everything
            self.view_controller_ptr = null_mut();
        }
    }
}
unsafe impl Encode for ViewControllerPtr {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("^v") } // "^v" represents
    }
}

impl ViewControllerRust {
    /// Registers the ViewController class and its methods with the Objective C runtime
    pub fn register() {
        let ui_view_controller_class = class!(UIViewController);
        let mut view_controller_class_decl = ClassDecl::new("ViewController", ui_view_controller_class).unwrap();
        unsafe {
            view_controller_class_decl.add_ivar::<ViewControllerPtr>("rustViewController");
            view_controller_class_decl.add_method(
                sel!(viewDidLoad),
                view_did_load as extern "C" fn(&mut Object, Sel),
            );
            view_controller_class_decl.add_method(
                sel!(dealloc),
                dealloc as extern "C" fn(&mut Object, Sel),
            );
            view_controller_class_decl.add_method(
                sel!(prefersStatusBarHidden),
                prefers_status_bar_hidden as extern "C" fn(&Object, Sel) -> BOOL,
            )
        }
        view_controller_class_decl.register();
    }
}
//
//
// @end
//
// ============================================================================================
// //
// //  ViewController.m
// //  SimpleIOSViewController
// //
// //  Created by TR Solutions on 5/7/20.
// //  Copyright © 2020 TR Solutions. All rights reserved.
// //
//
// #import "ViewController.h"
//
// @implementation ViewController
//
// - (void)viewDidLoad {
//     [super viewDidLoad];
// }
//
// - (void)viewDidLoad {
extern "C" fn view_did_load(_self: &mut Object, _sel: Sel) {
    //   [super viewDidLoad];
    unsafe {
        let class = object_getClass(_self);
        let superclass: &Class = msg_send![class, superclass];
        msg_send![super(_self, superclass), viewDidLoad]
    }

    //   // Do any additional setup after loading the view.
    debug_log("In ViewController: view_did_load");
    // self.renderer = [MBERenderer new];
    // self.metalView.delegate = self.renderer;
    //x let renderer = RustMBERenderer::new();
    //x let view:id = unsafe { msg_send![_self, view] };
    //x let _:() = unsafe { msg_send![view, setDelegate: renderer.to_objc()]};
    //x let mut rust_view_controller = get_mut_rust_view_controller(_self).unwrap();
    //x rust_view_controller.renderer = renderer;
    // TOD: it's not rust-appropriate for ViewController to own
    // the renderer and then have MetalView refer to it.
    // The code to set the renderer has been moved to MetalView.
}
// }
//
// - (BOOL)prefersStatusBarHidden {
extern "C" fn prefers_status_bar_hidden(_self: &Object, _sel: Sel) -> BOOL {
    // return YES;
    YES
}
// }
//
// @end
//
fn get_rust_view_controller(_self: &Object) -> LockResult<RwLockReadGuard<'_, ViewControllerRust>> {
    unsafe {
        let view_controller_ptr_ptr = _self.get_ivar::<ViewControllerPtr>("rustViewController");
        if view_controller_ptr_ptr.view_controller_ptr.is_null() {
            view_controller_ptr_ptr.init() // will point a pointer in.
        }
        view_controller_ptr_ptr.read()
    }

}
fn get_mut_rust_view_controller(_self: &mut Object) -> LockResult<RwLockWriteGuard<'_, ViewControllerRust>> {
    unsafe {
        let view_controller_ptr_ptr = _self.get_mut_ivar::<ViewControllerPtr>("rustViewController");
        if view_controller_ptr_ptr.view_controller_ptr.is_null() {
            view_controller_ptr_ptr.init() // will point a pointer in.
        }
        view_controller_ptr_ptr.write()
    }
}
extern "C" fn dealloc(_self: &mut Object, _sel: Sel) {
    // This method should free the objects allocated when initializing the ViewControllerPtr
    unsafe {
        let view_controller_ptr_ptr = _self.get_mut_ivar::<ViewControllerPtr>("rustViewController");
        if !view_controller_ptr_ptr.view_controller_ptr.is_null() {
            view_controller_ptr_ptr.drop() // let everything go
        }
    }

}
