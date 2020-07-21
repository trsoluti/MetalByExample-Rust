//
//  matrix_types.rs
//
//  Original objc code Copyright (c) 2015 Warren Moore
//  from https://github.com/metal-by-example/sample-code
//  Licensed under MIT.
//
//  Translated to Rust by TR Solutions on 18/7/20.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//
//  See appropriate LICENCE files for details.
//
// /*
// //
// //  MBEMathUtilities.h
// //  DrawingIn3DIOS
// //
// //  Original Copyright (c) 2015 Warren Moore
// //  from https://github.com/metal-by-example/sample-code
// //  Licensed under MIT.
// //
// //  Updates by TR Solutions on 18/7/20.
// //  Copyright © 2020 TR Solutions Pte. Ltd.
// //  Licensed under Apache 2.0 and MIT
// //
// //  See appropriate LICENCE files for details.
// //
// @import simd;
//
//! Some vector types that provide simdd support.
//!
//! From Apple doc in usr/include/simd/matrix_types.h:
//!
//! For compatibility with common graphics libraries, these matrices are stored
//! in column-major order, and implemented as arrays of column vectors.
//! Column-major storage order may seem a little strange if you aren't used to
//! it, but for most usage the memory layout of the matrices shouldn't matter
//! at all; instead you should think of matrices as abstract mathematical
//! objects that you use to perform arithmetic without worrying about the
//! details of the underlying representation.

use crate::{vector_float4, vector_float3};
use std::ops::Mul;
use std::fmt::{Debug, Formatter};

// Use the Apple shared library instead of bringing in rust's:
extern {
    fn sinf(value: f32) -> f32;
    fn cosf(value: f32) -> f32;
    fn tanf(value: f32) -> f32;
}
/// Return the sin of the given value in radians
#[inline]
fn sinf32(value: f32) -> f32 { unsafe { sinf(value) } }
/// Return the cosine of the given value in radians
#[inline]
fn cosf32(value: f32) -> f32 { unsafe { cosf(value) } }
/// Return the tangent of the given value in radians
#[inline]
fn tanf32(value: f32) -> f32 { unsafe { tanf(value) } }


/// Provides a 4x4 float32 vector
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default)]
pub struct matrix_float4x4 {
    _private: [vector_float4; 4]
}
impl Debug for matrix_float4x4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (x, y, z, w) = self.to_tuple();
        f.write_fmt(format_args!("{{ matrix_4x4:\nx: {:?}\ny: {:?}\nz: {:?}\nw: {:?}}}", x, y, z, w))
    }
}

