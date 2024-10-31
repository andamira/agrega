// agrega::interp::line_impls
//
//! Line interpolation, implementations.
//
// - LineInterpolator
// - LineInterpolatorAA
// - AA0
// - AA1
// - AA2
// - AA3

use super::{
    DistanceInterpolator, DistanceInterpolator1, DistanceInterpolator2, DistanceInterpolator3,
    LineInterpolator, LineInterpolatorAA, LineParameters, AA0, AA1, AA2, AA3,
};
use crate::{util::*, RenderOutline};
use alloc::vec;
use devela::iif;
#[allow(unused_imports)]
use devela::ExtFloat;

impl LineInterpolator {
    /// Create a new Forward Adjust Interpolator
    ///
    /// Values should be in Subpixel coordinates
    ///
    /// Error term is initialized as: `rem` - `count`
    ///
    /// `xmod`, `rem` and `left` are adjusted if `xmod` is negative
    pub fn new(y1: i64, y2: i64, count: i64) -> Self {
        let cnt = core::cmp::max(1, count);
        let mut left = (y2 - y1) / cnt;
        let mut rem = (y2 - y1) % cnt;
        let mut xmod = rem;
        let y = y1;
        iif![xmod <= 0; { xmod += count; rem += count; left -= 1 }];
        xmod -= count;
        Self { y, left, rem, xmod, count: cnt }
    }

    /// TODO
    #[inline]
    pub fn adjust_forward(&mut self) {
        self.xmod -= self.count;
    }
    // pub fn adjust_backward(&mut self) {
    //     self.xmod += self.count;
    // }

    /// Create a Forward Adjusted Interpolator
    pub fn new_foward_adjusted(y1: i64, y2: i64, count: i64) -> Self {
        Self::new(y1, y2, count)
    }

    /// Create a Back Adjusted Interpolator
    ///
    /// Assumes the First point is 0
    ///
    /// Error term is initialied as `rem`
    ///
    /// `xmod`, `rem` and `left` are adjusted if `xmod` is negative
    pub fn new_back_adjusted_2(y: i64, count: i64) -> Self {
        let cnt = core::cmp::max(1, count);
        let mut left = y / cnt;
        let mut rem = y % cnt;
        let mut xmod = rem;
        let m_y = 0;

        if xmod <= 0 {
            xmod += count;
            rem += count;
            left -= 1;
        }

        Self { y: m_y, left, rem, xmod, count: cnt }
    }

    // #[inline]
    // pub fn new_back_adjusted_1(y1: i64, y2: i64, count: i64) -> Self {
    //     let mut back = Self::new(y1, y2, count);
    //     back.count += count;
    //     back
    // }

    /// Increment the Interpolator
    #[inline]
    pub fn inc(&mut self) {
        self.xmod += self.rem;
        self.y += self.left;
        if self.xmod > 0 {
            self.xmod -= self.count;
            self.y += 1;
        }
    }
    /// Decement the Interpolator
    #[inline]
    pub fn dec(&mut self) {
        if self.xmod <= self.rem {
            self.xmod += self.count;
            self.y -= 1;
        }
        self.xmod -= self.rem;
        self.y -= self.left;
    }

    // used in tests:
    #[inline] #[must_use] #[rustfmt::skip] #[allow(dead_code)]
    pub const fn xmod(&self) -> i64 { self.xmod }
    #[inline] #[must_use] #[rustfmt::skip] #[allow(dead_code)]
    pub const fn count(&self) -> i64 { self.count }
    #[inline] #[must_use] #[rustfmt::skip] #[allow(dead_code)]
    pub const fn left(&self) -> i64 { self.left }
    #[inline] #[must_use] #[rustfmt::skip] #[allow(dead_code)]
    pub const fn rem(&self) -> i64 { self.rem }
}

