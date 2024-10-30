// agrega::interp::distance
//
// - definitions
//   - fn line_mr
//   - trait DistanceInterpolator
//   - struct DistanceInterpolator00
//   - struct DistanceInterpolator0
//   - struct DistanceInterpolator1
//   - struct DistanceInterpolator2
//   - struct DistanceInterpolator3
// - struct implementations
// - trait implementations

use crate::util::*;
#[allow(unused_imports)]
use devela::ExtFloat;

/* definitions */

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
    /// Increments the y-distance by `dx`.
    fn inc_y(&mut self, dx: i64);
    /// Decrements the x-distance by `dy`.
    fn dec_x(&mut self, dy: i64);
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
pub(super) struct DistanceInterpolator2 {
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

/* implementations */

impl DistanceInterpolator00 {
    /// Creates a new `DistanceInterpolator00` instance.
    ///
    /// Calculates initial distance values `dist1` and `dist2` based on the
    /// provided coordinates, using a set of adjustments based on subpixel scaling.
    /// The resulting interpolator allows for precise distance interpolation
    /// between two points with subpixel accuracy.
    ///
    /// # Parameters
    /// - `xc`, `yc`: Center reference point coordinates.
    /// - `x1`, `y1`: Coordinates for point 1.
    /// - `x2`, `y2`: Coordinates for point 2.
    /// - `x`, `y`: Target coordinates for interpolation.
    #[expect(clippy::too_many_arguments)]
    pub const fn new(xc: i64, yc: i64, x1: i64, y1: i64, x2: i64, y2: i64, x: i64, y: i64) -> Self {
        let dx1 = line_mr(x1) - line_mr(xc);
        let dy1 = line_mr(y1) - line_mr(yc);
        let dx2 = line_mr(x2) - line_mr(xc);
        let dy2 = line_mr(y2) - line_mr(yc);
        let dist1 = (line_mr(x + POLY_SUBPIXEL_SCALE / 2) - line_mr(x1)) * dy1
            - (line_mr(y + POLY_SUBPIXEL_SCALE / 2) - line_mr(y1)) * dx1;
        let dist2 = (line_mr(x + POLY_SUBPIXEL_SCALE / 2) - line_mr(x2)) * dy2
            - (line_mr(y + POLY_SUBPIXEL_SCALE / 2) - line_mr(y2)) * dx2;
        let dx1 = dx1 << POLY_MR_SUBPIXEL_SHIFT;
        let dy1 = dy1 << POLY_MR_SUBPIXEL_SHIFT;
        let dx2 = dx2 << POLY_MR_SUBPIXEL_SHIFT;
        let dy2 = dy2 << POLY_MR_SUBPIXEL_SHIFT;

        Self { dx1, dy1, dx2, dy2, dist1, dist2 }
    }

    /// Increments the x-distance for both points, adjusting `dist1` and `dist2`
    /// based on `dy1` and `dy2`.
    ///
    /// This method modifies `dist1` and `dist2` by adding the `dy` component
    /// of each respective distance vector, effectively updating the interpolated
    /// distance along the x-axis.
    #[inline]
    pub fn inc_x(&mut self) {
        self.dist1 += self.dy1;
        self.dist2 += self.dy2;
    }
}

impl DistanceInterpolator0 {
    /// Creates a new `DistanceInterpolator0` instance.
    ///
    /// Initializes the interpolator with the provided coordinates, calculating the
    /// initial values of `dx`, `dy`, and `dist`. The coordinates are adjusted using
    /// mid-range subpixel precision to provide fine-grained control, which is beneficial
    /// for rendering tasks requiring subpixel accuracy.
    ///
    /// # Parameters
    /// - `x1`, `y1`: Coordinates of the starting point.
    /// - `x2`, `y2`: Coordinates of the ending point.
    /// - `x`, `y`: Coordinates of the target point for interpolation.
    pub const fn new(x1: i64, y1: i64, x2: i64, y2: i64, x: i64, y: i64) -> Self {
        let dx = line_mr(x2) - line_mr(x1);
        let dy = line_mr(y2) - line_mr(y1);
        let dist = (line_mr(x + POLY_SUBPIXEL_SCALE / 2) - line_mr(x2)) * dy
            - (line_mr(y + POLY_SUBPIXEL_SCALE / 2) - line_mr(y2)) * dx;
        let dx = dx << POLY_MR_SUBPIXEL_SHIFT;
        let dy = dy << POLY_MR_SUBPIXEL_SHIFT;
        Self { dx, dy, dist }
    }