impl matrix_float4x4 {
    /// Creates a new 4x4 matrix from the given values.
    #[inline]
    pub fn new(x: vector_float4, y: vector_float4, z: vector_float4, w: vector_float4) -> matrix_float4x4 {
        matrix_float4x4 { _private: [x, y, z, w] }
    }
    // /// Builds a translation matrix that translates by the supplied vector
    // matrix_float4x4 matrix_float4x4_translation(vector_float3 t);
    // matrix_float4x4 matrix_float4x4_translation(vector_float3 t)
    // {
    /// Builds a translation matrix that translates by the supplied vector.
    pub fn translation(translation: vector_float3) -> Self {
        // vector_float4 X = { 1, 0, 0, 0 };
        // vector_float4 Y = { 0, 1, 0, 0 };
        // vector_float4 Z = { 0, 0, 1, 0 };
        // vector_float4 W = { t.x, t.y, t.z, 1 };
        //
        // matrix_float4x4 mat = { X, Y, Z, W };
        // return mat;
        let x = vector_float4::new(1., 0., 0., 0.);
        let y = vector_float4::new(0., 1., 0., 0.);
        let z = vector_float4::new(0., 0., 1., 0.);
        let w = vector_float4::new(
            translation.x(),
            translation.y(),
            translation.z(),
            1.
        );
        matrix_float4x4::new(x,y,z,w)
    }
    // }
    //
    //
    // /// Builds a scale matrix that uniformly scales all axes by the supplied factor
    // matrix_float4x4 matrix_float4x4_uniform_scale(float scale);
    // matrix_float4x4 matrix_float4x4_uniform_scale(float scale)
    // {
    /// Builds a scale matrix that uniformly scales all axes by the supplied factor.
    pub fn uniform_scale(scale: f32) -> matrix_float4x4 {
        // vector_float4 X = { scale, 0, 0, 0 };
        // vector_float4 Y = { 0, scale, 0, 0 };
        // vector_float4 Z = { 0, 0, scale, 0 };
        // vector_float4 W = { 0, 0, 0, 1 };
        let x = vector_float4::new(scale, 0., 0., 0.);
        let y = vector_float4::new(0., scale, 0., 0.);
        let z = vector_float4::new(0., 0., scale, 0.);
        let w = vector_float4::new(0., 0., 0., 1.);
        //
        // matrix_float4x4 mat = { X, Y, Z, W };
        // return mat;
        matrix_float4x4::new(x, y, z, w)
    }
    // }
    //
    // /// Builds a rotation matrix that rotates about the supplied axis by an
    // /// angle (given in radians). The axis should be normalized.
    // matrix_float4x4 matrix_float4x4_rotation(vector_float3 axis, float angle);
    /// Builds a rotation matrix that rotates about the supplied axis
    /// by an angle (given in radians). The axis should be normalized.
    // matrix_float4x4 matrix_float4x4_rotation(vector_float3 axis, float angle)
    // {
    pub fn rotation(axis: vector_float3, angle: f32) -> Self {
        // float c = cos(angle);
        // float s = sin(angle);
        let c = cosf32(angle);
        let s = sinf32(angle);
        //
        // vector_float4 X;
        // X.x = axis.x * axis.x + (1 - axis.x * axis.x) * c;
        // X.y = axis.x * axis.y * (1 - c) - axis.z * s;
        // X.z = axis.x * axis.z * (1 - c) + axis.y * s;
        // X.w = 0.0;
        let x = vector_float4::new(
            axis.x() * axis.x() + ( 1. - axis.x() * axis.x() ) * c,
            axis.x() * axis.y() * (1. - c) - axis.z() * s,
            axis.x() * axis.z() * (1. - c) + axis.y() * s,
            0.,
        );
        //
        // vector_float4 Y;
        // Y.x = axis.x * axis.y * (1 - c) + axis.z * s;
        // Y.y = axis.y * axis.y + (1 - axis.y * axis.y) * c;
        // Y.z = axis.y * axis.z * (1 - c) - axis.x * s;
        // Y.w = 0.0;
        let y = vector_float4::new (
            axis.x() * axis.y() * (1. - c) + axis.z() * s,
            axis.y() * axis.y() + (1. - axis.y() * axis.y()) * c,
            axis.y() * axis.z() * (1. - c) - axis.x() * s,
            0.0,
        );
        //
        // vector_float4 Z;
        // Z.x = axis.x * axis.z * (1 - c) - axis.y * s;
        // Z.y = axis.y * axis.z * (1 - c) + axis.x * s;
        // Z.z = axis.z * axis.z + (1 - axis.z * axis.z) * c;
        // Z.w = 0.0;
        let z = vector_float4::new (
            axis.x() * axis.z() * (1. - c) - axis.y() * s,
            axis.y() * axis.z() * (1. - c) + axis.x() * s,
            axis.z() * axis.z() + (1. - axis.z() * axis.z()) * c,
            0.
        );
        //
        // vector_float4 W;
        // W.x = 0.0;
        // W.y = 0.0;
        // W.z = 0.0;
        // W.w = 1.0;
        let w = vector_float4::new(
            0.,
            0.,
            0.,
            1.,
        );
        //
        // matrix_float4x4 mat = { X, Y, Z, W };
        // return mat;
        matrix_float4x4::new(x, y, z, w)
    }
    // }
    //
    // /// Builds a symmetric perspective projection matrix with the supplied aspect ratio,
    // /// vertical field of view (in radians), and near and far distances
    // matrix_float4x4 matrix_float4x4_perspective(float aspect, float fovy, float near, float far);
    // matrix_float4x4 matrix_float4x4_perspective(float aspect, float fovy, float near, float far)
    // {
    /// Builds a symmetric perspective projection matrix with the supplied aspect ratio,
    /// vertical field of view (in radians), and near and far distances.
    pub fn perspective(aspect: f32, fovy: f32, near: f32, far: f32) -> Self {
        // float yScale = 1 / tan(fovy * 0.5);
        // float xScale = yScale / aspect;
        // float zRange = far - near;
        // float zScale = -(far + near) / zRange;
        // float wzScale = -2 * far * near / zRange;
        let y_scale = 1. / tanf32(fovy * 0.5);
        let x_scale = y_scale / aspect;
        let z_range = far - near;
        let z_scale = -(far - near) / z_range;
        let wz_scale = -2. * far * near / z_range;
        //
        // vector_float4 P = { xScale, 0, 0, 0 };
        // vector_float4 Q = { 0, yScale, 0, 0 };
        // vector_float4 R = { 0, 0, zScale, -1 };
        // vector_float4 S = { 0, 0, wzScale, 0 };
        //
        // matrix_float4x4 mat = { P, Q, R, S };
        // return mat;
        matrix_float4x4::new(
            vector_float4::new(x_scale, 0., 0., 0.),
            vector_float4::new(0., y_scale, 0., 0.),
            vector_float4::new(0., 0., z_scale, -1.),
            vector_float4::new(0., 0., wz_scale, 0.)
        )
    }
    // }
    //
    fn to_tuple(self) -> (vector_float4, vector_float4, vector_float4, vector_float4) {
        ( self._private[0], self._private[1], self._private[2], self._private[3])
    }
    fn transpose(self) -> Self {
        let (vx, vy, vz, vw) = self.to_tuple();
        matrix_float4x4::new(
            vector_float4::new(vx.x(), vy.x(), vz.x(), vw.x()),
            vector_float4::new(vx.y(), vy.y(), vz.y(), vw.y()),
            vector_float4::new(vx.z(), vy.z(), vz.z(), vw.z()),
            vector_float4::new(vx.w(), vy.w(), vz.w(), vw.w()),
        )
    }
}

