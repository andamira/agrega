// agrega::outline::aa_rend

use crate::{
    base::RenderingBase,
    clip::Rectangle,
    color::Rgba8,
    interp::{DistanceInterpolator0, DistanceInterpolator00, LineParameters},
    raster::len_i64_xy,
    render::{clip_line_segment, LINE_MAX_LENGTH},
    Color, DrawOutline, Pixel, RenderOutline, MAX_HALF_WIDTH, POLY_SUBPIXEL_MASK,
    POLY_SUBPIXEL_SCALE, POLY_SUBPIXEL_SHIFT,
};
use alloc::{vec, vec::Vec};
#[allow(unused_imports)]
use devela::ExtFloat;

#[derive(Debug)]
/// Outline Renderer with Anti-Aliasing
pub struct RendererOutlineAA<'a, T> {
    ren: &'a mut RenderingBase<T>,
    color: Rgba8,
    clip_box: Option<Rectangle<i64>>,
    profile: LineProfileAA,
}

impl<'a, T: Pixel> RendererOutlineAA<'a, T> {
    /// Create Outline Renderer with a [`RenderingBase`](../base/struct.RenderingBase.html)
    pub fn with_base(ren: &'a mut RenderingBase<T>) -> Self {
        let profile = LineProfileAA::new();
        Self { ren, color: Rgba8::black(), clip_box: None, profile }
    }
    /// Set width of the line
    pub fn width(&mut self, width: f64) {
        self.profile.width(width);
    }
    /// Set minimum with of the line
    ///
    /// Use [`width`](#method.width) for this to take effect
    pub fn min_width(&mut self, width: f64) {
        self.profile.min_width(width);
    }
    /// Set smoother width of the line
    ///
    /// Use [`width`](#method.width) for this to take effect
    pub fn smoother_width(&mut self, width: f64) {
        self.profile.smoother_width(width);
    }

    fn subpixel_width(&self) -> i64 {
        self.profile.subpixel_width
    }