impl LineInterpolatorAA {
    /// Create new Line Interpolator AA
    pub fn new(lp: LineParameters, subpixel_width: i64) -> Self {
        let len = if lp.vertical == (lp.inc > 0) {
            -lp.len
        } else {
            lp.len
        };
        let x = lp.x1 >> POLY_SUBPIXEL_SHIFT;
        let y = lp.y1 >> POLY_SUBPIXEL_SHIFT;
        let old_x = x;
        let old_y = y;
        let count = if lp.vertical {
            ((lp.y2 >> POLY_SUBPIXEL_SHIFT) - y).abs()
        } else {
            ((lp.x2 >> POLY_SUBPIXEL_SHIFT) - x).abs()
        };
        let width = subpixel_width;
        let max_extent = (width + POLY_SUBPIXEL_MASK) >> POLY_SUBPIXEL_SHIFT;
        let step = 0;
        let y1 = if lp.vertical {
            (lp.x2 - lp.x1) << POLY_SUBPIXEL_SHIFT
        } else {
            (lp.y2 - lp.y1) << POLY_SUBPIXEL_SHIFT
        };
        let n = if lp.vertical {
            (lp.y2 - lp.y1).abs()
        } else {
            (lp.x2 - lp.x1).abs() + 1
        };

        // Setup Number Interpolator from 0 .. y1 with n segments
        let m_li = LineInterpolator::new_back_adjusted_2(y1, n);

        // Length of line in subpixels
        let mut dd = if lp.vertical { lp.dy } else { lp.dx };
        dd <<= POLY_SUBPIXEL_SHIFT; // to subpixels
        let mut li = LineInterpolator::new_foward_adjusted(0, dd, lp.len);

        // Get Distances along the line
        let mut dist = vec![0i64; MAX_HALF_WIDTH + 1];
        let stop = width + POLY_SUBPIXEL_SCALE * 2;
        for item in dist.iter_mut().take(MAX_HALF_WIDTH) {
            *item = li.y;
            if li.y >= stop {
                break;
            }
            li.inc();
        }
        dist[MAX_HALF_WIDTH] = 0x7FFF_0000;
        // Setup covers to 0
        let covers = vec![0u64; MAX_HALF_WIDTH * 2 + 4];
        Self {
            lp,
            li: m_li,
            len,
            x,
            y,
            old_x,
            old_y,
            count,
            width,
            max_extent,
            step,
            dist,
            covers,
        }
    }
    /// Step the Line forward horizontally
    pub(crate) fn step_hor_base<DI>(&mut self, di: &mut DI) -> i64
    where
        DI: DistanceInterpolator,
    {
        // Increment the Interpolator
        self.li.inc();
        // Increment the x by the LineParameter increment, typically +1 or -1
        self.x += self.lp.inc;
        // Set y value to initial + new y value
        self.y = (self.lp.y1 + self.li.y) >> POLY_SUBPIXEL_SHIFT;
        // "Increment" the distance interpolator
        if self.lp.inc > 0 {
            di.inc_x(self.y - self.old_y);
        } else {
            di.dec_x(self.y - self.old_y);
        }
        // Save current point
        self.old_y = self.y;
        // Return some measure of distance
        di.dist() / self.len
    }
    pub(crate) fn step_ver_base<DI>(&mut self, di: &mut DI) -> i64
    where
        DI: DistanceInterpolator,
    {
        self.li.inc();
        self.y += self.lp.inc;
        self.x = (self.lp.x1 + self.li.y) >> POLY_SUBPIXEL_SHIFT;

        if self.lp.inc > 0 {
            di.inc_y(self.x - self.old_x);
        } else {
            di.dec_y(self.x - self.old_x);
        }

        self.old_x = self.x;
        di.dist() / self.len
    }
}

