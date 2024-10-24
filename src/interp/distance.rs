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

//
pub(crate) const fn line_mr(x: i64) -> i64 {
    x >> (POLY_SUBPIXEL_SHIFT - POLY_MR_SUBPIXEL_SHIFT)
}

//
pub(crate) trait DistanceInterpolator {
    fn dist(&self) -> i64;
    fn inc_x(&mut self, dy: i64);
    fn inc_y(&mut self, dx: i64);
    fn dec_x(&mut self, dy: i64);
    fn dec_y(&mut self, dx: i64);
}

//
#[derive(Debug)]
pub(crate) struct DistanceInterpolator00 {
    #[expect(dead_code)]
    dx1: i64,
    dy1: i64,
    #[expect(dead_code)]
    dx2: i64,
    dy2: i64,
    pub dist1: i64,
    pub dist2: i64,
}
//
#[derive(Debug)]
pub(crate) struct DistanceInterpolator0 {
    #[expect(dead_code)]
    dx: i64,
    dy: i64,
    pub dist: i64,
}

/// Distance Interpolator v1
#[derive(Debug)]
pub(crate) struct DistanceInterpolator1 {
    /// x distance from point 1 to point 2 in Subpixel Coordinates
    dx: i64,
    /// y distance from point 1 to point 2 in Subpixel Coordinates
    dy: i64,
    /// Distance
    pub dist: i64,
}

/// Distance Interpolator v2
#[derive(Debug)]
pub(super) struct DistanceInterpolator2 {
    dx: i64,
    dy: i64,
    pub dx_start: i64,
    pub dy_start: i64,
    dist: i64,
    pub dist_start: i64,
}

/// Distance Interpolator v3
#[derive(Debug)]
pub(crate) struct DistanceInterpolator3 {
    dx: i64,
    dy: i64,
    pub dx_start: i64,
    pub dy_start: i64,
    pub dx_end: i64,
    pub dy_end: i64,
    dist: i64,
    pub dist_start: i64,
    pub dist_end: i64,
}

/* implementations */

impl DistanceInterpolator00 {
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
    pub fn inc_x(&mut self) {
        self.dist1 += self.dy1;
        self.dist2 += self.dy2;
    }
}

impl DistanceInterpolator0 {
    pub const fn new(x1: i64, y1: i64, x2: i64, y2: i64, x: i64, y: i64) -> Self {
        let dx = line_mr(x2) - line_mr(x1);
        let dy = line_mr(y2) - line_mr(y1);
        let dist = (line_mr(x + POLY_SUBPIXEL_SCALE / 2) - line_mr(x2)) * dy
            - (line_mr(y + POLY_SUBPIXEL_SCALE / 2) - line_mr(y2)) * dx;
        let dx = dx << POLY_MR_SUBPIXEL_SHIFT;
        let dy = dy << POLY_MR_SUBPIXEL_SHIFT;
        Self { dx, dy, dist }
    }
    pub fn inc_x(&mut self) {
        self.dist += self.dy;
    }
}

impl DistanceInterpolator1 {
    /// Create a new Distance Interpolator
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
    #[allow(dead_code)]
    pub const fn dx(&self) -> i64 {
        self.dx
    }
    #[allow(dead_code)]
    pub const fn dy(&self) -> i64 {
        self.dy
    }
}

impl DistanceInterpolator2 {
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        sx: i64,
        sy: i64,
        x: i64,
        y: i64,
        start: bool,
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
    #[expect(clippy::too_many_arguments)]
    pub fn new(
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        sx: i64,
        sy: i64,
        ex: i64,
        ey: i64,
        x: i64,
        y: i64,
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
    /// Return the current distance
    fn dist(&self) -> i64 {
        self.dist
    }
    /// Increment x
    ///
    /// Add dy to distance and adjust dist by dx value
    fn inc_x(&mut self, dy: i64) {
        self.dist += self.dy;
        if dy > 0 {
            self.dist -= self.dx;
        }
        if dy < 0 {
            self.dist += self.dx;
        }
    }
    /// Decrement x
    ///
    /// Remove dy to distance and adjust dist by dx value
    fn dec_x(&mut self, dy: i64) {
        self.dist -= self.dy;
        if dy > 0 {
            self.dist -= self.dx;
        }
        if dy < 0 {
            self.dist += self.dx;
        }
    }
    /// Increment y
    ///
    /// Remove `dx` to `distance` and adjust dist by `dy` value
    fn inc_y(&mut self, dx: i64) {
        self.dist -= self.dx;
        if dx > 0 {
            self.dist += self.dy;
        }
        if dx < 0 {
            self.dist -= self.dy;
        }
    }
    /// Decrement y
    ///
    /// Add `dx` to `distance` and adjust dist by `dy` value
    fn dec_y(&mut self, dx: i64) {
        self.dist += self.dx;
        if dx > 0 {
            self.dist += self.dy;
        }
        if dx < 0 {
            self.dist -= self.dy;
        }
    }
}

impl DistanceInterpolator for DistanceInterpolator2 {
    fn dist(&self) -> i64 {
        self.dist
    }
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
