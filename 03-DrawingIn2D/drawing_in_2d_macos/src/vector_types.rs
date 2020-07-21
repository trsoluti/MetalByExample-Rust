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
use std::ops::Mul;

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
    /// Returns the dot product of this vector_float4 with another
    pub fn dot_product(self, other:vector_float4) -> f32 {
        let (ax, ay, az, aw) = self.to_tuple();
        let (bx, by, bz, bw) = other.to_tuple();
        ax * bx + ay * by + az * bz + aw * bw
    }
    fn to_tuple(self) -> (f32, f32, f32, f32) {
        (self.x(), self.y(), self.z(), self.w())
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
impl Mul<f32> for vector_float4 {
    type Output = vector_float4;

    fn mul(self, rhs: f32) -> Self::Output {
        let (x, y, z, w) = self.to_tuple();
        Self::Output::new(x * rhs, y * rhs, z * rhs, w * rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::vector_float4;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_dot_product() {
        let v1 = vector_float4::new(1.,2.,3.,4.);
        let v2 = vector_float4::new(5.,6.,7.,8.);
        assert_eq!(70., v1.dot_product(v2))
    }
}
