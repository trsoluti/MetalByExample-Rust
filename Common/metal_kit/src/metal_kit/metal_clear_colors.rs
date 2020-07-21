//
//  metal_clear_color.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for MTLClearColor struct and associated data

use std::os::raw::c_double;
use objc::{Encode, Encoding};

// From Metal.framework/Versions/A/Headers/MTLRenderPass.h
// typedef struct
// {
//     double red;
//     double green;
//     double blue;
//     double alpha;
// } MTLClearColor;
/// An RGBA value used for a color pixel.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MetalClearColor {
    red: c_double,
    green: c_double,
    blue: c_double,
    alpha: c_double,
}
impl Default for MetalClearColor {
    fn default() -> Self {
        MetalClearColor {
            red: 0.,
            green: 0.,
            blue: 0.,
            alpha: 0.,
        }
    }
}
unsafe impl Encode for MetalClearColor {
    fn encode() -> Encoding {
        let d:String = f64::encode().as_str().parse().unwrap();
        unsafe { Encoding::from_str(format!("{{?={}{}{}{}}}",d,d,d,d).as_str()) }
    }
}
// MTL_INLINE MTLClearColor MTLClearColorMake(double red, double green, double blue, double alpha);
impl MetalClearColor {
    /// Returns a value used to clear a color attachment
    /// (in effect, when the loadAction property of MTLRenderPassAttachmentDescriptor
    /// is MTLLoadActionClear).
    ///
    /// Equivalent to Objective C MTLClearColorMake()
    pub fn make(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        MetalClearColor {
            red,
            green,
            blue,
            alpha
        }
    }
}
