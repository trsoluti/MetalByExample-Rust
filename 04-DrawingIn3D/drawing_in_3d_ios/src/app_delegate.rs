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
use objc::msg_send;
use cocoa::foundation::NSString;
use cocoa::base::{nil, id};
use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel};
use crate::{debug_log};

// //
// //  AppDelegate.h
// //  HelloRustiPad
// //
// //  Created by TR Solutions on 4/7/20.
// //  Copyright © 2020 TR Solutions. All rights reserved.
// //
//
// #import <UIKit/UIKit.h>
//
// @interface AppDelegate : UIResponder <UIApplicationDelegate>
//
//
// @end
/// Rust version of an Objective C AppDelegate class
pub struct AppDelegateRust;

impl AppDelegateRust {
    /// Registers the AppDelegate class and its methods with the Objective C runtime
    pub fn register() {
        let ns_object_class = class!(NSObject);
        let mut app_delegate_builder = ClassDecl::new("AppDelegate", ns_object_class).unwrap();
        unsafe {
            app_delegate_builder.add_method(
                sel!(application:didFinishLaunchingWithOptions:),
                application_did_finish_launching_with_options as extern "C" fn(&Object, Sel, id, id) -> bool,
            );
            app_delegate_builder.add_method(
                sel!(application:configurationForConnectingSceneSession:options:),
                application_configuration_for_connecting_scene_session_options as extern "C" fn(&Object, Sel, id, id, id) -> id,
            )
        }
        app_delegate_builder.register();
    }
}
// //
// //  AppDelegate.m
// //  HelloRustiPad
// //
// //  Created by TR Solutions on 4/7/20.
// //  Copyright © 2020 TR Solutions. All rights reserved.
// //
//
// #import "AppDelegate.h"
//
// @interface AppDelegate ()
//
// @end
//
// @implementation AppDelegate
//
//
// - (BOOL)application:(UIApplication *)application didFinishLaunchingWithOptions:(NSDictionary *)launchOptions {
//   // Override point for customization after application launch.
//   return YES;
// }
#[allow(dead_code)]
extern "C" fn application_did_finish_launching_with_options(_self: &Object, _sel: Sel, _application: id, _launch_options: id) -> bool {
    // Override point for customization after application launch.
    debug_log("in application did finish launching with options.");
    true
}
//
//
// #pragma mark - UISceneSession lifecycle
//
//
// - (UISceneConfiguration *)application:(UIApplication *)application configurationForConnectingSceneSession:(UISceneSession *)connectingSceneSession options:(UISceneConnectionOptions *)options {
//   // Called when a new scene session is being created.
//   // Use this method to select a configuration to create the new scene with.
//   return [[UISceneConfiguration alloc] initWithName:@"Default Configuration" sessionRole:connectingSceneSession.role];
// }
#[allow(dead_code)]
extern "C" fn application_configuration_for_connecting_scene_session_options(_self: &Object, _sel: Sel, _application: id, _connecting_scene_session: id, _options: id) -> id {
    debug_log("in application configuration for connecting scene session with options.");
    let ui_scene_configuration_class = class!(UISceneConfiguration);
    let allocated_class:id = unsafe { msg_send![ui_scene_configuration_class, alloc] };
    let default_configuration_name = unsafe { NSString::alloc(nil).init_str("Default Configuration") };
    let role:id = unsafe { msg_send![_connecting_scene_session, role] };
    let initialized_class:id =unsafe { msg_send![allocated_class, initWithName:default_configuration_name sessionRole:role] };
    initialized_class
}
//
//
// - (void)application:(UIApplication *)application didDiscardSceneSessions:(NSSet<UISceneSession *> *)sceneSessions {
//   // Called when the user discards a scene session.
//   // If any sessions were discarded while the application was not running, this will be called shortly after application:didFinishLaunchingWithOptions.
//   // Use this method to release any resources that were specific to the discarded scenes, as they will not return.
// }
//
//
// @end
//
