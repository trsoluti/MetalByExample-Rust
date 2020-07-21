//
//  generic_id.rs
//
//  Created by TR Solutions on 2020-07-18.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
/// A simple Rust wrapper around id for items
/// where we don't need any functions
/// but we do need Default and Drop

use cocoa::base::{id, nil};
use objc::runtime::{objc_retain, objc_release};

pub struct GenericId {
    id: id,
}
impl Default for GenericId {
    fn default() -> Self { GenericId { id: nil } }
}
impl From<id> for GenericId {
    fn from(id: id) -> Self {
        let id = unsafe { objc_retain(id) };
        GenericId { id }
    }
}
impl Drop for GenericId {
    fn drop(&mut self) { unsafe { objc_release(self.id) } }
}
impl GenericId {
    /// Returns the underlying Objective C object.
    pub fn to_objc(&self) -> id { self.id }
}