    /// Increments the x-distance component, adjusting `dist` by `dy`.
    ///
    /// This method increments the interpolated distance along the x-axis by adding
    /// the y-distance component `dy` to `dist`. It supports efficient horizontal
    /// interpolation by updating `dist` incrementally.
    #[inline]
    pub fn inc_x(&mut self) {
        self.dist += self.dy;
    }
}

impl DistanceInterpolator1 {
    /// Creates a new `DistanceInterpolator1` instance.
    ///
    /// Initializes the interpolator by calculating the x and y distances (`dx` and `dy`)
    /// in both pixel and subpixel coordinates and computing an initial `dist` value
    /// in subpixel precision. This interpolator is suitable for scenarios requiring
    /// high-precision distance calculations between two points with floating-point
    /// rounding to accommodate accurate interpolation.
    ///
    /// # Parameters
    /// - `x1`, `y1`: Coordinates of the starting point.
    /// - `x2`, `y2`: Coordinates of the ending point.
    /// - `x`, `y`: Coordinates of the target point for interpolation.
    pub fn new(x1: i64, y1: i64, x2: i64, y2: i64, x: i64, y: i64) -> Self {
        let dx = x2 - x1; // pixels
        let dy = y2 - y1; // pixels
        let dist_fp = (x + POLY_SUBPIXEL_SCALE / 2 - x2) as f64 * dy as f64
            - (y + POLY_SUBPIXEL_SCALE / 2 - y2) as f64 * dx as f64;
        let dist = dist_fp.round() as i64;
        let dx = dx << POLY_SUBPIXEL_SHIFT; // subpixels
        let dy = dy << POLY_SUBPIXEL_SHIFT; // subpixels
        Self { dist, dx, dy }
    }

