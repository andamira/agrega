// agrega::traits

#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
use crate::{LineParameters, RenderData, Vertex};
// #[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg(any(feature = "std", feature = "no_std"))]
use crate::Color;

/// A source of vertex points.
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "no_std", feature = "alloc"))))
)]
pub trait VertexSource {
    // /// Rewind the vertex source (unused)
    // fn rewind(&self) {}

    /// Get the cloned values from the source.
    ///
    /// This could be turned into an iterator
    #[must_use]
    fn xconvert(&self) -> alloc::vec::Vec<Vertex<f64>>;
}

/// Render scanlines to Image
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "no_std", feature = "alloc"))))
)]
pub trait Render {
    /// Render a single scanlines to the image
    fn render(&mut self, data: &RenderData);
    /// Set the Color of the Renderer
    fn color<C: Color>(&mut self, color: C);
    /// Prepare the Renderer
    fn prepare(&self) {}
}

/*
// MAYBE
/// Rasterize lines, path, and other things to scanlines
pub trait Rasterize {
    /// Setup Rasterizer, returns if data is available
    fn rewind_scanlines(&mut self) -> bool;
    /// Sweeps cells in a scanline for data, returns if data is available
    fn sweep_scanline(&mut self, sl: &mut ScanlineU8) -> bool;
    /// Return maximum x value of rasterizer
    fn min_x(&self) -> i64;
    /// Return maximum x value of rasterizer
    fn max_x(&self) -> i64;
    /// Resets the rasterizer, clearing content
    fn reset(&mut self);
    /// Rasterize a path
    fn add_path<VS: VertexSource>(&mut self, path: &VS);
}
*/

/// Functions for Drawing Outlines.
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "no_std", feature = "alloc"))))
)]
// pub trait DrawOutline: Lines + AccurateJoins + SetColor {} // MAYBE
pub trait DrawOutline {
    /// Set the current Color
    fn color<C: Color>(&mut self, color: C);
    /// If Line Joins are Accurate
    fn accurate_join_only(&self) -> bool;
    fn line0(&mut self, lp: &LineParameters);
    fn line1(&mut self, lp: &LineParameters, sx: i64, sy: i64);
    fn line2(&mut self, lp: &LineParameters, ex: i64, ey: i64);
    fn line3(&mut self, lp: &LineParameters, sx: i64, sy: i64, ex: i64, ey: i64);
    fn semidot<F>(&mut self, cmp: F, xc1: i64, yc1: i64, xc2: i64, yc2: i64)
    where
        F: Fn(i64) -> bool;
    fn pie(&mut self, xc: i64, y: i64, x1: i64, y1: i64, x2: i64, y2: i64);
}

// TODO
// pub(crate) trait LineInterp {
//     fn init(&mut self);
//     fn step_hor(&mut self);
//     fn step_ver(&mut self);
// }

pub(crate) trait RenderOutline {
    fn cover(&self, d: i64) -> u64;
    fn blend_solid_hspan(&mut self, x: i64, y: i64, len: i64, covers: &[u64]);
    fn blend_solid_vspan(&mut self, x: i64, y: i64, len: i64, covers: &[u64]);
}
