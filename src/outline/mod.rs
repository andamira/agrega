// agrega::outline
//
//! # Examples
//!
//! ## Rendering Outline, not Anti-Aliased
//! <img style="margin: 16; display:block; margin:auto;"
//! src="https://raw.githubusercontent.com/andamira/agrega/master/tests/images/primitive.png">
//! ```
//! # use agrega::{
//! #     Path, Pixfmt, Rgb8, Rgba8, RasterizerOutline, RenderingBase, RendererOutline,
//! # };
//! let pix = Pixfmt::<Rgb8>::new(100,100);
//! let mut ren_base = RenderingBase::new(pix);
//! ren_base.clear(Rgba8::new(255, 255, 255, 255) );
//!
//! let mut ren = RendererOutline::with_base(&mut ren_base);
//! ren.line_color(Rgba8::new(0,0,0,255));
//!
//! let mut path = Path::new();
//! path.move_to(10.0, 10.0);
//! path.line_to(50.0, 90.0);
//! path.line_to(90.0, 10.0);
//!
//! let mut ras = RasterizerOutline::with_primitive(&mut ren);
//! ras.add_path(&path);
//! # #[cfg(feature = "std")]
//! ren_base.to_file("tests/std/tmp/primitive.png").unwrap();
//! ```
//!
//! ## Rasterizer for Outlines with Anti-Aliasing
//! <img style="margin: 16; display:block; margin:auto;"
//! src="https://raw.githubusercontent.com/andamira/agrega/master/tests/images/outline_aa.png">
//! ```
//! # use agrega::{
//! #     Path, Pixfmt, Rgb8, Rgba8, DrawOutline, RendererOutlineAA, RenderingBase,
//! #     RasterizerOutlineAA
//! # };
//! // Create Image and Rendering Base
//! let pix = Pixfmt::<Rgb8>::new(100,100);
//! let mut ren_base = RenderingBase::new(pix);
//! ren_base.clear(Rgba8::new(255, 255, 255, 255) );
//!
//! // Create Outline Rendering, set color and width
//! let mut ren = RendererOutlineAA::with_base(&mut ren_base);
//! ren.color(Rgba8::new(0,0,0,255));
//! ren.width(20.0);
//!
//! // Create a Path
//! let mut path = Path::new();
//! path.move_to(10.0, 10.0);
//! path.line_to(50.0, 90.0);
//! path.line_to(90.0, 10.0);
//!
//! // Create Outline Rasterizer and add path
//! let mut ras = RasterizerOutlineAA::with_renderer(&mut ren);
//! ras.round_cap(true);
//! ras.add_path(&path);
//! # #[cfg(feature = "std")]
//! ren_base.to_file("tests/std/tmp/outline_aa.png").unwrap();
//! ```
mod aa_rast;
mod aa_rend;
mod rast;
mod rend;
pub use {aa_rast::*, aa_rend::*, rast::*, rend::*};

/* */

/// Represents a coordinate with subpixel precision, stored as a fixed-point integer.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub(crate) struct Subpixel(i64);
impl Subpixel {
    /// Returns the raw subpixel value as an `i64`.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn value(self) -> i64 { self.0 }
}
impl From<i64> for Subpixel {
    /// Creates a `Subpixel` from an `i64`.
    #[inline] #[rustfmt::skip]
    fn from(v: i64) -> Self { Subpixel(v) }
}
impl From<Subpixel> for i64 {
    /// Converts a `Subpixel` to an `i64`.
    #[inline] #[rustfmt::skip]
    fn from(v: Subpixel) -> Self { v.0 >> crate::util::POLY_SUBPIXEL_SHIFT }
}