impl AA0 {
    /// Create a new Line Interpolator-0
    #[rustfmt::skip]
    pub fn new(lp: LineParameters, subpixel_width: i64) -> Self {
        let mut li = LineInterpolatorAA::new(lp, subpixel_width);
        li.li.adjust_forward();
        Self {
            li, di: DistanceInterpolator1::new(lp.x1, lp.y1, lp.x2, lp.y2,
            lp.x1 & !POLY_SUBPIXEL_MASK, lp.y1 & !POLY_SUBPIXEL_MASK),
        }
    }
    /// Size of the Interpolation
    #[inline]
    #[must_use]
    pub fn count(&self) -> i64 {
        self.li.count
    }
    /// Return if the line is more Vertical than horizontal
    #[inline]
    #[must_use]
    pub fn vertical(&self) -> bool {
        self.li.lp.vertical
    }

    /// Conduct a horizontal step, used for "horizontal lines"
    pub fn step_hor<R: RenderOutline>(&mut self, ren: &mut R) -> bool {
        // Step the Interpolator horizontally and get the width
        //   projected onto the vertical
        let s1 = self.li.step_hor_base(&mut self.di);
        let mut p0 = MAX_HALF_WIDTH + 2;
        let mut p1 = p0;

        // Get the coverage at the center for value of s1
        self.li.covers[p1] = ren.cover(s1);

        p1 += 1;
        //Generate covers for "one" side of the line
        let mut dy = 1;
        let mut dist = self.li.dist[dy] - s1;
        while dist <= self.li.width {
            self.li.covers[p1] = ren.cover(dist);
            p1 += 1;
            dy += 1;
            dist = self.li.dist[dy] - s1;
        }
        //Generate covers for the "other" side of the line
        let mut dy = 1;
        dist = self.li.dist[dy] + s1;
        while dist <= self.li.width {
            p0 -= 1;
            self.li.covers[p0] = ren.cover(dist);
            dy += 1;
            dist = self.li.dist[dy] + s1;
        }
        // Draw Line using coverages
        ren.blend_solid_vspan(
            self.li.x,
            self.li.y - dy as i64 + 1,
            (p1 - p0) as i64,
            &self.li.covers[p0..],
        );
        // Step the Line Interpolator AA
        self.li.step += 1;
        self.li.step < self.li.count
    }
    /// Conduct a vertical step, used for "vertical lines"
    pub fn step_ver<R: RenderOutline>(&mut self, ren: &mut R) -> bool {
        let s1 = self.li.step_ver_base(&mut self.di);
        let mut p0 = MAX_HALF_WIDTH + 2;
        let mut p1 = p0;
        self.li.covers[p1] = ren.cover(s1);
        p1 += 1;
        let mut dx = 1;
        let mut dist = self.li.dist[dx] - s1;
        while dist <= self.li.width {
            self.li.covers[p1] = ren.cover(dist);
            p1 += 1;
            dx += 1;
            dist = self.li.dist[dx] - s1;
        }

        dx = 1;
        dist = self.li.dist[dx] + s1;
        while dist <= self.li.width {
            p0 -= 1;
            self.li.covers[p0] = ren.cover(dist);
            dx += 1;
            dist = self.li.dist[dx] + s1;
        }
        ren.blend_solid_hspan(
            self.li.x - dx as i64 + 1,
            self.li.y,
            (p1 - p0) as i64,
            &self.li.covers[p0..],
        );
        self.li.step += 1;
        self.li.step < self.li.count
    }
}

