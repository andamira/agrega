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

// allows a group of items to share the same cfg options
#[allow(unused_macros)]
macro_rules! items { ( $($item:item)* ) => { $($item)* }; }

mod traits;
mod util;

#[cfg(feature = "alloc")]
items! {
    pub(crate) mod cell;
    mod clip;
    pub(crate) mod scan;
}

#[cfg(any(feature = "std", feature = "no_std"))]
items! {
    pub mod color;
    pub use color::*;
}

#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
items! {
    mod alphamask;
    mod base;
    mod pixfmt;
    pub mod interp;
    pub mod outline;
    pub mod outline_aa;
    pub mod paths;
    pub mod raster;
    pub mod render;
    pub mod stroke;
    pub mod text;
    pub mod transform;

    pub use {alphamask::*, util::*, pixfmt::*};
    #[doc(hidden)]
    pub use {
        base::*, clip::*, interp::*, outline::*, outline_aa::*, paths::*, raster::*,
        render::*, stroke::*, text::*, traits::*, transform::*,
    };
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub mod ppm;

/// Library dependencies. <br/><hr>
pub mod _dep {
    #[doc(inline)]
    pub use devela;
    #[doc(inline)]
    #[cfg(feature = "freetype")]
    pub use freetype;
}
/// Gallery of images. <br/><hr>
pub mod _gallery {
    #![doc = include_str!("./Gallery.md")]
}
/// All items are flat re-exported here. <br/><hr>
pub mod all {
    #[doc(inline)]
    pub use super::{traits::*, util::*};

    #[doc(inline)]
    #[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
    pub use super::{
        alphamask::*, base::*, clip::*, color::*, interp::*, outline::*, outline_aa::*, paths::*,
        pixfmt::*, raster::*, render::*, stroke::*, text::*, transform::*,
    };

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::ppm::*;
}
