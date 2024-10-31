// agrega::scanline::gradient
//
//! Gradient.
//!
//! This module provides structures and functions to apply gradient transformations
//! to spans of color data in a rasterized image. Gradients can be used to produce
//! smooth color transitions based on positional data and transformations.

use crate::{Interpolator, Rgb8, Transform};
use alloc::{vec, vec::Vec};
use devela::iif;
#[allow(unused_imports)]
use devela::ExtFloat;

/// Represents a gradient in the x-direction.
#[derive(Clone, Debug)]
pub struct GradientX {}
impl GradientX {
    /// Calculates the gradient value for the x-position.
    ///
    /// # Parameters
    /// - `x`: The x-coordinate to calculate the gradient value for.
    #[inline]
    pub const fn calculate(&self, x: i64, _: i64, _: i64) -> i64 {
        x
    }
}

/// Holds gradient data and properties to generate color spans.
///
/// Supports the application of a gradient across a specified range (`d1` to `d2`).
/// It interpolates colors from a palette based on position and transformation.
#[derive(Clone, Debug)]
pub struct SpanGradient {
    d1: i64,
    d2: i64,
    gradient: GradientX,
    color: Vec<Rgb8>,
    trans: Transform,
}

impl SpanGradient {
    /// Creates a new `SpanGradient`.
    ///
    /// # Parameters
    /// - `trans`: Transformation to apply to the gradient.
    /// - `gradient`: The gradient type to use.
    /// - `color`: A slice of colors used for interpolation.
    /// - `d1`: Start of the gradient range.
    /// - `d2`: End of the gradient range.
    #[inline]
    pub fn new(trans: Transform, gradient: GradientX, color: &[Rgb8], d1: f64, d2: f64) -> Self {
        let mut s = Self { d1: 0, d2: 1, color: color.to_vec(), gradient, trans };
        s.d1(d1);
        s.d2(d2);
        s
    }

    /// Returns the subpixel shift value, used for precise positioning.
    #[inline]
    pub const fn subpixel_shift(&self) -> i64 {
        4
    }

    /// Returns the subpixel scaling factor, derived from the subpixel shift.
    #[inline]
    pub const fn subpixel_scale(&self) -> i64 {
        1 << self.subpixel_shift()
    }

    /// Sets the starting point (`d1`) of the gradient range after scaling.
    #[inline]
    pub fn d1(&mut self, d1: f64) {
        self.d1 = (d1 * self.subpixel_scale() as f64).round() as i64;
    }
    /// Sets the endpoint (`d2`) of the gradient range after scaling.
    #[inline]
    pub fn d2(&mut self, d2: f64) {
        self.d2 = (d2 * self.subpixel_scale() as f64).round() as i64;
    }

    /// Prepares the gradient for use, applying necessary pre-calculations. (no-op)
    #[inline]
    pub fn prepare(&mut self) {}

    /// Generates a span of colors based on the gradient at the specified coordinates.
    ///
    /// # Parameters
    /// - `x`: The x-coordinate to start generating the span.
    /// - `y`: The y-coordinate to start generating the span.
    /// - `len`: The number of pixels to generate in the span.
    pub fn generate(&self, x: i64, y: i64, len: usize) -> Vec<Rgb8> {
        let mut interp = Interpolator::new(self.trans);
        let downscale_shift = interp.subpixel_shift() - self.subpixel_shift();

        let mut dd = self.d2 - self.d1;
        iif![dd < 1; dd = 1];
        let ncolors = self.color.len() as i64;
        let mut span = vec![Rgb8::white(); len];

        interp.begin(x as f64 + 0.5, y as f64 + 0.5, len);

        for item in span.iter_mut().take(len) {
            let (x, y) = interp.coordinates();
            let d = self.gradient.calculate(x >> downscale_shift, y >> downscale_shift, self.d2);
            let mut d = ((d - self.d1) * ncolors) / dd;
            iif![ d < 0; d = 0];
            iif![d >= ncolors; d = ncolors - 1];
            *item = self.color[d as usize];
            interp.inc();
        }
        span
    }
}
