//
//  core_animation.rs
//
//  Created by TR Solutions on 2020-07-12.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//
//! Thin wrappers for the classes and methods we use from Core Animation

mod core_anim_display_link;
mod core_anim_metal_drawable;
mod core_anim_metal_layer;

pub use core_anim_display_link::CoreAnimDisplayLink;
pub use core_anim_metal_drawable::CoreAnimMetalDrawable;
pub use core_anim_metal_layer::MTLPixelFormat;
pub use core_anim_metal_layer::MTLPixelFormatBGRA8Unorm;
pub use core_anim_metal_layer::CoreAnimMetalLayer;