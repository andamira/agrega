//
#![doc = include_str!("./Lib.md")]
#![warn(clippy::all)]
#![allow(clippy::doc_lazy_continuation, clippy::module_inception)]
#![cfg_attr(not(any(feature = "std", feature = "no_std")), allow(unused))]
// nightly, safety, environment
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguarding: environment, safety
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable `safe` and `unsafe*` features at the same time.");

macro_rules! items { ( $($item:item)* ) => { $($item)* }; }

mod math;
mod traits;

/* alloc */

// private
#[cfg(feature = "alloc")]
items! {
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub(crate) mod cell;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    mod clip;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub(crate) mod scan;
}

/* std || no_std + alloc */

#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
items! {
    // private, few items
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    mod alphamask;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    mod base; // uses color
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    mod pixfmt; // uses color
    // public
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub mod color;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub mod interp;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub mod outline;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub mod outline_aa;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub mod paths;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub mod raster;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub mod render;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub mod stroke;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub mod text;
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub mod transform;
}

#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
pub use {alphamask::*, math::*, pixfmt::*};

#[doc(hidden)]
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
pub use {
    base::*, clip::*, color::*, interp::*, outline::*, outline_aa::*, paths::*, raster::*,
    render::*, stroke::*, text::*, traits::*, transform::*,
};

/* std */

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub mod ppm;

const POLY_SUBPIXEL_SHIFT: i64 = 8;
const POLY_SUBPIXEL_SCALE: i64 = 1 << POLY_SUBPIXEL_SHIFT;
const POLY_SUBPIXEL_MASK: i64 = POLY_SUBPIXEL_SCALE - 1;
const POLY_MR_SUBPIXEL_SHIFT: i64 = 4;
const MAX_HALF_WIDTH: usize = 64;

/// Library dependencies.
/// <br/><hr>
pub mod _dep {
    #[doc(inline)]
    pub use devela;
    #[doc(inline)]
    #[cfg(feature = "freetype")]
    pub use freetype;
}
/// Gallery of images.
/// <br/><hr>
pub mod _gallery {
    #![doc = include_str!("./Gallery.md")]
}

/// All items are flat re-exported here.
/// <br/><hr>
pub mod all {
    #[doc(inline)]
    pub use super::{math::*, traits::*};

    #[doc(inline)]
    #[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
    pub use super::{
        alphamask::*, base::*, clip::*, color::*, interp::*, outline::*, outline_aa::*, paths::*,
        pixfmt::*, raster::*, render::*, stroke::*, text::*, transform::*, DrawOutline, Render,
        Source, VertexSource,
    };

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::ppm::*;
}
