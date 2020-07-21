//
//  dispatch_semaphor.rs
//
//  Created by TR Solutions on 2020-07-18.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for dispatch_semaphore methods

use cocoa::base::{id, nil};
use std::os::raw::{c_long};
use objc::runtime::{objc_retain, objc_release};

#[allow(non_camel_case_types)]
type dispatch_semaphore_t = id;
// From usr/include/dispatch/time.h:
// /*!
//  * @typedef dispatch_time_t
//  *
//  * @abstract
//  * A somewhat abstract representation of time; where zero means "now" and
//  * DISPATCH_TIME_FOREVER means "infinity" and every value in between is an
//  * opaque encoding.
//  */
// typedef uint64_t dispatch_time_t;
/// A somewhat abstract representation of time; where zero means "now" and
/// DISPATCH_TIME_FOREVER means "infinity" and every value in between is an
/// opaque encoding.
#[allow(non_camel_case_types)]
pub type dispatch_time_t = u64;
/// Dispatch time of 'now'
pub const DISPATCH_TIME_NOW: dispatch_time_t = 0;
/// Dispatch time of 'forever'
pub const DISPATCH_TIME_FOREVER: dispatch_time_t = !0;

extern {
    // /*!
    //  * @function dispatch_semaphore_create
    //  *
    //  * @abstract
    //  * Creates new counting semaphore with an initial value.
    //  *
    //  * @discussion
    //  * Passing zero for the value is useful for when two threads need to reconcile
    //  * the completion of a particular event. Passing a value greater than zero is
    //  * useful for managing a finite pool of resources, where the pool size is equal
    //  * to the value.
    //  *
    //  * @param value
    //  * The starting value for the semaphore. Passing a value less than zero will
    //  * cause NULL to be returned.
    //  *
    //  * @result
    //  * The newly created semaphore, or NULL on failure.
    //  */
    // API_AVAILABLE(macos(10.6), ios(4.0))
    // DISPATCH_EXPORT DISPATCH_MALLOC DISPATCH_RETURNS_RETAINED DISPATCH_WARN_RESULT
    // DISPATCH_NOTHROW
    // dispatch_semaphore_t
    // dispatch_semaphore_create(long value);
    fn dispatch_semaphore_create(value: c_long) -> dispatch_semaphore_t;
    // /*!
    //  * @function dispatch_semaphore_wait
    //  *
    //  * @abstract
    //  * Wait (decrement) for a semaphore.
    //  *
    //  * @discussion
    //  * Decrement the counting semaphore. If the resulting value is less than zero,
    //  * this function waits for a signal to occur before returning.
    //  *
    //  * @param dsema
    //  * The semaphore. The result of passing NULL in this parameter is undefined.
    //  *
    //  * @param timeout
    //  * When to timeout (see dispatch_time). As a convenience, there are the
    //  * DISPATCH_TIME_NOW and DISPATCH_TIME_FOREVER constants.
    //  *
    //  * @result
    //  * Returns zero on success, or non-zero if the timeout occurred.
    //  */
    // API_AVAILABLE(macos(10.6), ios(4.0))
    // DISPATCH_EXPORT DISPATCH_NONNULL_ALL DISPATCH_NOTHROW
    // long
    // dispatch_semaphore_wait(dispatch_semaphore_t dsema, dispatch_time_t timeout);
    fn dispatch_semaphore_wait(dsema: dispatch_semaphore_t, timeout: dispatch_time_t ) -> c_long;
    // /*!
    //  * @function dispatch_semaphore_signal
    //  *
    //  * @abstract
    //  * Signal (increment) a semaphore.
    //  *
    //  * @discussion
    //  * Increment the counting semaphore. If the previous value was less than zero,
    //  * this function wakes a waiting thread before returning.
    //  *
    //  * @param dsema The counting semaphore.
    //  * The result of passing NULL in this parameter is undefined.
    //  *
    //  * @result
    //  * This function returns non-zero if a thread is woken. Otherwise, zero is
    //  * returned.
    //  */
    // API_AVAILABLE(macos(10.6), ios(4.0))
    // DISPATCH_EXPORT DISPATCH_NONNULL_ALL DISPATCH_NOTHROW
    // long
    // dispatch_semaphore_signal(dispatch_semaphore_t dsema);
    fn dispatch_semaphore_signal(dsema: dispatch_semaphore_t) -> c_long;
}


/// A Rust wrapper for a counting semaphore
#[derive(Debug)]
pub struct DispatchSemaphore {
    semaphore: id
}
impl Default for DispatchSemaphore {
    fn default() -> Self { DispatchSemaphore { semaphore: nil } }
}
impl From<dispatch_semaphore_t> for DispatchSemaphore {
    fn from(semaphore: dispatch_semaphore_t) -> Self {
        let semaphore = unsafe { objc_retain(semaphore) };
        DispatchSemaphore { semaphore }
    }
}
impl Drop for DispatchSemaphore {
    fn drop(&mut self) { unsafe { objc_release(self.semaphore) } }
}
impl DispatchSemaphore {
    /// Creates new counting semaphore with an initial value.
    #[inline]
    pub fn create(value: c_long) -> Self {
        let semaphore:id = unsafe { dispatch_semaphore_create(value) };
        DispatchSemaphore::from(semaphore)
    }
    /// Waits (decrements) for a semaphore.
    #[inline]
    pub fn wait(&self, timeout: dispatch_time_t) -> c_long {
        unsafe { dispatch_semaphore_wait(self.semaphore, timeout ) }
    }
    /// Signals (increments) a semaphore.
    #[inline]
    pub fn signal(&self) -> c_long {
        unsafe { dispatch_semaphore_signal(self.semaphore) }
    }
}
