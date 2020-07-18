//
//  scene_delegate.rs
//
//  Created by TR Solutions on 2020-07-13.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//

//! Emulates the standard UIKit SceneDelegate class in Rust
use objc::{class, Encode, Encoding};
use objc::msg_send;
use objc::sel;
use objc::sel_impl;
use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel, Protocol, objc_retain, objc_release};
use cocoa::base::{id, nil};
use state::Storage;
use std::sync::{RwLock, RwLockReadGuard, LockResult, RwLockWriteGuard};
use std::ptr::null_mut;
use crate::{debug_log};
use cocoa::foundation::NSUInteger;

// //
// //  SceneDelegate.h
// //  SimpleIOSViewController
// //
// //  Created by TR Solutions on 5/7/20.
// //  Copyright © 2020 TR Solutions. All rights reserved.
// //
//
// #import <UIKit/UIKit.h>
//
// @interface SceneDelegate : UIResponder <UIWindowSceneDelegate>
/// Rust implementation of a standard UIKit SceneDelegate
pub struct SceneDelegateRust {
    window: id,
}
unsafe impl Send for SceneDelegateRust {}
unsafe impl Sync for SceneDelegateRust {}
impl Default for SceneDelegateRust {
    fn default() -> Self {
        SceneDelegateRust {
            window: nil,
        }
    }
}
impl Drop for SceneDelegateRust {
    fn drop(&mut self) { unsafe { objc_release(self.window) } }
}
type SceneDelegateRustState = Storage<RwLock<SceneDelegateRust>>;
#[derive(Copy, Clone)]
struct SceneDelegatePtr { view_controller_ptr: *mut SceneDelegateRustState, }
unsafe impl Sync for SceneDelegatePtr {}
impl SceneDelegatePtr {
    fn init(&self) {
        let storage = Box::into_raw(Box::new(SceneDelegateRustState::new()));
        let view_controller_ptr_ptr = &self.view_controller_ptr as *const _ as *mut *mut SceneDelegateRustState;
        unsafe { view_controller_ptr_ptr.write(storage); }
        unsafe { (*storage).set(RwLock::new(SceneDelegateRust::default())); }
    }
    fn read(&self) -> LockResult<RwLockReadGuard<'_, SceneDelegateRust>> {
        unsafe { (*self.view_controller_ptr).get().read() }
    }
    fn write(&mut self) -> LockResult<RwLockWriteGuard<'_, SceneDelegateRust>> {
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
unsafe impl Encode for SceneDelegatePtr {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("^v") } // "^v" represents *mut c_void
    }
}