impl AA1 {
    #[rustfmt::skip]
    pub fn new(lp: LineParameters, sx: i64, sy: i64, subpixel_width: i64) -> Self {
        let mut li = LineInterpolatorAA::new(lp, subpixel_width);
        let mut di = DistanceInterpolator2::new(
            lp.x1, lp.y1, lp.x2, lp.y2, sx, sy,
            lp.x1 & !POLY_SUBPIXEL_MASK, lp.y1 & !POLY_SUBPIXEL_MASK, true,
        );
        let mut npix = 1;
        if lp.vertical {
            loop {
                li.li.dec();
                li.y -= lp.inc;
                li.x = (li.lp.x1 + li.li.y) >> POLY_SUBPIXEL_SHIFT;

                if lp.inc > 0 {
                    di.dec_y(li.x - li.old_x);
                } else {
                    di.inc_y(li.x - li.old_x);
                }
                li.old_x = li.x;

                let mut dist1_start = di.dist_start;
                let mut dist2_start = di.dist_start;

                let mut dx = 0;
                iif![dist1_start < 0; npix += 1];
                loop {
                    dist1_start += di.dy_start;
                    dist2_start -= di.dy_start;
                    if dist1_start < 0 {
                        npix += 1;
                    }
                    iif![dist2_start < 0; npix += 1];
                    dx += 1;
                    iif![li.dist[dx] > li.width; break];
                }
                li.step -= 1;
                iif![npix == 0; break];
                npix = 0;
                iif![li.step < -li.max_extent; break];
            }
        } else {
            loop {
                li.li.dec();
                li.x -= lp.inc;
                li.y = (li.lp.y1 + li.li.y) >> POLY_SUBPIXEL_SHIFT;
                if lp.inc > 0 {
                    di.dec_x(li.y - li.old_y);
                } else {
                    di.inc_x(li.y - li.old_y);
                }
                li.old_y = li.y;

                let mut dist1_start = di.dist_start;
                let mut dist2_start = di.dist_start;

                let mut dy = 0;
                if dist1_start < 0 {
                    npix += 1;
                }
                loop {
                    dist1_start -= di.dx_start;
                    dist2_start += di.dx_start;
                    if dist1_start < 0 {
                        npix += 1;
                    }
                    if dist2_start < 0 {
                        npix += 1;
                    }
                    dy += 1;
                    if li.dist[dy] > li.width {
                        break;
                    }
                }
                li.step -= 1;
                if npix == 0 {
                    break;
                }
                npix = 0;
                if li.step < -li.max_extent {
                    break;
                }
            }
        }
        li.li.adjust_forward();
        Self { li, di }
    }
    //pub fn count(&self) -> i64 {        self.li.count    }
    pub fn vertical(&self) -> bool {
        self.li.lp.vertical
    }
    pub fn step_hor<R: RenderOutline>(&mut self, ren: &mut R) -> bool {
        let s1 = self.li.step_hor_base(&mut self.di);

        let mut dist_start = self.di.dist_start;
        let mut p0 = MAX_HALF_WIDTH + 2;
        let mut p1 = p0;
        self.li.covers[p1] = 0;
        if dist_start <= 0 {
            self.li.covers[p1] = ren.cover(s1);
        }
        p1 += 1;
        let mut dy = 1;
        let mut dist = self.li.dist[dy] - s1;
        while dist <= self.li.width {
            dist_start -= self.di.dx_start;
            self.li.covers[p1] = 0;
            if dist_start <= 0 {
                self.li.covers[p1] = ren.cover(dist);
            }
            p1 += 1;
            dy += 1;
            dist = self.li.dist[dy] - s1;
        }

        dy = 1;
        dist_start = self.di.dist_start;
        dist = self.li.dist[dy] + s1;
        while dist <= self.li.width {
            dist_start += self.di.dx_start;
            p0 -= 1;
            self.li.covers[p0] = 0;
            if dist_start <= 0 {
                self.li.covers[p0] = ren.cover(dist);
            }
            dy += 1;
            dist = self.li.dist[dy] + s1;
        }
        ren.blend_solid_vspan(
            self.li.x,
            self.li.y - dy as i64 + 1,
            (p1 - p0) as i64,
            &self.li.covers[p0..],
        );
        self.li.step += 1;
        self.li.step < self.li.count
    }
    pub fn step_ver<R: RenderOutline>(&mut self, ren: &mut R) -> bool {
        let s1 = self.li.step_ver_base(&mut self.di);
        let mut p0 = MAX_HALF_WIDTH + 2;
        let mut p1 = p0;

        let mut dist_start = self.di.dist_start;
        self.li.covers[p1] = 0;
        if dist_start <= 0 {
            self.li.covers[p1] = ren.cover(s1);
        }
        p1 += 1;
        let mut dx = 1;
        let mut dist = self.li.dist[dx] - s1;
        while dist <= self.li.width {
            dist_start += self.di.dy_start;
            self.li.covers[p1] = 0;
            if dist_start <= 0 {
                self.li.covers[p1] = ren.cover(dist);
            }
            p1 += 1;
            dx += 1;
            dist = self.li.dist[dx] - s1;
        }
        dx = 1;
        dist_start = self.di.dist_start;
        dist = self.li.dist[dx] + s1;
        while dist <= self.li.width {
            dist_start -= self.di.dy_start;
            p0 -= 1;
            self.li.covers[p0] = 0;
            if dist_start <= 0 {
                self.li.covers[p0] = ren.cover(dist);
            }
            dx += 1;
            dist = self.li.dist[dx] + s1;
        }
        ren.blend_solid_hspan(
            self.li.x - dx as i64 + 1,
            self.li.y,
            (p1 - p0) as i64,
            &self.li.covers[p0..],
        );
        self.li.step += 1;
        self.li.step < self.li.count
    }
}