//
// ============================================================================================
// //  MBEMathUtilities.m
// //  DrawingIn3DIOS
// //
// //  Original Copyright (c) 2015 Warren Moore
// //  from https://github.com/metal-by-example/sample-code
// //  Licensed under MIT.
// //
// //  Updates by TR Solutions on 18/7/20.
// //  Copyright © 2020 TR Solutions Pte. Ltd.
// //  Licensed under Apache 2.0 and MIT
// //
// //  See appropriate LICENCE files for details.
// //
// #import "MBEMathUtilities.h"
//
//
//  */

impl Mul<f32> for matrix_float4x4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        // This really should be in simd!
        let (x, y, z, w) = self.to_tuple();
        Self::Output::new(x * rhs, y * rhs, z * rhs, w *rhs)
    }
}
impl Mul<matrix_float4x4> for matrix_float4x4 {
    type Output = Self;

    fn mul(self, rhs: matrix_float4x4) -> Self::Output {
        let (tx, ty, tz, tw) = self.transpose().to_tuple();
        let (rx, ry, rz, rw) = rhs.to_tuple();
        matrix_float4x4::new(
            vector_float4::new(
                tx.dot_product(rx),
                ty.dot_product(rx),
                tz.dot_product(rx),
                tw.dot_product(rx),
            ),
            vector_float4::new(
                tx.dot_product(ry),
                ty.dot_product(ry),
                tz.dot_product(ry),
                tw.dot_product(ry),
            ),
            vector_float4::new(
                tx.dot_product(rz),
                ty.dot_product(rz),
                tz.dot_product(rz),
                tw.dot_product(rz),
            ),
            vector_float4::new(
                tx.dot_product(rw),
                ty.dot_product(rw),
                tz.dot_product(rw),
                tw.dot_product(rw),
            )
        )
    }
}


