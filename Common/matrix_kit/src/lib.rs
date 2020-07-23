//
//  lib.rs
//
//  Created by TR Solutions on 2020-07-23.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
#![allow(dead_code)]
#![deny(missing_docs)]
//! Thin wrappers for the classes and methods we use from Core Animation

mod vector_types;
mod matrix_types;

pub use vector_types::vector_uint2;
pub use vector_types::vector_float2;
pub use vector_types::vector_float3;
pub use vector_types::vector_float4;
pub use matrix_types::matrix_float4x4;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