impl AA2 {
    pub fn new(lp: LineParameters, ex: i64, ey: i64, subpixel_width: i64) -> Self {
        let mut li = LineInterpolatorAA::new(lp, subpixel_width);
        let di = DistanceInterpolator2::new(
            lp.x1,
            lp.y1,
            lp.x2,
            lp.y2,
            ex,
            ey,
            lp.x1 & !POLY_SUBPIXEL_MASK,
            lp.y1 & !POLY_SUBPIXEL_MASK,
            false,
        );
        li.li.adjust_forward();
        li.step -= li.max_extent;
        Self { li, di }
    }
    //pub fn count(&self) -> i64 {        self.li.count    }
    pub fn vertical(&self) -> bool {
        self.li.lp.vertical
    }
    pub fn step_hor<R: RenderOutline>(&mut self, ren: &mut R) -> bool {
        let s1 = self.li.step_hor_base(&mut self.di);
        let mut p0 = MAX_HALF_WIDTH + 2;
        let mut p1 = p0;

        let mut dist_end = self.di.dist_start;

        let mut npix = 0;
        self.li.covers[p1] = 0;
        if dist_end > 0 {
            self.li.covers[p1] = ren.cover(s1);
            npix += 1;
        }
        p1 += 1;

        let mut dy = 1;
        let mut dist = self.li.dist[dy] - s1;
        while dist <= self.li.width {
            dist_end -= self.di.dx_start;
            self.li.covers[p1] = 0;
            if dist_end > 0 {
                self.li.covers[p1] = ren.cover(dist);
                npix += 1;
            }
            p1 += 1;
            dy += 1;
            dist = self.li.dist[dy] - s1;
        }

        dy = 1;
        dist_end = self.di.dist_start;
        dist = self.li.dist[dy] + s1;
        while dist <= self.li.width {
            dist_end += self.di.dx_start;
            p0 -= 1;
            self.li.covers[p0] = 0;
            if dist_end > 0 {
                self.li.covers[p0] = ren.cover(dist);
                npix += 1;
            }
            dy += 1;
            dist = self.li.dist[dy] + s1;
        }
        ren.blend_solid_vspan(
            self.li.x,
            self.li.y - dy as i64 + 1,
            (p1 - p0) as i64,
            &self.li.covers[p0..],
        );
        self.li.step += 1;
        npix != 0 && self.li.step < self.li.count
    }
    pub fn step_ver<R: RenderOutline>(&mut self, ren: &mut R) -> bool {
        let s1 = self.li.step_ver_base(&mut self.di);
        let mut p0 = MAX_HALF_WIDTH + 2;
        let mut p1 = p0;

        let mut dist_end = self.di.dist_start; // Really dist_end

        let mut npix = 0;
        self.li.covers[p1] = 0;
        if dist_end > 0 {
            self.li.covers[p1] = ren.cover(s1);
            npix += 1;
        }
        p1 += 1;

        let mut dx = 1;
        let mut dist = self.li.dist[dx] - s1;
        while dist <= self.li.width {
            dist_end += self.di.dy_start;
            self.li.covers[p1] = 0;
            if dist_end > 0 {
                self.li.covers[p1] = ren.cover(dist);
                npix += 1;
            }
            p1 += 1;
            dx += 1;
            dist = self.li.dist[dx] - s1;
        }

        dx = 1;
        dist_end = self.di.dist_start;
        dist = self.li.dist[dx] + s1;
        while dist <= self.li.width {
            dist_end -= self.di.dy_start;
            p0 -= 1;
            self.li.covers[p0] = 0;
            if dist_end > 0 {
                self.li.covers[p0] = ren.cover(dist);
                npix += 1;
            }
            dx += 1;
            dist = self.li.dist[dx] + s1;
        }
        ren.blend_solid_hspan(
            self.li.x - dx as i64 + 1,
            self.li.y,
            (p1 - p0) as i64,
            &self.li.covers[p0..],
        );
        self.li.step += 1;
        npix != 0 && self.li.step < self.li.count
    }
}

