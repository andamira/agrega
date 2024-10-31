// agrega::interp::dist_defs
//
//! Distance interpolation, definitions.
//
// - fn line_mr
// - struct Interpolator
// - trait DistanceInterpolator
// - struct DistanceInterpolator00
// - struct DistanceInterpolator0
// - struct DistanceInterpolator1
// - struct DistanceInterpolator2
// - struct DistanceInterpolator3
// - struct DistanceInterpolator4

use crate::util::*;

/// Converts a coordinate from full subpixel precision to mid-range subpixel precision.
///
/// This function shifts the input coordinate `x` from a high-precision subpixel scale
/// (defined by `POLY_SUBPIXEL_SHIFT`) down to a mid-range subpixel scale
/// (defined by `POLY_MR_SUBPIXEL_SHIFT`).
#[inline]
#[must_use]
pub(crate) const fn line_mr(x: i64) -> i64 {
    x >> (POLY_SUBPIXEL_SHIFT - POLY_MR_SUBPIXEL_SHIFT)
}

/// Common trait for distance interpolators.
///
/// Defines a set of methods for manipulating distance values in interpolation
/// processes used for rendering. Each interpolator has its own behavior for
/// incrementing and decrementing distances based on x or y deltas.
pub(crate) trait DistanceInterpolator {
    /// Returns the current distance.
    #[must_use]
    fn dist(&self) -> i64;
    /// Increments the x-distance by `dy`.
    fn inc_x(&mut self, dy: i64);
    /// Decrements the x-distance by `dy`.
    fn dec_x(&mut self, dy: i64);
    /// Increments the y-distance by `dx`.
    fn inc_y(&mut self, dx: i64);
    /// Decrements the y-distance by `dx`.
    fn dec_y(&mut self, dx: i64);
}

/// Distance Interpolator v00.
///
/// A basic interpolator version that stores two distance pairs `(dx1, dy1)`
/// and `(dx2, dy2)` and their associated distances `dist1` and `dist2`.
/// Useful for scenarios where two distinct distance metrics are tracked.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DistanceInterpolator00 {
    /// X distance in subpixel coordinates to point 1.
    pub dx1: i64,
    /// Y distance in subpixel coordinates to point 1.
    pub dy1: i64,
    /// X distance in subpixel coordinates to point 2.
    pub dx2: i64,
    /// Y distance in subpixel coordinates to point 2.
    pub dy2: i64,
    /// Distance from point 1.
    pub dist1: i64,
    /// Distance from point 2.
    pub dist2: i64,
}

/// Distance Interpolator v0.
///
/// Simplified interpolator holding a single distance vector `(dx, dy)`
/// and a single distance measurement `dist`. Used when only one primary
/// distance needs tracking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DistanceInterpolator0 {
    /// X distance in subpixel coordinates
    pub dx: i64,
    /// Y distance in subpixel coordinates
    pub dy: i64,
    /// The primary distance measurement.
    pub dist: i64,
}

/// Distance Interpolator v1.
///
/// This variant maintains a distance vector `(dx, dy)` representing the
/// x and y distances in subpixel coordinates between two points, along
/// with a distance value `dist`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DistanceInterpolator1 {
    /// X distance in subpixel coordinates
    pub dx: i64,
    /// Y distance in subpixel coordinates.
    pub dy: i64,
    /// Distance
    pub dist: i64,
}

/// Distance Interpolator v2.
///
/// Provides additional control over the starting point, storing the initial
/// `(dx_start, dy_start)` vector and `dist_start`. Useful when the interpolation
/// requires a reference to starting conditions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DistanceInterpolator2 {
    /// X distance in subpixel coordinates
    pub dx: i64,
    /// Y distance in subpixel coordinates
    pub dy: i64,
    /// Initial x-distance component.
    pub dx_start: i64,
    /// Initial y-distance component.
    pub dy_start: i64,
    /// Current distance.
    pub dist: i64,
    /// Initial distance.
    pub dist_start: i64,
}

/// Distance Interpolator v3.
///
/// Adds end-point control, storing both starting and ending vectors
/// (`dx_start`, `dy_start`, `dx_end`, `dy_end`) and distances `dist_start`
/// and `dist_end`. Useful for bi-directional interpolations where both
/// endpoints are known.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DistanceInterpolator3 {
    /// X distance in subpixel coordinates
    pub dx: i64,
    /// Y distance in subpixel coordinates
    pub dy: i64,
    /// Initial x-distance component.
    pub dx_start: i64,
    /// Initial y-distance component.
    pub dy_start: i64,
    /// Final x-distance component.
    pub dx_end: i64,
    /// Final y-distance component.
    pub dy_end: i64,
    /// Current distance.
    pub dist: i64,
    /// Initial distance.
    pub dist_start: i64,
    /// Final distance.
    pub dist_end: i64,
}

/// Distance Interpolator v4.
///
/// An advanced interpolator with complete starting and ending vectors,
/// pictorial transformation vectors, and associated distances. Used for
/// complex interpolations needing multiple transformation references.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DistanceInterpolator4 {
    /// X distance in subpixel coordinates.
    pub dx: i64,
    /// Y distance in subpixel coordinates.
    pub dy: i64,
    /// X distance from the starting point in subpixel coordinates.
    pub dx_start: i64,
    /// Y distance from the starting point in subpixel coordinates.
    pub dy_start: i64,
    /// X distance for pictorial transformation.
    pub dx_pict: i64,
    /// Y distance for pictorial transformation.
    pub dy_pict: i64,
    /// X distance to the end point in subpixel coordinates.
    pub dx_end: i64,
    /// Y distance to the end point in subpixel coordinates.
    pub dy_end: i64,
    /// Current interpolated distance.
    pub dist: i64,
    /// Distance from the start point.
    pub dist_start: i64,
    /// Distance for the pictorial transformation.
    pub dist_pict: i64,
    /// Distance to the end point.
    pub dist_end: i64,
    /// Length of the interpolation adjusted by scale.
    pub len: i64,
}
