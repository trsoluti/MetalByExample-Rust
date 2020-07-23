//
//  app_delegate.rs
//
//  Created by TR Solutions on 2020-07-13.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//

//! Emulates the standard UIKit AppDelegate class in Rust
use objc::class;
use objc::sel;
use objc::sel_impl;
use cocoa::base::{id};
use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};
use crate::{debug_log};

// /*
// //
// //  AppDelegate.h
// //  DrawingIn3DMacOS
// //
// //  Created by TR Solutions on 14/7/20.
// //  Copyright © 2020 TR Solutions. All rights reserved.
// //
//
// #import <Cocoa/Cocoa.h>
//
// @interface AppDelegate : NSObject <NSApplicationDelegate>
/// Rust version of an Objective C AppDelegate class
pub struct AppDelegateRust;

//
//
// @end
//
// ================================================================================================
// //
// //  AppDelegate.m
// //  DrawingIn2DMacOS
// //
// //  Created by TR Solutions on 14/7/20.
// //  Copyright © 2020 TR Solutions. All rights reserved.
// //
//
// #import "AppDelegate.h"
//
// @interface AppDelegate ()
impl AppDelegateRust {
    /// Registers the AppDelegate class and its methods with the Objective C runtime
    pub fn register() {
        let ns_object_class = class!(NSObject);
        let mut app_delegate_builder = ClassDecl::new("AppDelegate", ns_object_class).unwrap();
        unsafe {
            app_delegate_builder.add_method(
                sel!(applicationDidFinishLaunching:),
                application_did_finish_launching_with_notification as extern "C" fn(&Object, Sel, id),
            );
            app_delegate_builder.add_method(
                sel!(applicationWillTerminate:),
                application_will_terminate_with_notification as extern "C" fn(&Object, Sel, id),
            )
        }
        app_delegate_builder.register();
    }
}

//
// @end
//
// @implementation AppDelegate
//
// - (void)applicationDidFinishLaunching:(NSNotification *)aNotification {
extern "C" fn application_did_finish_launching_with_notification(_self: &Object, _sel: Sel, _notification: id) {
    // // Insert code here to initialize your application
    debug_log("In Application did finish launching with notification")
}
// }
//
//
// - (void)applicationWillTerminate:(NSNotification *)aNotification {
extern "C" fn application_will_terminate_with_notification(_self: &Object, _sel: Sel, _notification: id) {
    // // Insert code here to tear down your application
    debug_log("In application will terminate")
}
// }
//
//
// @end
//
//  */