    /// Draw a Line Segment
    ///
    /// If line to "too long", divide it by two and draw both segments
    /// otherwise, interpolate along the line to draw
    ///
    fn line0_no_clip(&mut self, lp: &LineParameters) {
        if lp.len > LINE_MAX_LENGTH {
            let (lp1, lp2) = lp.divide();
            self.line0_no_clip(&lp1);
            self.line0_no_clip(&lp2);
            return;
        }
        let mut li = lp.interp0(self.subpixel_width());
        if li.count() > 0 {
            if li.vertical() {
                while li.step_ver(self) {}
            } else {
                while li.step_hor(self) {}
            }
        }
    }
    fn line1_no_clip(&mut self, lp: &LineParameters, sx: i64, sy: i64) {
        if lp.len > LINE_MAX_LENGTH {
            let (lp1, lp2) = lp.divide();
            self.line1_no_clip(&lp1, (lp.x1 + sx) >> 1, (lp.y1 + sy) >> 1);
            self.line1_no_clip(&lp2, lp.x2 + (lp.y1 + lp1.y1), lp1.y2 - (lp1.x2 - lp1.x1));
            return;
        }
        let (sx, sy) = lp.fix_degenerate_bisectrix_start(sx, sy);
        let mut li = lp.interp1(sx, sy, self.subpixel_width());
        if li.vertical() {
            while li.step_ver(self) {}
        } else {
            while li.step_hor(self) {}
        }
    }
    fn line2_no_clip(&mut self, lp: &LineParameters, ex: i64, ey: i64) {
        if lp.len > LINE_MAX_LENGTH {
            let (lp1, lp2) = lp.divide();
            self.line2_no_clip(&lp1, lp1.x2 + (lp1.y2 - lp1.y1), lp1.y2 - (lp1.x2 - lp1.x1));
            self.line2_no_clip(&lp2, (lp.x2 + ex) >> 1, (lp.y2 + ey) >> 1);
            return;
        }
        let (ex, ey) = lp.fix_degenerate_bisectrix_end(ex, ey);
        let mut li = lp.interp2(ex, ey, self.subpixel_width());
        if li.vertical() {
            while li.step_ver(self) {}
        } else {
            while li.step_hor(self) {}
        }
    }
    fn line3_no_clip(&mut self, lp: &LineParameters, sx: i64, sy: i64, ex: i64, ey: i64) {
        if lp.len > LINE_MAX_LENGTH {
            let (lp1, lp2) = lp.divide();
            let mx = lp1.x2 + (lp1.y2 - lp1.y1);
            let my = lp1.y2 - (lp1.x2 - lp1.x1);
            self.line3_no_clip(&lp1, (lp.x1 + sx) >> 1, (lp.y1 + sy) >> 1, mx, my);
            self.line3_no_clip(&lp2, mx, my, (lp.x2 + ex) >> 1, (lp.y2 + ey) >> 1);
            return;
        }
        let (sx, sy) = lp.fix_degenerate_bisectrix_start(sx, sy);
        let (ex, ey) = lp.fix_degenerate_bisectrix_end(ex, ey);
        let mut li = lp.interp3(sx, sy, ex, ey, self.subpixel_width());
        if li.vertical() {
            while li.step_ver(self) {}
        } else {
            while li.step_hor(self) {}
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn semidot_hline<F>(
        &mut self,
        cmp: F,
        xc1: i64,
        yc1: i64,
        xc2: i64,
        yc2: i64,
        x1: i64,
        y1: i64,
        x2: i64,
    ) where
        F: Fn(i64) -> bool,
    {
        let mut x1 = x1;
        let mut covers = [0u64; MAX_HALF_WIDTH * 2 + 4];
        let p0 = 0;
        let mut p1 = 0;
        let mut x = x1 << POLY_SUBPIXEL_SHIFT;
        let mut y = y1 << POLY_SUBPIXEL_SHIFT;
        let w = self.subpixel_width();

        let mut di = DistanceInterpolator0::new(xc1, yc1, xc2, yc2, x, y);
        x += POLY_SUBPIXEL_SCALE / 2;
        y += POLY_SUBPIXEL_SCALE / 2;

        let x0 = x1;
        let mut dx = x - xc1;
        let dy = y - yc1;
        loop {
            let d = ((dx * dx + dy * dy) as f64).sqrt() as i64;
            covers[p1] = 0;
            if cmp(di.dist) && d <= w {
                covers[p1] = self.cover(d);
            }
            p1 += 1;
            dx += POLY_SUBPIXEL_SCALE;
            di.inc_x();
            x1 += 1;
            if x1 > x2 {
                break;
            }
        }
        self.ren.blend_solid_hspan(x0, y1, (p1 - p0) as i64, self.color, &covers);
    }

    #[allow(clippy::too_many_arguments)]
    fn pie_hline(
        &mut self,
        xc: i64,
        yc: i64,
        xp1: i64,
        yp1: i64,
        xp2: i64,
        yp2: i64,
        xh1: i64,
        yh1: i64,
        xh2: i64,
    ) {
        if let Some(clip_box) = self.clip_box {
            if clip_box.clip_flags_i64(xc, yc) != 0 {
                return;
            }
        }
        let mut xh1 = xh1;
        let mut covers = [0u64; MAX_HALF_WIDTH * 2 + 4];

        let p0 = 0;
        let mut p1 = 0;
        let mut x = xh1 << POLY_SUBPIXEL_SHIFT;
        let mut y = yh1 << POLY_SUBPIXEL_SHIFT;
        let w = self.subpixel_width();

        let mut di = DistanceInterpolator00::new(xc, yc, xp1, yp1, xp2, yp2, x, y);
        x += POLY_SUBPIXEL_SCALE / 2;
        y += POLY_SUBPIXEL_SCALE / 2;

        let xh0 = xh1;
        let mut dx = x - xc;
        let dy = y - yc;
        loop {
            let d = ((dx * dx + dy * dy) as f64).sqrt() as i64;
            covers[p1] = 0;
            if di.dist1 <= 0 && di.dist2 > 0 && d <= w {
                covers[p1] = self.cover(d);
            }
            p1 += 1;
            dx += POLY_SUBPIXEL_SCALE;
            di.inc_x();
            xh1 += 1;
            if xh1 > xh2 {
                break;
            }
        }
        self.ren.blend_solid_hspan(xh0, yh1, (p1 - p0) as i64, self.color, &covers);
    }
}

impl<T: Pixel> RenderOutline for RendererOutlineAA<'_, T> {
    fn cover(&self, d: i64) -> u64 {
        let subpixel_shift = POLY_SUBPIXEL_SHIFT;
        let subpixel_scale = 1 << subpixel_shift;
        let index = d + i64::from(subpixel_scale) * 2;
        assert!(index >= 0);
        u64::from(self.profile.profile[index as usize])
    }
    fn blend_solid_hspan(&mut self, x: i64, y: i64, len: i64, covers: &[u64]) {
        self.ren.blend_solid_hspan(x, y, len, self.color, covers);
    }
    fn blend_solid_vspan(&mut self, x: i64, y: i64, len: i64, covers: &[u64]) {
        self.ren.blend_solid_vspan(x, y, len, self.color, covers);
    }
}

impl<T: Pixel> DrawOutline for RendererOutlineAA<'_, T> {
    fn line3(&mut self, lp: &LineParameters, sx: i64, sy: i64, ex: i64, ey: i64) {
        if let Some(clip_box) = self.clip_box {
            let (x1, y1, x2, y2, flags) = clip_line_segment(lp.x1, lp.y1, lp.x2, lp.y2, clip_box);
            if (flags & 4) == 0 {
                let (mut sx, mut sy, mut ex, mut ey) = (sx, sy, ex, ey);
                if flags != 0 {
                    let lp2 = LineParameters::new(x1, y1, x2, y2, len_i64_xy(x1, y1, x2, y2));
                    if flags & 1 != 0 {
                        sx = x1 + (y2 - y1);
                        sy = y1 - (x2 - x1);
                    } else {
                        while (sx - lp.x1).abs() + (sy - lp.y1).abs() > lp2.len {
                            sx = (lp.x1 + sx) >> 1;
                            sy = (lp.y1 + sy) >> 1;
                        }
                    }
                    if flags & 2 != 0 {
                        ex = x2 + (y2 - y1);
                        ey = y2 - (x2 - x1);
                    } else {
                        while (ex - lp.x2).abs() + (ey - lp.y2).abs() > lp2.len {
                            ex = (lp.x2 + ex) >> 1;
                            ey = (lp.y2 + ey) >> 1;
                        }
                    }
                    self.line3_no_clip(&lp2, sx, sy, ex, ey);
                } else {
                    self.line3_no_clip(lp, sx, sy, ex, ey);
                }
            }
        } else {
            self.line3_no_clip(lp, sx, sy, ex, ey);
        }
    }
    fn semidot<F>(&mut self, cmp: F, xc1: i64, yc1: i64, xc2: i64, yc2: i64)
    where
        F: Fn(i64) -> bool,
    {
        if let Some(clip_box) = self.clip_box {
            if clip_box.clip_flags_i64(xc1, yc1) != 0 {
                return;
            }
        }

        let mut r = (self.subpixel_width() + POLY_SUBPIXEL_MASK) >> POLY_SUBPIXEL_SHIFT;
        if r < 1 {
            r = 1;
        }
        let mut ei = EllipseInterpolator::new(r, r);
        let mut dx = 0;
        let mut dy = -r;
        let mut dy0 = dy;
        let mut dx0 = dx;
        let x = xc1 >> POLY_SUBPIXEL_SHIFT;
        let y = yc1 >> POLY_SUBPIXEL_SHIFT;

        loop {
            dx += ei.dx;
            dy += ei.dy;

            if dy != dy0 {
                self.semidot_hline(&cmp, xc1, yc1, xc2, yc2, x - dx0, y + dy0, x + dx0);
                self.semidot_hline(&cmp, xc1, yc1, xc2, yc2, x - dx0, y - dy0, x + dx0);
            }
            dx0 = dx;
            dy0 = dy;
            ei.inc();
            if dy >= 0 {
                break;
            }
        }
        self.semidot_hline(&cmp, xc1, yc1, xc2, yc2, x - dx0, y + dy0, x + dx0);
    }

