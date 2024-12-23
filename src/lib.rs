//
#![doc = include_str!("./Lib.md")]
#![warn(clippy::all)]
#![allow(
    clippy::doc_lazy_continuation,
    clippy::module_inception,
    clippy::zero_prefixed_literal
)]
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
use items;

/* modules ordered first by feature-bounds */

mod util;
#[doc(hidden)]
#[allow(unused_imports)]
pub use util::*;

#[cfg(feature = "alloc")]
items! {
    mod cell;
    pub mod paths;
    pub mod scanline;
    #[doc(hidden)]
    #[allow(unused_imports)]
    pub use {cell::*, paths::*, scanline::*};
}
#[cfg(any(feature = "std", feature = "no_std"))]
items! {
    pub mod color;
    #[doc(hidden)]
    pub use color::*;
}
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
items! {
    mod alphamask;
    mod base;
    pub mod interp;
    pub mod outline;
    pub mod pixfmt;
    pub mod text;

    #[doc(hidden)]
    #[allow(unused_imports)]
    pub use {alphamask::*, base::*, interp::*, outline::*,  pixfmt::*, text::*};
}

/// All items are flat re-exported here.<br/><hr>
pub mod all {
    #[doc(inline)]
    pub use super::util::*;

    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::{paths::*, scanline::*};

    #[doc(inline)]
    #[cfg(any(feature = "std", feature = "no_std"))]
    pub use super::color::*;

    #[doc(inline)]
    #[allow(unused_imports)]
    #[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
    pub use super::{alphamask::*, base::*, interp::*, outline::*, pixfmt::*, text::*};
}
/// Library dependencies.<br/><hr>
pub mod _dep {
    pub use devela;
    #[cfg(feature = "freetype")]
    pub use freetype;
}
/// Gallery of images. <br/><hr>
pub mod _gallery {
    #![doc = include_str!("./Gallery.md")]
}