impl AA3 {
    pub fn new(
        lp: LineParameters,
        sx: i64,
        sy: i64,
        ex: i64,
        ey: i64,
        subpixel_width: i64,
    ) -> Self {
        let mut li = LineInterpolatorAA::new(lp, subpixel_width);
        let mut di = DistanceInterpolator3::new(
            lp.x1,
            lp.y1,
            lp.x2,
            lp.y2,
            sx,
            sy,
            ex,
            ey,
            lp.x1 & !POLY_SUBPIXEL_MASK,
            lp.y1 & !POLY_SUBPIXEL_MASK,
        );
        let mut npix = 1;
        if lp.vertical {
            loop {
                li.li.dec();
                li.y -= lp.inc;
                li.x = (li.lp.x1 + li.li.y) >> POLY_SUBPIXEL_SHIFT;

                if lp.inc > 0 {
                    di.dec_y(li.x - li.old_x);
                } else {
                    di.inc_y(li.x - li.old_x);
                }

                li.old_x = li.x;

                let mut dist1_start = di.dist_start;
                let mut dist2_start = di.dist_start;

                let mut dx = 0;
                if dist1_start < 0 {
                    npix += 1;
                }
                loop {
                    dist1_start += di.dy_start;
                    dist2_start -= di.dy_start;
                    if dist1_start < 0 {
                        npix += 1;
                    }
                    if dist2_start < 0 {
                        npix += 1;
                    }
                    dx += 1;
                    if li.dist[dx] > li.width {
                        break;
                    }
                }
                if npix == 0 {
                    break;
                }
                npix = 0;
                li.step -= 1;
                if li.step < -li.max_extent {
                    break;
                }
            }
        } else {
            loop {
                li.li.dec();
                li.x -= lp.inc;
                li.y = (li.lp.y1 + li.li.y) >> POLY_SUBPIXEL_SHIFT;

                if lp.inc > 0 {
                    di.dec_x(li.y - li.old_y);
                } else {
                    di.inc_x(li.y - li.old_y);
                }

                li.old_y = li.y;

                let mut dist1_start = di.dist_start;
                let mut dist2_start = di.dist_start;

                let mut dy = 0;
                if dist1_start < 0 {
                    npix += 1;
                }
                loop {
                    dist1_start -= di.dx_start;
                    dist2_start += di.dx_start;
                    if dist1_start < 0 {
                        npix += 1;
                    }
                    if dist2_start < 0 {
                        npix += 1;
                    }
                    dy += 1;
                    if li.dist[dy] > li.width {
                        break;
                    }
                }
                if npix == 0 {
                    break;
                }
                npix = 0;
                li.step -= 1;
                if li.step < -li.max_extent {
                    break;
                }
            }
        }
        li.li.adjust_forward();
        li.step -= li.max_extent;
        Self { li, di }
    }
    //pub fn count(&self) -> i64 {        self.li.count    }
    pub fn vertical(&self) -> bool {
        self.li.lp.vertical
    }
    pub fn step_hor<R: RenderOutline>(&mut self, ren: &mut R) -> bool {
        let s1 = self.li.step_hor_base(&mut self.di);
        let mut p0 = MAX_HALF_WIDTH + 2;
        let mut p1 = p0;

        let mut dist_start = self.di.dist_start;
        let mut dist_end = self.di.dist_end;

        let mut npix = 0;
        self.li.covers[p1] = 0;
        if dist_end > 0 {
            if dist_start <= 0 {
                self.li.covers[p1] = ren.cover(s1);
            }
            npix += 1;
        }
        p1 += 1;

        let mut dy = 1;
        let mut dist = self.li.dist[dy] - s1;
        while dist <= self.li.width {
            dist_start -= self.di.dx_start;
            dist_end -= self.di.dx_end;
            self.li.covers[p1] = 0;
            if dist_end > 0 && dist_start <= 0 {
                self.li.covers[p1] = ren.cover(dist);
                npix += 1;
            }
            p1 += 1;
            dy += 1;
            dist = self.li.dist[dy] - s1;
        }

        dy = 1;
        dist_start = self.di.dist_start;
        dist_end = self.di.dist_end;
        dist = self.li.dist[dy] + s1;
        while dist <= self.li.width {
            dist_start += self.di.dx_start;
            dist_end += self.di.dx_end;
            p0 -= 1;
            self.li.covers[p0] = 0;
            if dist_end > 0 && dist_start <= 0 {
                self.li.covers[p0] = ren.cover(dist);
                npix += 1;
            }
            dy += 1;
        }
        ren.blend_solid_vspan(
            self.li.x,
            self.li.y - dy as i64 + 1,
            (p1 - p0) as i64,
            &self.li.covers[p0..],
        );
        self.li.step -= 1;
        npix != 0 && self.li.step < self.li.count
    }
    pub fn step_ver<R: RenderOutline>(&mut self, ren: &mut R) -> bool {
        let s1 = self.li.step_ver_base(&mut self.di);
        let mut p0 = MAX_HALF_WIDTH + 2;
        let mut p1 = p0;

        let mut dist_start = self.di.dist_start;
        let mut dist_end = self.di.dist_end;

        let mut npix = 0;
        self.li.covers[p1] = 0;
        if dist_end > 0 {
            if dist_start <= 0 {
                self.li.covers[p1] = ren.cover(s1);
            }
            npix += 1;
        }
        p1 += 1;

        let mut dx = 1;
        let mut dist = self.li.dist[dx] - s1;
        while dist <= self.li.width {
            dist_start += self.di.dy_start;
            dist_end += self.di.dy_end;
            self.li.covers[p1] = 0;
            if dist_end > 0 && dist_start <= 0 {
                self.li.covers[p1] = ren.cover(dist);
                npix += 1;
            }
            p1 += 1;
            dx += 1;
            dist = self.li.dist[dx] - s1;
        }

        dx = 1;
        dist_start = self.di.dist_start;
        dist_end = self.di.dist_end;
        dist = self.li.dist[dx] + s1;
        while dist <= self.li.width {
            dist_start -= self.di.dy_start;
            dist_end -= self.di.dy_end;
            p0 -= 1;
            self.li.covers[p0] = 0;
            if dist_end > 0 && dist_start <= 0 {
                self.li.covers[p0] = ren.cover(dist);
                npix += 1;
            }
            dx += 1;
            dist = self.li.dist[dx] + s1;
        }
        ren.blend_solid_hspan(
            self.li.x - dx as i64 + 1,
            self.li.y,
            (p1 - p0) as i64,
            &self.li.covers[p0..p1],
        );
        self.li.step -= 1;
        npix != 0 && self.li.step < self.li.count
    }
}
