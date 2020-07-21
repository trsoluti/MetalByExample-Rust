//
//  metal_depth_stencil_state.rs
//
//  Created by TR Solutions on 2020-07-18.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
/// A simple Rust wrapper around MTLDepthStencilState


use cocoa::base::{id, nil};
use objc::runtime::{objc_retain, objc_release};

//  metal_depth_stencil_state.rs
//
//  Created by TR Solutions on 2020-07-18.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
/// A simple Rust wrapper around MTLDepthStencilState
/// Rust wrapper around an object that configures new MTLDepthStencilState objects.
pub struct MetalDepthStencilState {
    state: id,
}
impl Default for MetalDepthStencilState {
    fn default() -> Self { MetalDepthStencilState { state: nil } }
}
impl From<id> for MetalDepthStencilState {
    fn from(state: id) -> Self {
        let state = unsafe { objc_retain(state) };
        MetalDepthStencilState { state }
    }
}
impl Drop for MetalDepthStencilState {
    fn drop(&mut self) { unsafe { objc_release(self.state) } }
}
impl MetalDepthStencilState {
    /// Returns the underlying Objective C object.
    pub fn to_objc(&self) -> id { self.state }
}