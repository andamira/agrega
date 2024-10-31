// agrega::interp::dist_impls
//
//! Distance interpolation, implementations.
//
// TOC
// - trait DistanceInterpolator
// - struct DistanceInterpolator00
// - struct DistanceInterpolator0
// - struct DistanceInterpolator1
// - struct DistanceInterpolator2
// - struct DistanceInterpolator3
// - struct DistanceInterpolator4

use super::{
    line_mr, DistanceInterpolator, DistanceInterpolator0, DistanceInterpolator00,
    DistanceInterpolator1, DistanceInterpolator2, DistanceInterpolator3, DistanceInterpolator4,
};
use crate::util::*;
use devela::iif;
#[allow(unused_imports)]
use devela::ExtFloat;

macro_rules! impl_distance_interp {
    ($($t:ty),+) => { $( impl_distance_interp!(@$t); )+ };
    (@$t:ty) => { impl DistanceInterpolator for $t {
        #[inline] fn dist(&self) -> i64 { self.dist }
        #[inline] fn inc_x(&mut self, dy: i64) { self.inc_x_by(dy) }
        #[inline] fn dec_x(&mut self, dy: i64) { self.dec_x_by(dy) }
        #[inline] fn inc_y(&mut self, dx: i64) { self.inc_y_by(dx) }
        #[inline] fn dec_y(&mut self, dx: i64) { self.dec_y_by(dx) }
    }};
}
impl_distance_interp![
    DistanceInterpolator1,
    DistanceInterpolator2,
    DistanceInterpolator3,
    DistanceInterpolator4
];

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

    /// Increments the x-distance by `dy`.
    #[inline] #[rustfmt::skip]
    pub fn inc_x_by(&mut self, dy: i64) {
        self.dist += self.dy;
        if dy > 0 { self.dist -= self.dx; }
        if dy < 0 { self.dist += self.dx; }
    }
    /// Decrements the x-distance by `dy`.
    #[inline] #[rustfmt::skip]
    pub fn dec_x_by(&mut self, dy: i64) {
        self.dist -= self.dy;
        if dy > 0 { self.dist -= self.dx; }
        if dy < 0 { self.dist += self.dx; }
    }
    /// Increments the y-distance by `dx`.
    #[inline] #[rustfmt::skip]
    pub fn inc_y_by(&mut self, dx: i64) {
        self.dist -= self.dx;
        if dx > 0 { self.dist += self.dy; }
        if dx < 0 { self.dist -= self.dy; }
    }
    /// Decrements the y-distance by `dx`.
    #[inline] #[rustfmt::skip]
    pub fn dec_y_by(&mut self, dx: i64) {
        self.dist += self.dx;
        if dx > 0 { self.dist += self.dy; }
        if dx < 0 { self.dist -= self.dy; }
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

    pub fn inc_x_by(&mut self, dy: i64) {
        self.dist += self.dy;
        self.dist_start += self.dy_start;
        iif![dy > 0; { self.dist -= self.dx; self.dist_start -= self.dx_start }];
        iif![dy < 0; { self.dist += self.dx; self.dist_start += self.dx_start }];
    }
    pub fn dec_x_by(&mut self, dy: i64) {
        self.dist -= self.dy;
        self.dist_start -= self.dy_start;
        iif![dy > 0; { self.dist -= self.dx; self.dist_start -= self.dx_start }];
        iif![dy < 0; { self.dist += self.dx; self.dist_start += self.dx_start }];
    }
    pub fn inc_y_by(&mut self, dx: i64) {
        self.dist -= self.dx;
        self.dist_start -= self.dx_start;
        iif![dx > 0; { self.dist += self.dy; self.dist_start += self.dy_start }];
        iif![dx < 0; { self.dist -= self.dy; self.dist_start -= self.dy_start }];
    }
    pub fn dec_y_by(&mut self, dx: i64) {
        self.dist += self.dx;
        self.dist_start += self.dx_start;
        iif![dx > 0; { self.dist += self.dy; self.dist_start += self.dy_start }];
        iif![dx < 0; { self.dist -= self.dy; self.dist_start -= self.dy_start }];
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

    pub fn inc_x_by(&mut self, dy: i64) {
        self.dist += self.dy;
        self.dist_start += self.dy_start;
        self.dist_end += self.dy_end;
        #[allow(clippy::comparison_chain)]
        if dy > 0 {
            self.dist -= self.dx;
            self.dist_start -= self.dx_start;
            self.dist_end -= self.dx_end;
        } else if dy < 0 {
            self.dist += self.dx;
            self.dist_start += self.dx_start;
            self.dist_end += self.dx_end;
        }
    }
    pub fn dec_x_by(&mut self, dy: i64) {
        self.dist -= self.dy;
        self.dist_start -= self.dy_start;
        self.dist_end -= self.dy_end;
        #[allow(clippy::comparison_chain)]
        if dy > 0 {
            self.dist -= self.dx;
            self.dist_start -= self.dx_start;
            self.dist_end -= self.dx_end;
        } else if dy < 0 {
            self.dist += self.dx;
            self.dist_start += self.dx_start;
            self.dist_end += self.dx_end;
        }
    }

    pub fn inc_y_by(&mut self, dx: i64) {
        self.dist -= self.dx;
        self.dist_start -= self.dx_start;
        self.dist_end -= self.dx_end;
        #[allow(clippy::comparison_chain)]
        if dx > 0 {
            self.dist += self.dy;
            self.dist_start += self.dy_start;
            self.dist_end += self.dy_end;
        } else if dx < 0 {
            self.dist -= self.dy;
            self.dist_start -= self.dy_start;
            self.dist_end -= self.dy_end;
        }
    }
    pub fn dec_y_by(&mut self, dx: i64) {
        self.dist += self.dx;
        self.dist_start += self.dx_start;
        self.dist_end += self.dx_end;
        #[allow(clippy::comparison_chain)]
        if dx > 0 {
            self.dist += self.dy;
            self.dist_start += self.dy_start;
            self.dist_end += self.dy_end;
        } else if dx < 0 {
            self.dist -= self.dy;
            self.dist_start -= self.dy_start;
            self.dist_end -= self.dy_end;
        }
    }
}

impl DistanceInterpolator4 {
    /// Creates a new `DistanceInterpolator4` for multi-point interpolation.
    ///
    /// # Parameters
    /// - `x1`, `y1`: Starting coordinates.
    /// - `x2`, `y2`: Ending coordinates.
    /// - `sx`, `sy`: Start transformation coordinates.
    /// - `ex`, `ey`: End transformation coordinates.
    /// - `len`: Length of the interpolation.
    /// - `scale`: Scaling factor to adjust the length.
    /// - `x`, `y`: Current coordinates for interpolation calculations.
    #[allow(clippy::too_many_arguments)] #[rustfmt::skip]
    pub fn new(
        x1: i64, y1: i64, x2: i64, y2: i64, sx: i64, sy: i64,
        ex: i64, ey: i64, len: i64, scale: f64, x: i64, y: i64,
    ) -> Self {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dx_start = line_mr(sx) - line_mr(x1);
        let dy_start = line_mr(sy) - line_mr(y1);
        let dx_end = line_mr(ex) - line_mr(x2);
        let dy_end = line_mr(ey) - line_mr(y2);

        let dist = ((x + POLY_SUBPIXEL_SCALE / 2 - x2) as f64 * dy as f64
            - (y + POLY_SUBPIXEL_SCALE / 2 - y2) as f64 * dx as f64)
            .round() as i64;

        let dist_start = (line_mr(x + POLY_SUBPIXEL_SCALE / 2) - line_mr(sx)) * dy_start
            - (line_mr(y + POLY_SUBPIXEL_SCALE / 2) - line_mr(sy)) * dx_start;
        let dist_end = (line_mr(x + POLY_SUBPIXEL_SCALE / 2) - line_mr(ex)) * dy_end
            - (line_mr(y + POLY_SUBPIXEL_SCALE / 2) - line_mr(ey)) * dx_end;
        let len = (len as f64 / scale).round() as i64;
        let d = len as f64 * scale;
        let tdx = (((x2 - x1) << POLY_SUBPIXEL_SHIFT) as f64 / d).round() as i64;
        let tdy = (((y2 - y1) << POLY_SUBPIXEL_SHIFT) as f64 / d).round() as i64;
        let dx_pict = -tdy;
        let dy_pict = tdx;
        let dist_pict = ((x + POLY_SUBPIXEL_SCALE / 2 - (x1 - tdy)) * dy_pict
            - (y + POLY_SUBPIXEL_SCALE / 2 - (y1 + tdx)) * dx_pict)
            >> POLY_SUBPIXEL_SHIFT;
        let dx = dx << POLY_SUBPIXEL_SHIFT;
        let dy = dy << POLY_SUBPIXEL_SHIFT;
        let dx_start = dx_start << POLY_MR_SUBPIXEL_SHIFT;
        let dy_start = dy_start << POLY_MR_SUBPIXEL_SHIFT;
        let dx_end = dx_end << POLY_MR_SUBPIXEL_SHIFT;
        let dy_end = dy_end << POLY_MR_SUBPIXEL_SHIFT;

        Self { dx, dy, dx_start, dx_end, dy_start, dy_end,
            dx_pict, dy_pict, dist, dist_pict, dist_start, dist_end, len }
    }

    // /// Increments the x-axis distance values.
    // pub fn inc_x(&mut self) {
    //     self.dist += self.dy;
    //     self.dist_start += self.dy_start;
    //     self.dist_pict += self.dy_pict;
    //     self.dist_end += self.dy_end;
    // }
    // /// Decrements the x-axis distance values.
    // pub fn dec_x(&mut self) {
    //     self.dist -= self.dy;
    //     self.dist_start -= self.dy_start;
    //     self.dist_pict -= self.dy_pict;
    //     self.dist_end -= self.dy_end;
    // }
    // /// Increments the y-axis distance values.
    // pub fn inc_y(&mut self) {
    //     self.dist -= self.dx;
    //     self.dist_start -= self.dx_start;
    //     self.dist_pict -= self.dx_pict;
    //     self.dist_end -= self.dx_end;
    // }
    // /// Decrements the y-axis distance values.
    // pub fn dec_y(&mut self) {
    //     self.dist += self.dx;
    //     self.dist_start += self.dx_start;
    //     self.dist_pict += self.dx_pict;
    //     self.dist_end += self.dx_end;
    // }

    /// Incrementally adjusts x-axis values by a specific `dy` parameter.
    pub fn inc_x_by(&mut self, dy: i64) {
        self.dist += self.dy;
        self.dist_start += self.dy_start;
        self.dist_pict += self.dy_pict;
        self.dist_end += self.dy_end;
        #[allow(clippy::comparison_chain)]
        if dy > 0 {
            self.dist -= self.dx;
            self.dist_start -= self.dx_start;
            self.dist_pict -= self.dx_pict;
            self.dist_end -= self.dx_end;
        } else if dy < 0 {
            self.dist += self.dx;
            self.dist_start += self.dx_start;
            self.dist_pict += self.dx_pict;
            self.dist_end += self.dx_end;
        }
    }
    /// Decrementally adjusts x-axis values by a specific `dy` parameter.
    pub fn dec_x_by(&mut self, dy: i64) {
        self.dist -= self.dy;
        self.dist_start -= self.dy_start;
        self.dist_pict -= self.dy_pict;
        self.dist_end -= self.dy_end;
        #[allow(clippy::comparison_chain)]
        if dy > 0 {
            self.dist -= self.dx;
            self.dist_start -= self.dx_start;
            self.dist_pict -= self.dx_pict;
            self.dist_end -= self.dx_end;
        } else if dy < 0 {
            self.dist += self.dx;
            self.dist_start += self.dx_start;
            self.dist_pict += self.dx_pict;
            self.dist_end += self.dx_end;
        }
    }
    /// Incrementally adjusts y-axis values by a specific `dx` parameter.
    pub fn inc_y_by(&mut self, dx: i64) {
        self.dist -= self.dx;
        self.dist_start -= self.dx_start;
        self.dist_pict -= self.dx_pict;
        self.dist_end -= self.dx_end;
        #[allow(clippy::comparison_chain)]
        if dx > 0 {
            self.dist += self.dy;
            self.dist_start += self.dy_start;
            self.dist_pict += self.dy_pict;
            self.dist_end += self.dy_end;
        } else if dx < 0 {
            self.dist -= self.dy;
            self.dist_start -= self.dy_start;
            self.dist_pict -= self.dy_pict;
            self.dist_end -= self.dy_end;
        }
    }
    /// Decrementally adjusts y-axis values by a specific `dx` parameter.
    pub fn dec_y_by(&mut self, dx: i64) {
        self.dist += self.dx;
        self.dist_start += self.dx_start;
        self.dist_pict += self.dx_pict;
        self.dist_end += self.dx_end;
        #[allow(clippy::comparison_chain)]
        if dx > 0 {
            self.dist += self.dy;
            self.dist_start += self.dy_start;
            self.dist_pict += self.dy_pict;
            self.dist_end += self.dy_end;
        } else if dx < 0 {
            self.dist -= self.dy;
            self.dist_start -= self.dy_start;
            self.dist_pict -= self.dy_pict;
            self.dist_end -= self.dy_end;
        }
    }
}