#[cfg(test)]
mod tests {
    use crate::{vector_float4, matrix_float4x4};

    // fn mul_explicit(lhs: matrix_float4x4, rhs: matrix_float4x4) -> matrix_float4x4 {
    //     println!("Multiplying\n{:?}\nby\n{:?}", lhs, rhs);
    //     let (tx, ty, tz, tw) = lhs.transpose().to_tuple();
    //     let (rx, ry, rz, rw) = rhs.to_tuple();
    //     let axx = tx.dot_product(rx);
    //     println!("axx = {:?} dot {:?} = {:?}", tx, rx, axx);
    //     let axy = ty.dot_product(rx);
    //     println!("axx = {:?} dot {:?} = {:?}", ty, rx, axy);
    //     let axz = tz.dot_product(rx);
    //     println!("axx = {:?} dot {:?} = {:?}", tz, rx, axz);
    //     let axw = tw.dot_product(rx);
    //     println!("axx = {:?} dot {:?} = {:?}", tw, rx, axw);
    //     matrix_float4x4::new(
    //         vector_float4::new(
    //             tx.dot_product(rx),
    //             ty.dot_product(rx),
    //             tz.dot_product(rx),
    //             tw.dot_product(rx),
    //         ),
    //         vector_float4::new(
    //             tx.dot_product(ry),
    //             ty.dot_product(ry),
    //             tz.dot_product(ry),
    //             tw.dot_product(ry),
    //         ),
    //         vector_float4::new(
    //             tx.dot_product(rz),
    //             ty.dot_product(rz),
    //             tz.dot_product(rz),
    //             tw.dot_product(rz),
    //         ),
    //         vector_float4::new(
    //             tx.dot_product(rw),
    //             ty.dot_product(rw),
    //             tz.dot_product(rw),
    //             tw.dot_product(rw),
    //         )
    //     )
    // }


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_matrix_multiplication_scalar() {
        let m1 = matrix_float4x4::new(
            vector_float4::new(1., 2., 3., 4.),
            vector_float4::new(5., 6., 7., 8.),
            vector_float4::new(9., 10., 11., 12.),
            vector_float4::new(13., 14., 15., 16.)
        );
        let m2 = m1 * 0.5;

        assert_eq!(0.5, m2._private[0].x());
        assert_eq!(1.0, m2._private[0].y());
        assert_eq!(1.5, m2._private[0].z());
        assert_eq!(2.0, m2._private[0].w());

        assert_eq!(2.5, m2._private[1].x());
        assert_eq!(3.0, m2._private[1].y());
        assert_eq!(3.5, m2._private[1].z());
        assert_eq!(4.0, m2._private[1].w());

        assert_eq!(4.5, m2._private[2].x());
        assert_eq!(5.0, m2._private[2].y());
        assert_eq!(5.5, m2._private[2].z());
        assert_eq!(6.0, m2._private[2].w());

        assert_eq!(6.5, m2._private[3].x());
        assert_eq!(7.0, m2._private[3].y());
        assert_eq!(7.5, m2._private[3].z());
        assert_eq!(8.0, m2._private[3].w());
    }