    /// Returns the x-distance `dx` in subpixel coordinates.
    #[inline]
    #[allow(dead_code)]
    pub const fn dx(&self) -> i64 {
        self.dx
    }
    /// Returns the y-distance `dy` in subpixel coordinates.
    #[inline]
    #[allow(dead_code)]
    pub const fn dy(&self) -> i64 {
        self.dy
    }
}

impl DistanceInterpolator2 {
    /// Creates a new `DistanceInterpolator2` instance.
    ///
    /// Initializes the interpolator by computing distances in both full subpixel
    /// and mid-range subpixel precision for increased flexibility in interpolation.
    ///
    /// The starting distances (`dx_start`, `dy_start`, `dist_start`) are conditionally
    /// set based on the `start` flag, determining if the starting point is closer
    /// to `(x1, y1)` or `(x2, y2)`. This setup supports precise interpolation for cases
    /// requiring a distinct reference point.
    ///
    /// # Parameters
    /// - `x1`, `y1`: Coordinates of the starting point.
    /// - `x2`, `y2`: Coordinates of the ending point.
    /// - `sx`, `sy`: Coordinates of the reference start point.
    /// - `x`, `y`: Target coordinates for interpolation.
    /// - `start`: Boolean flag that determines if `sx` and `sy` are closer to `(x1, y1)`
    ///   (if `true`) or `(x2, y2)` (if `false`).
    #[expect(clippy::too_many_arguments)] #[rustfmt::skip]
    pub fn new( x1: i64, y1: i64, x2: i64, y2: i64, sx: i64, sy: i64, x: i64, y: i64, start: bool,
    ) -> Self {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let (dx_start, dy_start) = if start {
            (line_mr(sx) - line_mr(x1), line_mr(sy) - line_mr(y1))
        } else {
            (line_mr(sx) - line_mr(x2), line_mr(sy) - line_mr(y2))
        };
        let dist = (x + POLY_SUBPIXEL_SCALE / 2 - x2) as f64 * dy as f64
            - (y + POLY_SUBPIXEL_SCALE / 2 - y2) as f64 * dx as f64;
        let dist = dist.round() as i64;
        let dist_start = (line_mr(x + POLY_SUBPIXEL_SCALE / 2) - line_mr(sx)) * dy_start
            - (line_mr(y + POLY_SUBPIXEL_SCALE / 2) - line_mr(sy)) * dx_start;
        let dx = dx << POLY_SUBPIXEL_SHIFT;
        let dy = dy << POLY_SUBPIXEL_SHIFT;
        let dx_start = dx_start << POLY_MR_SUBPIXEL_SHIFT;
        let dy_start = dy_start << POLY_MR_SUBPIXEL_SHIFT;

        Self { dx, dy, dx_start, dy_start, dist, dist_start }
    }
}

impl DistanceInterpolator3 {
    /// Creates a new `DistanceInterpolator3` instance.
    ///
    /// Initializes the interpolator with distances calculated for both the start and end
    /// reference points in mid-range subpixel precision. This interpolator allows for
    /// interpolation that accounts for both an initial and terminal point, providing
    /// a richer model for scenarios where interpolation endpoints need precise tracking.
    ///
    /// # Parameters
    /// - `x1`, `y1`: Coordinates of the starting point.
    /// - `x2`, `y2`: Coordinates of the ending point.
    /// - `sx`, `sy`: Coordinates of the start reference point.
    /// - `ex`, `ey`: Coordinates of the end reference point.
    /// - `x`, `y`: Target coordinates for interpolation.
    #[expect(clippy::too_many_arguments)] #[rustfmt::skip]
    pub fn new(
        x1: i64, y1: i64, x2: i64, y2: i64, sx: i64, sy: i64, ex: i64, ey: i64, x: i64, y: i64,
    ) -> Self {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dx_start = line_mr(sx) - line_mr(x1);
        let dy_start = line_mr(sy) - line_mr(y1);
        let dx_end = line_mr(ex) - line_mr(x2);
        let dy_end = line_mr(ey) - line_mr(y2);

        let dist = (x + POLY_SUBPIXEL_SCALE / 2 - x2) as f64 * dy as f64
            - (y + POLY_SUBPIXEL_SCALE / 2 - y2) as f64 * dx as f64;
        let dist = dist.round() as i64;
        let dist_start = (line_mr(x + POLY_SUBPIXEL_SCALE / 2) - line_mr(sx)) * dy_start
            - (line_mr(y + POLY_SUBPIXEL_SCALE / 2) - line_mr(sy)) * dx_start;
        let dist_end = (line_mr(x + POLY_SUBPIXEL_SCALE / 2) - line_mr(ex)) * dy_end
            - (line_mr(y + POLY_SUBPIXEL_SCALE / 2) - line_mr(ey)) * dx_end;

        let dx = dx << POLY_SUBPIXEL_SHIFT;
        let dy = dy << POLY_SUBPIXEL_SHIFT;
        let dx_start = dx_start << POLY_MR_SUBPIXEL_SHIFT;
        let dy_start = dy_start << POLY_MR_SUBPIXEL_SHIFT;
        let dx_end = dx_end << POLY_MR_SUBPIXEL_SHIFT;
        let dy_end = dy_end << POLY_MR_SUBPIXEL_SHIFT;
        Self { dx, dy, dx_start, dy_start, dx_end, dy_end, dist_start, dist_end, dist }
    }
}

/* trait implementations */

impl DistanceInterpolator for DistanceInterpolator1 {
    #[inline] #[rustfmt::skip]
    fn dist(&self) -> i64 { self.dist }
    #[inline] #[rustfmt::skip]
    fn inc_x(&mut self, dy: i64) {
        self.dist += self.dy;
        if dy > 0 { self.dist -= self.dx; }
        if dy < 0 { self.dist += self.dx; }
    }
    #[inline] #[rustfmt::skip]
    fn dec_x(&mut self, dy: i64) {
        self.dist -= self.dy;
        if dy > 0 { self.dist -= self.dx; }
        if dy < 0 { self.dist += self.dx; }
    }
    #[inline] #[rustfmt::skip]
    fn inc_y(&mut self, dx: i64) {
        self.dist -= self.dx;
        if dx > 0 { self.dist += self.dy; }
        if dx < 0 { self.dist -= self.dy; }
    }
    #[inline] #[rustfmt::skip]
    fn dec_y(&mut self, dx: i64) {
        self.dist += self.dx;
        if dx > 0 { self.dist += self.dy; }
        if dx < 0 { self.dist -= self.dy; }
    }
}

impl DistanceInterpolator for DistanceInterpolator2 {
    #[inline] #[rustfmt::skip]
    fn dist(&self) -> i64 { self.dist }
    fn inc_x(&mut self, dy: i64) {
        self.dist += self.dy;
        self.dist_start += self.dy_start;
        if dy > 0 {
            self.dist -= self.dx;
            self.dist_start -= self.dx_start;
        }
        if dy < 0 {
            self.dist += self.dx;
            self.dist_start += self.dx_start;
        }
    }
    fn inc_y(&mut self, dx: i64) {
        self.dist -= self.dx;
        self.dist_start -= self.dx_start;
        if dx > 0 {
            self.dist += self.dy;
            self.dist_start += self.dy_start;
        }
        if dx < 0 {
            self.dist -= self.dy;
            self.dist_start -= self.dy_start;
        }
    }
    fn dec_x(&mut self, dy: i64) {
        self.dist -= self.dy;
        self.dist_start -= self.dy_start;
        if dy > 0 {
            self.dist -= self.dx;
            self.dist_start -= self.dx_start;
        }
        if dy < 0 {
            self.dist += self.dx;
            self.dist_start += self.dx_start;
        }
    }
    fn dec_y(&mut self, dx: i64) {
        self.dist += self.dx;
        self.dist_start += self.dx_start;
        if dx > 0 {
            self.dist += self.dy;
            self.dist_start += self.dy_start;
        }
        if dx < 0 {
            self.dist -= self.dy;
            self.dist_start -= self.dy_start;
        }
    }
}

impl DistanceInterpolator for DistanceInterpolator3 {
    #[inline]
    fn dist(&self) -> i64 {
        self.dist
    }
    fn inc_x(&mut self, dy: i64) {
        self.dist += self.dy;
        self.dist_start += self.dy_start;
        self.dist_end += self.dy_end;
        if dy > 0 {
            self.dist -= self.dx;
            self.dist_start -= self.dx_start;
            self.dist_end -= self.dx_end;
        }
        if dy < 0 {
            self.dist += self.dx;
            self.dist_start += self.dx_start;
            self.dist_end += self.dx_end;
        }
    }
    fn inc_y(&mut self, dx: i64) {
        self.dist -= self.dx;
        self.dist_start -= self.dx_start;
        self.dist_end -= self.dx_end;
        if dx > 0 {
            self.dist += self.dy;
            self.dist_start += self.dy_start;
            self.dist_end += self.dy_end;
        }
        if dx < 0 {
            self.dist -= self.dy;
            self.dist_start -= self.dy_start;
            self.dist_end -= self.dy_end;
        }
    }
    fn dec_x(&mut self, dy: i64) {
        self.dist -= self.dy;
        self.dist_start -= self.dy_start;
        self.dist_end -= self.dy_end;
        if dy > 0 {
            self.dist -= self.dx;
            self.dist_start -= self.dx_start;
            self.dist_end -= self.dx_end;
        }
        if dy < 0 {
            self.dist += self.dx;
            self.dist_start += self.dx_start;
            self.dist_end += self.dx_end;
        }
    }
    fn dec_y(&mut self, dx: i64) {
        self.dist += self.dx;
        self.dist_start += self.dx_start;
        self.dist_end += self.dx_end;
        if dx > 0 {
            self.dist += self.dy;
            self.dist_start += self.dy_start;
            self.dist_end += self.dy_end;
        }
        if dx < 0 {
            self.dist -= self.dy;
            self.dist_start -= self.dy_start;
            self.dist_end -= self.dy_end;
        }
    }
}