    fn pie(&mut self, xc: i64, yc: i64, x1: i64, y1: i64, x2: i64, y2: i64) {
        let mut r = (self.subpixel_width() + POLY_SUBPIXEL_MASK) >> POLY_SUBPIXEL_SHIFT;
        if r < 1 {
            r = 1;
        }
        let mut ei = EllipseInterpolator::new(r, r);
        let mut dx = 0;
        let mut dy = -r;
        let mut dy0 = dy;
        let mut dx0 = dx;
        let x = xc >> POLY_SUBPIXEL_SHIFT;
        let y = yc >> POLY_SUBPIXEL_SHIFT;

        loop {
            dx += ei.dx;
            dy += ei.dy;

            if dy != dy0 {
                self.pie_hline(xc, yc, x1, y1, x2, y2, x - dx0, y + dy0, x + dx0);
                self.pie_hline(xc, yc, x1, y1, x2, y2, x - dx0, y - dy0, x + dx0);
            }
            dx0 = dx;
            dy0 = dy;
            ei.inc();
            if dy >= 0 {
                break;
            }
        }
        self.pie_hline(xc, yc, x1, y1, x2, y2, x - dx0, y + dy0, x + dx0);
    }
    /// Draw a Line Segment, clipping if necessary
    ///
    fn line0(&mut self, lp: &LineParameters) {
        if let Some(clip_box) = self.clip_box {
            let (x1, y1, x2, y2, flags) = clip_line_segment(lp.x1, lp.y1, lp.x2, lp.y2, clip_box);
            if flags & 4 == 0 {
                // Line in Visible
                if flags != 0 {
                    // Line is Clipped
                    // Create new Line from clipped lines and draw
                    let lp2 = LineParameters::new(x1, y1, x2, y2, len_i64_xy(x1, y1, x2, y2));
                    self.line0_no_clip(&lp2);
                } else {
                    // Line is not Clipped
                    self.line0_no_clip(lp)
                }
            }
        } else {
            // No clip box defined
            self.line0_no_clip(lp);
        }
    }
    fn line1(&mut self, lp: &LineParameters, sx: i64, sy: i64) {
        if let Some(clip_box) = self.clip_box {
            let (x1, y1, x2, y2, flags) = clip_line_segment(lp.x1, lp.y1, lp.x2, lp.y2, clip_box);
            if flags & 4 == 0 {
                if flags != 0 {
                    let (mut sx, mut sy) = (sx, sy);
                    let lp2 = LineParameters::new(x1, y1, x2, y2, len_i64_xy(x1, y1, x2, y2));
                    if flags & 1 == 0 {
                        sx = x1 + (y2 - y1);
                        sy = y1 - (x2 - x1);
                    } else {
                        while (sx - lp.x1).abs() + (sy - lp.y1).abs() > lp2.len {
                            sx = (lp.x1 + sx) >> 1;
                            sy = (lp.y1 + sy) >> 1;
                        }
                    }
                    self.line1_no_clip(&lp2, sx, sy);
                } else {
                    self.line1_no_clip(lp, sx, sy);
                }
            }
        } else {
            self.line1_no_clip(lp, sx, sy);
        }
    }
    fn line2(&mut self, lp: &LineParameters, ex: i64, ey: i64) {
        if let Some(clip_box) = self.clip_box {
            let (x1, y1, x2, y2, flags) = clip_line_segment(lp.x1, lp.y1, lp.x2, lp.y2, clip_box);
            if flags & 4 == 0 {
                if flags != 0 {
                    let (mut ex, mut ey) = (ex, ey);
                    let lp2 = LineParameters::new(x1, y1, x2, y2, len_i64_xy(x1, y1, x2, y2));
                    if flags & 2 != 0 {
                        ex = x2 + (y2 - y1);
                        ey = y2 + (x2 - x1);
                    } else {
                        while (ex - lp.x2).abs() + (ey - lp.y2).abs() > lp2.len {
                            ex = (lp.x2 + ex) >> 1;
                            ey = (lp.y2 + ey) >> 1;
                        }
                    }
                    self.line2_no_clip(&lp2, ex, ey);
                } else {
                    self.line2_no_clip(lp, ex, ey);
                }
            }
        } else {
            self.line2_no_clip(lp, ex, ey);
        }
    }
    fn color<C: Color>(&mut self, color: C) {
        self.color = Rgba8::from_trait(color);
    }

