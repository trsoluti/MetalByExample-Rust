//
//  vector_types.rs
//
//  Created by TR Solutions on 2020-07-13.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//

//! Some vector types that provide simdd support.

use std::convert::TryInto;
use objc::{Encode, Encoding};
use std::fmt::{Display, Formatter, Debug};
use std::ops::Mul;

// Use the Apple shared library instead of bringing in rust's:
extern {
    fn sinf(value: f32) -> f32;
    fn cosf(value: f32) -> f32;
}
/// Return the sin of the given value in radians
#[inline]
fn sinf32(value: f32) -> f32 { unsafe { sinf(value) } }
/// Return the cosine of the given value in radians
#[inline]
fn cosf32(value: f32) -> f32 { unsafe { cosf(value) } }

/// Simd vector of 2 unsigned integers.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default)]
pub struct vector_uint2 {
    _private: u64,
}

impl vector_uint2 {
    /// Creates a new simd vector from given integer x and y values.
    #[allow(dead_code)]
    pub fn new(x: u32, y: u32) -> Self {
        vector_uint2 {
            _private: x as u64 + ((y as u64) << 32)
        }
    }
    /// Returns the x portion of the vector.
    #[inline]
    pub fn x(self) -> u32 {
        (self._private & 0xffffffff).try_into().unwrap()
    }
    /// Returns the y portion of the vector.
    #[inline]
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

/// Simd vector of two 32-bit floats.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Default)]
pub struct vector_float2{
    _private: [f32; 4]
}

impl vector_float2 {
    /// Creates a new Simd vector from given float x & y values.
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32) -> Self {
        vector_float2 {
            _private: [ x, y, 0., 0.]
        }
    }
    /// Returns the x portion of the vector.
    #[inline]
    pub fn x(self) -> f32 {
        self._private[0]
    }
    /// Returns the y portion of the vector.
    #[inline]
    pub fn y(self) -> f32 {
        self._private[1]
    }
    /// Returns a vector where each item in the vector
    /// is the sin of the value in the current vector
    pub fn sin(self) -> Self {
        // Note: in the future, this should be a simd
        // instruction
        vector_float2 {
            _private: [
                sinf32(self._private[1]),
                sinf32(self._private[2]),
                0.,
                0.,
            ]
        }
    }
    /// Returns a vector where each item in the vector
    /// is the sin of the value in the current vector
    pub fn cos(self) -> Self {
        // Note: in the future, this should be a simd
        // instruction
        vector_float2 {
            _private: [
                cosf32(self._private[1]),
                cosf32(self._private[2]),
                0.,
                0.,
            ]
        }
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

/// Simd vector of 3 32-bit floats
///
/// Note: stored internally as a vector of 4
// @abstract A vector of three 32-bit floating-point numbers.
//  *  @description In C++ and Metal, this type is also available as
//  *  simd::float3. Note that vectors of this type are padded to have the same
//  *  size and alignment as simd_float4.
// typedef __attribute__((__ext_vector_type__(3))) float simd_float3;
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Default)]
pub struct vector_float3 {
    _private: [f32; 4],
}
impl From<[f32;3]> for vector_float3 {
    fn from(values:[f32;3]) -> Self {
        vector_float3 {
            _private: [values[0], values[1], values[2], 0. ]
        }
    }
}
impl vector_float3 {
    /// Returns the x portion of the vector.
    #[inline]
    pub fn x(self) -> f32 {
        self._private[0]
    }
    /// Returns the y portion of the vector.
    #[inline]
    pub fn y(self) -> f32 {
        self._private[1]
    }
    /// Returns the z portion of the vector.
    #[inline]
    pub fn z(self) -> f32 {
        self._private[2]
    }
}

/// Simd vector of 4 32-bit floats
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default)]
pub struct vector_float4 {
    _private: [f32; 4],
}
impl From<[f32;4]> for vector_float4 {
    fn from(values: [f32;4]) -> Self { vector_float4 { _private: values } }
}
impl Debug for vector_float4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x,y,z,w) = self.to_tuple();
        f.write_fmt(format_args!("({:.6}, {:.6}, {:.6}, {:.6})",
        x, y, z, w))
    }
}

impl vector_float4 {
    /// Creates a new Simd vector from given float x, y, z and w values.
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        vector_float4 {
            _private: [ x, y, z, w]
        }
    }
    /// Returns the x portion of the vector.
    #[inline]
    pub fn x(self) -> f32 {
        self._private[0]
    }
    /// Returns the y portion of the vector.
    #[inline]
    pub fn y(self) -> f32 {
        self._private[1]
    }
    /// Returns the z portion of the vector.
    #[inline]
    pub fn z(self) -> f32 {
        self._private[2]
    }
    /// Returns the w portion of the vector.
    #[inline]
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
// impl Into<(f32, f32, f32, f32)> for vector_float4 {
//     fn into(self) -> (f32, f32, f32, f32) {
//         (self.x(), self.y(), self.z(), self.w())
//     }
// }
impl From<vector_float4> for (f32, f32, f32, f32) {
    fn from(vector: vector_float4) -> Self {
        (vector.x(), vector.y(), vector.z(), vector.w())
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

