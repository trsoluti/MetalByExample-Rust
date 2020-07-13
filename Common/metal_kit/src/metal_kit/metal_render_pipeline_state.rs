//
//  metal_render_pipeline_state.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for methods we use from MTLRenderPipelineState

use cocoa::base::{id, nil};
use objc::runtime::{objc_release, objc_retain};

/// Rust wrapper for an object that contains
/// the graphics functions and configuration state
/// used in a render pass.
pub struct MetalRenderPipelineState {
    state: id,
}
impl Default for MetalRenderPipelineState {
    fn default() -> Self {
        MetalRenderPipelineState { state: nil }
    }
}
impl From<id> for MetalRenderPipelineState {
    fn from(state: id) -> Self {
        let state = unsafe { objc_retain(state) };
        MetalRenderPipelineState { state }
    }
}
impl Drop for MetalRenderPipelineState {
    fn drop(&mut self) { unsafe { objc_release(self.state) } }
}

impl MetalRenderPipelineState {
    /// Returns the underlying objective c pipeline state
    pub fn to_objc(&self) -> id { self.state }
}