    fn accurate_join_only(&self) -> bool {
        false
    }
}

#[derive(Debug, Default)]
/// Profile of a Line
struct LineProfileAA {
    min_width: f64,
    smoother_width: f64,
    subpixel_width: i64,
    gamma: Vec<u8>,
    profile: Vec<u8>,
}

impl LineProfileAA {
    /// Create new LineProfile
    ///
    /// Width is initialized to 0.0
    pub fn new() -> Self {
        let gamma: Vec<_> = (0..POLY_SUBPIXEL_SCALE).map(|x| x as u8).collect();
        let mut s =
            Self { min_width: 1.0, smoother_width: 1.0, subpixel_width: 0, profile: vec![], gamma };
        s.width(0.0);
        s
    }
    /// Set minimum width
    ///
    /// For this to take effect, the width needs to be set
    pub fn min_width(&mut self, width: f64) {
        self.min_width = width;
    }
    /// Set smoother width
    ///
    /// For this to take effect, the width needs to be set
    pub fn smoother_width(&mut self, width: f64) {
        self.smoother_width = width;
    }
    /// Set width
    ///
    /// Negative widths are set to 0.0
    ///
    /// Width less than smoother width are doubled, otherwise the smoother width is added
    ///  to the with
    /// Widths are then divied by 2 and the smoother width is removed.
    ///
    /// The line profile is then constructed and saved to `profile`
    pub fn width(&mut self, w: f64) {
        let mut w = w;
        if w < 0.0 {
            w = 0.0;
        }
        if w < self.smoother_width {
            w += w;
        } else {
            w += self.smoother_width;
        }
        w *= 0.5;
        w -= self.smoother_width;
        let mut s = self.smoother_width;
        if w < 0.0 {
            s += w;
            w = 0.0;
        }
        self.set(w, s);
    }
    fn profile(&mut self, w: f64) {
        let subpixel_shift = POLY_SUBPIXEL_SHIFT;
        let subpixel_scale = 1 << subpixel_shift;
        self.subpixel_width = (w * subpixel_scale as f64).round() as i64;
        let size = (self.subpixel_width + subpixel_scale * 6) as usize;
        if size > self.profile.capacity() {
            self.profile.resize(size, 0);
        }
    }
    /// Create the Line Profile
    ///
    ///
    fn set(&mut self, center_width: f64, smoother_width: f64) {
        let subpixel_shift = POLY_SUBPIXEL_SHIFT;
        let subpixel_scale = 1 << subpixel_shift;
        let aa_shift = POLY_SUBPIXEL_SHIFT;
        let aa_scale = 1 << aa_shift;
        let aa_mask = aa_scale - 1;

        let mut base_val = 1.0;
        let mut center_width = center_width;
        let mut smoother_width = smoother_width;

        // Set minimum values for the center and smoother widths
        if center_width == 0.0 {
            center_width = 1.0 / subpixel_scale as f64;
        }
        if smoother_width == 0.0 {
            smoother_width = 1.0 / subpixel_scale as f64;
        }
        // Full width
        let width = center_width + smoother_width;

        // Scale widths so they equal the minimum width
        if width < self.min_width {
            let k = width / self.min_width;
            base_val *= k;
            center_width /= k;
            smoother_width /= k;
        }

        // Allocate space for the line profile
        self.profile(center_width + smoother_width);

        // Width in Subpixel scales
        let subpixel_center_width: usize = (center_width * subpixel_scale as f64) as usize;
        let subpixel_smoother_width: usize = (smoother_width * subpixel_scale as f64) as usize;
        //
        let n_smoother = self.profile.len()
            - subpixel_smoother_width
            - subpixel_center_width
            - subpixel_scale * 2;

        // Center and Smoother Width Offsets
        let ch_center = subpixel_scale * 2;
        let ch_smoother = ch_center + subpixel_center_width;

        // Fill center portion of the profile (on one side) base_val
        let val = self.gamma[(base_val * f64::from(aa_mask)) as usize];
        for i in 0..subpixel_center_width {
            self.profile[ch_center + i] = val;
        }
        // Fill smoother portion of the profile with value decreasing linearly
        for i in 0..subpixel_smoother_width {
            let k = ((base_val - base_val * (i as f64 / subpixel_smoother_width as f64))
                * f64::from(aa_mask)) as usize;
            self.profile[ch_smoother + i] = self.gamma[k];
        }

        // Remainder is essentially 0.0
        let val = self.gamma[0];
        for i in 0..n_smoother {
            self.profile[ch_smoother + subpixel_smoother_width + i] = val;
        }
        // Copy to other side
        for i in 0..subpixel_scale * 2 {
            self.profile[ch_center - 1 - i] = self.profile[ch_center + i]
        }
    }
}

