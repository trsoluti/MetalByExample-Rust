//
//  vector_types.rs
//
//  Created by TR Solutions on 2020-07-13.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//

//! Some vector types that provide simdd support

use std::convert::TryInto;
use objc::{Encode, Encoding};
use std::fmt::{Display, Formatter};

/// A simd vector of 2 unsigned integers
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct vector_uint2 {
    _private: u64,
}

impl vector_uint2 {
    /// Creates a new simd vector from x and y values.
    #[allow(dead_code)]
    pub fn new(x: u32, y: u32) -> Self {
        vector_uint2 {
            _private: x as u64 + ((y as u64) << 32)
        }
    }
    /// Returns the x value of the vector.
    pub fn x(self) -> u32 {
        (self._private & 0xffffffff).try_into().unwrap()
    }
    /// Returns the y value of the vector.
    pub fn y(self) -> u32 {
        ((self._private >> 32) & 0xffffffff).try_into().unwrap()
    }
}

unsafe impl Encode for vector_uint2 {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("d") }
    }
}

impl Display for vector_uint2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x(), self.y())
    }
}

/// A simd vector of 2 floating-point values.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct vector_float2{
    _private: [f32; 4]
}

impl vector_float2 {
    /// Creates a simd vector from the given x and y values.
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32) -> Self {
        vector_float2 {
            _private: [ x, y, 0., 0.]
        }
    }
    /// Returns the x value of the vector.
    pub fn x(self) -> f32 {
        self._private[0]
    }
    /// Returns the y value of the vector.
    pub fn y(self) -> f32 {
        self._private[1]
    }
}

unsafe impl Encode for vector_float2 {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("ff") }
    }
}

impl Display for vector_float2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x(), self.y())
    }
}


/// A simd vector of 4 floating-point values.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct vector_float4 {
    _private: [f32; 4],
}

impl vector_float4 {
    /// Creates a new simd vector from the given x, y, z and w values
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        vector_float4 {
            _private: [ x, y, z, w]
        }
    }
    /// Returns the x value of the vector.
    pub fn x(self) -> f32 {
        self._private[0]
    }
    /// Returns the y value of the vector.
    pub fn y(self) -> f32 {
        self._private[1]
    }
    /// Returns the z value of the vector.
    pub fn z(self) -> f32 {
        self._private[2]
    }
    /// Returns the w value of the vector.
    pub fn w(self) -> f32 {
        self._private[3]
    }
}


unsafe impl Encode for vector_float4 {
    fn encode() -> Encoding {
        unsafe { Encoding::from_str("ffff") }
    }
}

impl Display for vector_float4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{},{})", self.x(), self.y(), self.z(), self.w())
    }
}