impl SceneDelegateRust {
    /// Registers the SceneDelegate class and its methods with the Objective C runtime.
    pub fn register() {
        let base_class = class!(UIResponder);
        let mut scene_delegate = ClassDecl::new("SceneDelegate", base_class).unwrap();
        unsafe {
            scene_delegate.add_ivar::<SceneDelegatePtr>("rustSceneDelegate");
            let protocol = Protocol::get("UIWindowSceneDelegate").unwrap();
            scene_delegate.add_protocol(protocol);

            //
            // @property (strong, nonatomic) UIWindow * window;
            scene_delegate.add_method(
                sel!(window),
                get_window as extern "C" fn(&Object, Sel) -> id,
            );
            scene_delegate.add_method(
                sel!(setWindow:),
                set_window as extern "C" fn(&mut Object, Sel, id),
            );
            scene_delegate.add_method(
                sel!(dealloc),
                dealloc as extern "C" fn(&mut Object, Sel),
            );
            // - scene:willConnectToSession:options:
            scene_delegate.add_method(
                sel!(scene:willConnectToSession:options:),
                scene_will_connect_to_session_options as extern "C" fn(&Object, Sel, id, id, id),
            );
            // TOD: from UIWindowSceneDelegate
            // - (void)windowScene:(UIWindowScene *)windowScene
            // didUpdateCoordinateSpace:(id<UICoordinateSpace>)previousCoordinateSpace
            // interfaceOrientation:(UIInterfaceOrientation)previousInterfaceOrientation
            //     traitCollection:(UITraitCollection *)previousTraitCollection;
            scene_delegate.add_method(
                sel!(windowScene:didUpdateCoordinateSpace:interfaceOrientation:traitCollection:),
                window_scene_did_update_coordinate_space_interface_orientation_trait_collection as extern "C" fn(&mut Object, Sel, id, id, id, id),
            );
            // - sceneDidDisconnect:
            scene_delegate.add_method(
                sel!(sceneDidDisconnect:),
                scene_did_disconnect as extern "C" fn(&Object, Sel, id),
            );
            // - sceneWillEnterForeground:
            scene_delegate.add_method(
                sel!(sceneWillEnterForeground:),
                scene_will_enter_foreground as extern "C" fn(&Object, Sel, id),
            );
            // - sceneDidBecomeActive:
            scene_delegate.add_method(
                sel!(sceneDidBecomeActive:),
                scene_did_become_active as extern "C" fn(&Object, Sel, id),
            );
            // - sceneWillResignActive:
            scene_delegate.add_method(
                sel!(sceneWillResignActive:),
                scene_will_resign_active as extern "C" fn(&Object, Sel, id),
            );
            // - sceneDidEnterBackground:
            scene_delegate.add_method(
                sel!(sceneDidEnterBackground:),
                scene_did_enter_background as extern "C" fn(&Object, Sel, id),
            );
            //
            // UISceneConnectionOptions
            // // - scene:openURLContexts:
            // scene_delegate.add_method(
            //     sel!(scene:openURLContexts:),
            //     scene_open_url_contexts as extern "C" fn(&Object, Sel, id, id),
            // );
            // // - scene:willContinueUserActivityWithType:
            // scene_delegate.add_method(
            //     sel!(scene:willContinueUserActivityWithType:),
            //     scene_will_continue_user_activity_with_type as extern "C" fn(&Object, Sel, id, id),
            // );
            // // - scene:continueUserActivity:
            // scene_delegate.add_method(
            //     sel!(scene:continueUserActivity:),
            //     scene_continue_user_activity as extern "C" fn(&Object, Sel, id, id),
            // );
            // // - scene:didFailToContinueUserActivityWithType:error:
            // scene_delegate.add_method(
            //     sel!(scene:didFailToContinueUserActivityWithType:error:),
            //     scene_did_fail_to_continue_user_activity_with_type_error as extern "C" fn(&Object, Sel, id, id, id),
            // );
            // // - stateRestorationActivityForScene:
            // scene_delegate.add_method(
            //     sel!(stateRestorationActivityForScene:),
            //     state_restoration_activity_for_scene as extern "C" fn(&Object, Sel, id) -> id,
            // );
            // // - scene:didUpdateUserActivity:
            // scene_delegate.add_method(
            //     sel!(scene:didUpdateUserActivity:),
            //     scene_did_update_user_activity as extern "C" fn(&Object, Sel, id, id),
            // );
            // //
        }
        scene_delegate.register();
    }
}
//
// @end
//
// //
// //  SceneDelegate.m
// //  SimpleIOSViewController
// //
// //  Created by TR Solutions on 5/7/20.
// //  Copyright © 2020 TR Solutions. All rights reserved.
// //
//
// #import "SceneDelegate.h"
//
// @interface SceneDelegate ()
//
// @end
//
// @implementation SceneDelegate
//
//
// - (void)scene:(UIScene *)scene willConnectToSession:(UISceneSession *)session options:(UISceneConnectionOptions *)connectionOptions {
#[allow(dead_code)]
extern "C" fn scene_will_connect_to_session_options(_self: &Object, _sel: Sel, _scene: id, _session: id, _connection_options: id) {
    // // Use this method to optionally configure and attach the UIWindow `window` to the provided UIWindowScene `scene`.
    // // If using a storyboard, the `window` property will automatically be initialized and attached to the scene.
    // // This delegate does not imply the connecting scene or session are new (see `application:configurationForConnectingSceneSession` instead).
    debug_log("In scene will connect to session options");
}
// }
// TOD: from UIWindowSceneDelegate
// - (void)windowScene:(UIWindowScene *)windowScene
// didUpdateCoordinateSpace:(id<UICoordinateSpace>)previousCoordinateSpace
// interfaceOrientation:(UIInterfaceOrientation)previousInterfaceOrientation
//     traitCollection:(UITraitCollection *)previousTraitCollection;
extern "C" fn window_scene_did_update_coordinate_space_interface_orientation_trait_collection(
    _self: &mut Object, _sel: Sel,
    _window_scene: id,
    _previous_coordinate_space: id,
    _previous_interface_orientation: id,
    _previous_trait_collection: id,
) {
    debug_log("in window scene did update coordinate space etc");
    let windows:id = unsafe { msg_send![_window_scene, windows] };
    let count:NSUInteger = unsafe { msg_send![windows, count] };
    let msg = format!("window scene has {} windows", count);
    debug_log(msg.as_str());
}
//
//
// - (void)sceneDidDisconnect:(UIScene *)scene {
#[allow(dead_code)]
extern "C" fn scene_did_disconnect(_self: &Object, _sel: Sel, _scene: id) {
    // // Called as the scene is being released by the system.
    // // This occurs shortly after the scene enters the background, or when its session is discarded.
    // // Release any resources associated with this scene that can be re-created the next time the scene connects.
    // // The scene may re-connect later, as its session was not neccessarily discarded (see `application:didDiscardSceneSessions` instead).
}
// }
//
//
// - (void)sceneDidBecomeActive:(UIScene *)scene {
#[allow(dead_code)]
extern "C" fn scene_did_become_active(_self: &Object, _sel: Sel, _scene: id) {
    // // Called when the scene has moved from an inactive state to an active state.
    // // Use this method to restart any tasks that were paused (or not yet started) when the scene was inactive.
}
// }
//
//
// - (void)sceneWillResignActive:(UIScene *)scene {
#[allow(dead_code)]
extern "C" fn scene_will_resign_active(_self: &Object, _sel: Sel, _scene: id) {
    // // Called when the scene will move from an active state to an inactive state.
    // // This may occur due to temporary interruptions (ex. an incoming phone call).
}
// }
//
//
// - (void)sceneWillEnterForeground:(UIScene *)scene {
#[allow(dead_code)]
extern "C" fn scene_will_enter_foreground(_self: &Object, _sel: Sel, _scene: id) {
    // // Called as the scene transitions from the background to the foreground.
    // // Use this method to undo the changes made on entering the background.
}
// }
//
//
// - (void)sceneDidEnterBackground:(UIScene *)scene {
#[allow(dead_code)]
extern "C" fn scene_did_enter_background(_self: &Object, _sel: Sel, _scene: id) {
    // // Called as the scene transitions from the foreground to the background.
    // // Use this method to save data, release shared resources, and store enough scene-specific state information
    // // to restore the scene back to its current state.
}
// }
//
//
// @end
#[allow(dead_code)]
extern "C" fn scene_open_url_contexts(_self: &Object, _sel: Sel, _scene: id, _url_contexts: id) {}
#[allow(dead_code)]
extern "C" fn scene_will_continue_user_activity_with_type(_self: &Object, _sel: Sel, _scene: id, _type: id) {}
#[allow(dead_code)]
extern "C" fn scene_continue_user_activity(_self: &Object, _sel: Sel, _scene: id, _activity: id){}
#[allow(dead_code)]
extern "C" fn scene_did_fail_to_continue_user_activity_with_type_error(_self: &Object, _sel: Sel, _scene: id, _activity: id, _error: id) {}
#[allow(dead_code)]
extern "C" fn state_restoration_activity_for_scene(_self: &Object, _sel: Sel, _scene: id) -> id{ nil }
#[allow(dead_code)]
extern "C" fn scene_did_update_user_activity(_self: &Object, _sel: Sel, _scene: id, _activity: id) {}