/// Ellipse Interpolator
#[derive(Debug)]
struct EllipseInterpolator {
    rx2: i64,
    ry2: i64,
    two_rx2: i64,
    two_ry2: i64,
    dx: i64,
    dy: i64,
    inc_x: i64,
    inc_y: i64,
    cur_f: i64,
}

impl EllipseInterpolator {
    /// Create new Ellipse Interpolator with axes lenghts `rx` and `ry`
    pub fn new(rx: i64, ry: i64) -> Self {
        let rx2 = rx * rx;
        let ry2 = ry * ry;
        let two_rx2 = rx2 * 2;
        let two_ry2 = ry2 * 2;
        let dx = 0;
        let dy = 0;
        let inc_x = 0;
        let inc_y = -ry * two_rx2;
        let cur_f = 0;

        Self { rx2, ry2, two_rx2, two_ry2, dx, dy, inc_x, inc_y, cur_f }
    }

    /// Increment the Interpolator
    fn inc(&mut self) {
        //
        let mut mx = self.cur_f + self.inc_x + self.ry2;
        let fx = mx;
        if mx < 0 {
            mx = -mx;
        }

        let mut my = self.cur_f + self.inc_y + self.rx2;
        let fy = my;
        if my < 0 {
            my = -my;
        }

        let mut mxy = self.cur_f + self.inc_x + self.ry2 + self.inc_y + self.rx2;
        let fxy = mxy;
        if mxy < 0 {
            mxy = -mxy;
        }

        let mut min_m = mx;

        let flag = if min_m > my {
            min_m = my;
            false
        } else {
            true
        };

        self.dx = 0;
        self.dy = 0;
        if min_m > mxy {
            self.inc_x += self.two_ry2;
            self.inc_y += self.two_rx2;
            self.cur_f = fxy;
            self.dx = 1;
            self.dy = 1;
            return;
        }

        if flag {
            self.inc_x += self.two_ry2;
            self.cur_f = fx;
            self.dx = 1;
            return;
        }

        self.inc_y += self.two_rx2;
        self.cur_f = fy;
        self.dy = 1;
    }
}
