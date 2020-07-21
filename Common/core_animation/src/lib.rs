//
//  lib.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
#![allow(dead_code)]
#![deny(missing_docs)]
//! Thin wrappers for the classes and methods we use from Core Animation

mod core_animation;
// Look in the core animation mod to see what's exported
pub use core_animation::*;

mod dispatch;
pub use dispatch::*;

use std::os::raw::c_int;
use std::fmt::{Display, Formatter};
use std::error::Error;

/// The rust error returned if the system failed
/// during core anim methods
#[derive(Debug)]
pub enum CoreAnimError {
    /// Attempt to connect to the timer failed.
    FailedToCreateTimer(c_int),
    /// Attempt to link to the output handler failed.
    FailedToLinkOutputHandler(c_int),
    /// Attempt to connect to the main display failed.
    FailedToConnectToDisplay(c_int),
}
impl Display for CoreAnimError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Core Anim error")
    }
}
impl Error for CoreAnimError {}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