    #[test]
    fn test_matrix_multiplication_matrix() {
        let m1 = matrix_float4x4::new(
            vector_float4::new(1., 2., 3., 4.),
            vector_float4::new(5., 6., 7., 8.),
            vector_float4::new(9., 10., 11., 12.),
            vector_float4::new(13., 14., 15., 16.)
        );
        let m2 = matrix_float4x4::new(
            vector_float4::new(17., 18., 19., 20.),
            vector_float4::new(21., 22., 23., 24.),
            vector_float4::new(25., 26., 27., 28.),
            vector_float4::new(29., 30., 31., 32.)
        );
        let m3 = m1 * m2;
        println!("m1 x m2 = {:?}", m3);
        // Answers found using online matrix multiplier:
        // https://www.symbolab.com/solver/matrix-multiply-calculator/%5Cbegin%7Bpmatrix%7D1%265%269%2613%5C%5C%20%20%202%266%2610%2614%5C%5C%20%20%203%267%2611%2615%5C%5C%20%20%204%268%2612%2616%5Cend%7Bpmatrix%7D%5Ccdot%5Cbegin%7Bpmatrix%7D17%2621%2625%2629%5C%5C%20%20%2018%2622%2626%2630%5C%5C%20%20%2019%2623%2627%2631%5C%5C%20%2020%2624%2628%2632%5Cend%7Bpmatrix%7D
        assert_eq!( 538., m3._private[0].x());
        assert_eq!( 650., m3._private[1].x());
        assert_eq!( 762., m3._private[2].x());
        assert_eq!( 874., m3._private[3].x());

        assert_eq!( 612., m3._private[0].y());
        assert_eq!( 740., m3._private[1].y());
        assert_eq!( 868., m3._private[2].y());
        assert_eq!( 996., m3._private[3].y());

        assert_eq!( 686., m3._private[0].z());
        assert_eq!( 830., m3._private[1].z());
        assert_eq!( 974., m3._private[2].z());
        assert_eq!(1118., m3._private[3].z());

        assert_eq!( 760., m3._private[0].w());
        assert_eq!( 920., m3._private[1].w());
        assert_eq!(1080., m3._private[2].w());
        assert_eq!(1240., m3._private[3].w());
    }

    #[test]
    fn test_matrix_rot_mult() {
        //                                                                           ------------------ x -----------------                                 ------------------ y -----------------                                 ------------------ z -----------------                                 ------------------ w -----------------
        // rust: x rot:      matrix_float4x4 { _private: [vector_float4 { _private: [1.000000, 0.000000, 0.000000, 0.000000] }, vector_float4 { _private: [0.000000, 0.998629, -0.052344, 0.000000] }, vector_float4 { _private: [ 0.000000, 0.052345, 0.998629, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
        // rust: y rot:      matrix_float4x4 { _private: [vector_float4 { _private: [0.999391, 0.000000, 0.034906, 0.000000] }, vector_float4 { _private: [0.000000, 1.000000,  0.000000, 0.000000] }, vector_float4 { _private: [-0.034905, 0.000000, 0.999391, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
        // objc: Rot Matrix: matrix_float4x4 { _private: [vector_float4 { _private: [0.999391, 0.001827, 0.034859, 0.000000] }, vector_float4 { _private: [0.000000, 0.998629, -0.052347, 0.000000] }, vector_float4 { _private: [-0.034907, 0.052315, 0.998020, 0.000000] }, vector_float4 { _private: [0.000000, 0.000000, 0.000000, 1.000000] }] }
        let x_rot = matrix_float4x4::new(
            vector_float4::new(1.000000, 0.000000, 0.000000, 0.000000),
            vector_float4::new(0.000000, 0.998629, -0.052344, 0.000000),
            vector_float4::new(0.000000, 0.052345, 0.998629, 0.000000),
            vector_float4::new(0.000000, 0.000000, 0.000000, 1.000000)
        );
        let y_rot = matrix_float4x4::new(
            vector_float4::new(0.999391, 0.000000, 0.034906, 0.000000),
            vector_float4::new(0.000000, 1.000000, 0.000000, 0.000000),
            vector_float4::new(-0.034905, 0.000000, 0.999391, 0.000000),
            vector_float4::new(0.000000, 0.000000, 0.000000, 1.000000)
        );
        let rot = x_rot * y_rot;

        let exp = matrix_float4x4::new(
            vector_float4::new(0.999391, 0.001827, 0.034859, 0.000000),
            vector_float4::new(0.000000, 0.998629, -0.052347, 0.000000),
            vector_float4::new(-0.034907, 0.052315, 0.998020, 0.000000),
            vector_float4::new(0.000000, 0.000000, 0.000000, 1.000000)
        );
        assert!(exp._private[0].x() - rot._private[0].x() < 0.00001);
    }
}
