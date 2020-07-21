//
//  dispatch.rs
//
//  Created by TR Solutions on 2020-07-18.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for the objects and methods related to dispatch
//!
//! For now we bundle them with core_animation, but in the future
//! this may be broken off into a separate library

mod dispatch_semaphore;

pub use dispatch_semaphore::DispatchSemaphore;
pub use dispatch_semaphore::dispatch_time_t;
pub use dispatch_semaphore::DISPATCH_TIME_NOW;
pub use dispatch_semaphore::DISPATCH_TIME_FOREVER;
