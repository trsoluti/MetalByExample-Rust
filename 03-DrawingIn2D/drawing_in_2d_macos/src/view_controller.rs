//
//  view_controller.rs
//
//  Created by TR Solutions on 2020-07-15.
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
use objc::runtime::{Object, Sel, object_getClass, Class};
use state::Storage;
use std::sync::{RwLock, RwLockReadGuard, LockResult, RwLockWriteGuard};
use std::ptr::null_mut;
use crate::debug_log;
use cocoa::base::id;

// //
// //  ViewController.h
// //  DrawingIn2DMacOS
// //
// //  Created by TR Solutions on 14/7/20.
// //  Copyright © 2020 TR Solutions.
// //
//
// #import <Cocoa/Cocoa.h>
//
// @interface ViewController : NSViewController
/// Rust implementation of our application's view controller
#[derive(Default)]
pub struct ViewControllerRust;

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
        let ui_view_controller_class = class!(NSViewController);
        let mut view_controller_class_decl = ClassDecl::new("ViewController", ui_view_controller_class).unwrap();
        unsafe {
            view_controller_class_decl.add_ivar::<ViewControllerPtr>("rustViewController");
            view_controller_class_decl.add_method(
                sel!(viewDidLoad),
                view_did_load as extern "C" fn(&Object, Sel),
            );
            view_controller_class_decl.add_method(
                sel!(setRepresentedObject:),
                set_represented_object as extern "C" fn(&mut Object, Sel, id),
            );
            view_controller_class_decl.add_method(
                sel!(dealloc),
                dealloc as extern "C" fn(&mut Object, Sel),
            );
        }
        view_controller_class_decl.register();
    }
}
//
//
// @end
//
// ==================================================================================================
// //
// //  ViewController.m
// //  DrawingIn2DMacOS
// //
// //  Created by TR Solutions on 14/7/20.
// //  Copyright © 2020 TR Solutions.
// //
//
// #import "ViewController.h"
//
// @implementation ViewController
//
// - (void)viewDidLoad {
extern "C" fn view_did_load(_self: &Object, _sel: Sel) {
    //   [super viewDidLoad];
    unsafe {
        let class = object_getClass(_self);
        let superclass: &Class = msg_send![class, superclass];
        msg_send![super(_self, superclass), viewDidLoad]
    }

    //   // Do any additional setup after loading the view.
    debug_log("In ViewController: view_did_load");
}
// }
//
//
// - (void)setRepresentedObject:(id)representedObject {
extern "C" fn set_represented_object(_self: &mut Object, _sel: Sel, represented_object: id) {
    // [super setRepresentedObject:representedObject];
    unsafe {
        let class = object_getClass(_self);
        let superclass: &Class = msg_send![class, superclass];
        msg_send![super(_self, superclass), setRepresentedObject:represented_object]
    }
    //
    // // Update the view, if already loaded.
    debug_log("In ViewController:set_represented_object")
}
// }
//
extern "C" fn dealloc(_self: &mut Object, _sel: Sel) {
    // This method should free the objects allocated when initializing the ViewControllerPtr
    unsafe {
        let view_controller_ptr_ptr = _self.get_mut_ivar::<ViewControllerPtr>("rustViewController");
        if !view_controller_ptr_ptr.view_controller_ptr.is_null() {
            view_controller_ptr_ptr.drop() // let everything go
        }
    }

}
//
// @end
//
//  */