fn get_rust_scene_delegate(_self: &Object) -> LockResult<RwLockReadGuard<'_, SceneDelegateRust>> {
    unsafe {
        let view_controller_ptr_ptr = _self.get_ivar::<SceneDelegatePtr>("rustSceneDelegate");
        if view_controller_ptr_ptr.view_controller_ptr.is_null() {
            view_controller_ptr_ptr.init() // will point a pointer in.
        }
        view_controller_ptr_ptr.read()
    }

}
fn get_mut_rust_scene_delegate(_self: &mut Object) -> LockResult<RwLockWriteGuard<'_, SceneDelegateRust>> {
    unsafe {
        let view_controller_ptr_ptr = _self.get_mut_ivar::<SceneDelegatePtr>("rustSceneDelegate");
        if view_controller_ptr_ptr.view_controller_ptr.is_null() {
            view_controller_ptr_ptr.init() // will point a pointer in.
        }
        view_controller_ptr_ptr.write()
    }
}

#[allow(dead_code)]
extern "C" fn get_window(_self: &Object, _sel: Sel) -> id {
    debug_log("In Scene Delegate get window.");
    get_rust_scene_delegate(_self).unwrap().window
}

#[allow(dead_code)]
extern "C" fn set_window(_self: &mut Object, _sel: Sel, value: id) {
    debug_log("In Scene Delegate set window.");
    get_mut_rust_scene_delegate(_self).unwrap().window = unsafe { objc_retain(value) };
}

#[allow(dead_code)]
extern "C" fn dealloc(_self: &mut Object, _sel: Sel) {
    // This method should free the objects allocated when initializing the ViewControllerPtr
    unsafe {
        let view_controller_ptr_ptr = _self.get_mut_ivar::<SceneDelegatePtr>("rustSceneDelegate");
        if !view_controller_ptr_ptr.view_controller_ptr.is_null() {
            view_controller_ptr_ptr.drop() // let everything go
        }
    }

}
