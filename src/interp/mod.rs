// agrega::interp

use crate::{util::*, LineInterpolatorImage, Transform};
use devela::iif;
#[allow(unused_imports)]
use devela::ExtFloat;

#[cfg(test)]
mod tests;

mod dist_defs;
mod dist_impls;
mod line_defs;
mod line_impls;
#[allow(unused_imports)]
pub use {dist_defs::*, line_defs::*};

/// TODO
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Interpolator {
    li_x: Option<LineInterpolator>,
    li_y: Option<LineInterpolator>,
    trans: Transform,
}
impl Interpolator {
    /// TODO
    #[inline]
    pub const fn new(trans: Transform) -> Self {
        Self { trans, li_x: None, li_y: None }
    }

    /// TODO
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn subpixel_shift(&self) -> i64 { 8 }

    /// TODO
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn subpixel_scale(&self) -> i64 { 1 << self.subpixel_shift() }

    /// TODO
    pub fn begin(&mut self, x: f64, y: f64, len: usize) {
        let (tx, ty) = self.trans.transform(x, y);
        let x1 = (tx * self.subpixel_scale() as f64).round() as i64;
        let y1 = (ty * self.subpixel_scale() as f64).round() as i64;

        let (tx, ty) = self.trans.transform(x + len as f64, y);
        let x2 = (tx * self.subpixel_scale() as f64).round() as i64;
        let y2 = (ty * self.subpixel_scale() as f64).round() as i64;
        self.li_x = Some(LineInterpolator::new(x1, x2, len as i64));
        self.li_y = Some(LineInterpolator::new(y1, y2, len as i64));
    }

    /// TODO
    #[inline]
    pub fn inc(&mut self) {
        iif![let Some(ref mut li) = self.li_x; (li).inc()];
        iif![let Some(ref mut li) = self.li_y; (li).inc()];
    }

    // TODO
    #[inline]
    #[must_use]
    pub const fn coordinates(&self) -> (i64, i64) {
        if let (Some(x), Some(y)) = (self.li_x.as_ref(), self.li_y.as_ref()) {
            (x.y, y.y)
        } else {
            panic!("Interpolator not Initialized");
        }
    }
}

impl LineParameters {
    /// Create a new Line Parameter
    #[must_use]
    pub const fn new(x1: i64, y1: i64, x2: i64, y2: i64, len: i64) -> Self {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let vertical = dy >= dx;
        let sx = if x2 > x1 { 1 } else { -1 };
        let sy = if y2 > y1 { 1 } else { -1 };
        let inc = if vertical { sy } else { sx };
        let octant = (sy & 4) as usize | (sx & 2) as usize | vertical as usize;
        Self { x1, y1, x2, y2, len, dx, dy, vertical, sx, sy, inc, octant }
    }

    /// Return the general direction of the line, see octant description
    pub const fn diagonal_quadrant(&self) -> u8 {
        let quads = [0, 1, 2, 1, 0, 3, 2, 3];
        quads[self.octant]
    }

    /// Split a Line Parameter into two parts
    pub fn divide(&self) -> (LineParameters, LineParameters) {
        let xmid = (self.x1 + self.x2) / 2;
        let ymid = (self.y1 + self.y2) / 2;
        let len2 = self.len / 2;

        let lp1 = LineParameters::new(self.x1, self.y1, xmid, ymid, len2);
        let lp2 = LineParameters::new(xmid, ymid, self.x2, self.y2, len2);

        (lp1, lp2)
    }

    /// Calculate demoninator of line-line intersection
    ///
    /// If value is small, lines are parallel or coincident
    ///
    /// - [Line-Line Intersection](https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection)
    fn fix_degenerate_bisectrix_setup(&self, x: i64, y: i64) -> i64 {
        let dx = (self.x2 - self.x1) as f64;
        let dy = (self.y2 - self.y1) as f64;
        let dx0 = (x - self.x2) as f64;
        let dy0 = (y - self.y2) as f64;
        let len = self.len as f64;
        ((dx0 * dy - dy0 * dx) / len).round() as i64
    }

    /// Move an end point bisectrix that lies on the line
    ///
    /// If point (`x`,`y`) is on the line, or sufficiently close, return a new value
    /// otherwise return the point
    ///
    /// New point:
    ///   (x2 + dy, y2 - dx)
    ///
    /// - [Bisectrix](https://en.wikipedia.org/wiki/Bisection)
    pub fn fix_degenerate_bisectrix_end(&self, x: i64, y: i64) -> (i64, i64) {
        let d = self.fix_degenerate_bisectrix_setup(x, y);
        if d < POLY_SUBPIXEL_SCALE / 2 {
            (self.x2 + (self.y2 - self.y1), self.y2 - (self.x2 - self.x1))
        } else {
            (x, y)
        }
    }
    /// Move an begin point bisectrix that lies on the line
    ///
    /// If point (`x`,`y`) is on the line, or sufficiently close, return a new value
    /// otherwise return the point
    ///
    /// New point:
    ///   (x1 + dy, y1 - dx)
    ///
    /// - [Bisectrix](https://en.wikipedia.org/wiki/Bisection)
    pub fn fix_degenerate_bisectrix_start(&self, x: i64, y: i64) -> (i64, i64) {
        let d = self.fix_degenerate_bisectrix_setup(x, y);
        if d < POLY_SUBPIXEL_SCALE / 2 {
            (self.x1 + (self.y2 - self.y1), self.y1 - (self.x2 - self.x1))
        } else {
            (x, y)
        }
    }

    /// Create a new Interpolator
    #[inline]
    #[must_use]
    pub(crate) fn interp0(&self, subpixel_width: i64) -> AA0 {
        AA0::new(*self, subpixel_width)
    }

    /// Create a new Interpolator
    #[inline]
    #[must_use]
    pub(crate) fn interp1(&self, sx: i64, sy: i64, subpixel_width: i64) -> AA1 {
        AA1::new(*self, sx, sy, subpixel_width)
    }

    /// Create a new Interpolator
    #[inline]
    #[must_use]
    pub(crate) fn interp2(&self, ex: i64, ey: i64, subpixel_width: i64) -> AA2 {
        AA2::new(*self, ex, ey, subpixel_width)
    }

    /// Create a new Interpolator
    #[inline]
    #[must_use]
    pub(crate) fn interp3(&self, sx: i64, sy: i64, ex: i64, ey: i64, subpixel_width: i64) -> AA3 {
        AA3::new(*self, sx, sy, ex, ey, subpixel_width)
    }

    /// Create a new Interpolator for an Image
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn interp_image(
        &self,
        sx: i64,
        sy: i64,
        ex: i64,
        ey: i64,
        subpixel_width: i64,
        pattern_start: i64,
        pattern_width: i64,
        scale_x: f64,
    ) -> LineInterpolatorImage {
        LineInterpolatorImage::new(
            *self,
            sx,
            sy,
            ex,
            ey,
            subpixel_width,
            pattern_start,
            pattern_width,
            scale_x,
        )
    }
}
