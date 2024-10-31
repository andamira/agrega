// agrega::scanline::raster
//
//! Rasterizer

use crate::{
    Clip, PathCommand, RasterizerCell, ScanlineU8, Vertex, VertexSource, POLY_SUBPIXEL_SCALE,
    POLY_SUBPIXEL_SHIFT,
};
use alloc::vec::Vec;
use core::cmp::{max, min};
#[allow(unused_imports)]
use devela::ExtFloat;

struct RasConvInt;
impl RasConvInt {
    pub fn upscale(v: f64) -> i64 {
        (v * POLY_SUBPIXEL_SCALE as f64).round() as i64
    }
}

/// Winding / Filling Rule.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum FillingRule {
    /// [Non-Zero Filling Rule](https://en.wikipedia.org/wiki/Nonzero-rule).
    #[default]
    NonZero,
    /// [Even-Odd Filling Rule](https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule).
    EvenOdd,
}

/// Path Status.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum PathStatus {
    #[default]
    Initial,
    Closed,
    MoveTo,
    LineTo,
}

/// Rasterizer Anti-Alias using Scanline.
#[derive(Debug)]
pub struct RasterizerScanline {
    /// Clipping Region
    pub(crate) clipper: Clip,
    /// Status of Path
    pub(crate) status: PathStatus,
    /// Current x position
    pub(crate) x0: i64,
    /// Current y position
    pub(crate) y0: i64,

    /// Collection of Rasterizing Cells
    outline: RasterizerCell,
    /// Current y row being worked on, for output
    scan_y: i64,
    /// Filling Rule for Polygons
    filling_rule: FillingRule,
    /// Gamma Corection Values
    gamma: Vec<u64>,
}

impl Default for RasterizerScanline {
    fn default() -> Self {
        Self::new()
    }
}

impl RasterizerScanline {
    /// Creates a new `RasterizerScanline`.
    pub fn new() -> Self {
        Self {
            clipper: Clip::new(),
            status: PathStatus::Initial,
            outline: RasterizerCell::new(),
            x0: 0,
            y0: 0,
            scan_y: 0,
            filling_rule: FillingRule::NonZero,
            gamma: (0..256).collect(),
        }
    }

    /// Create a new `RasterizerScanline` with a gamma function
    ///
    /// See [`gamma`][Self::gamma] for description.
    #[inline]
    #[must_use]
    pub fn with_gamma<F: Fn(f64) -> f64>(gfunc: F) -> Self {
        let mut new = Self::new();
        new.gamma(gfunc);
        new
    }

    /* */

    /// Resets the rasterizer.
    ///
    /// Reset the RasterizerCell and set PathStatus to Initial
    #[inline]
    pub fn reset(&mut self) {
        self.outline.reset();
        self.status = PathStatus::Initial;
    }

    /// Add a Path
    ///
    /// Walks the path from the VertexSource and rasterizes it
    pub fn add_path<VS: VertexSource>(&mut self, path: &VS) {
        //path.rewind();
        if !self.outline.sorted_y.is_empty() {
            self.reset();
        }
        for seg in path.xconvert() {
            match seg.cmd {
                PathCommand::LineTo => self.line_to(seg.x, seg.y),
                PathCommand::MoveTo => self.move_to(seg.x, seg.y),
                PathCommand::Close => self.close_polygon(),
                PathCommand::Stop => unimplemented!("stop encountered"),
            }
        }
    }

    /// Sets the filling rule.
    #[inline]
    pub fn set_filling_rule(&mut self, filling_rule: FillingRule) {
        self.filling_rule = filling_rule;
    }

    /// Rewind the Scanline
    ///
    /// Close active polygon, sort the Rasterizer Cells, set the
    /// scan_y value to the minimum y value and return if any cells
    /// are present
    pub(crate) fn rewind_scanlines(&mut self) -> bool {
        self.close_polygon();
        self.outline.sort_cells();
        if self.outline.total_cells() == 0 {
            false
        } else {
            self.scan_y = self.outline.min_y;
            true
        }
    }

    /// Sweep the Scanline
    ///
    /// For individual y rows adding any to the input Scanline
    ///
    /// Returns true if data exists in the input Scanline
    pub(crate) fn sweep_scanline(&mut self, sl: &mut ScanlineU8) -> bool {
        loop {
            if self.scan_y < 0 {
                self.scan_y += 1;
                continue;
            }
            if self.scan_y > self.outline.max_y {
                return false;
            }
            sl.reset_spans();
            let mut num_cells = self.outline.scanline_num_cells(self.scan_y);
            let cells = self.outline.scanline_cells(self.scan_y);

            let mut cover = 0;

            let mut iter = cells.iter();

            if let Some(mut cur_cell) = iter.next() {
                while num_cells > 0 {
                    let mut x = cur_cell.x;
                    let mut area = cur_cell.area;

                    cover += cur_cell.cover;
                    num_cells -= 1;
                    //accumulate all cells with the same X
                    while num_cells > 0 {
                        cur_cell = iter.next().unwrap();
                        if cur_cell.x != x {
                            break;
                        }
                        area += cur_cell.area;
                        cover += cur_cell.cover;
                        num_cells -= 1;
                    }
                    if area != 0 {
                        let alpha =
                            self.calculate_alpha((cover << (POLY_SUBPIXEL_SHIFT + 1)) - area);
                        if alpha > 0 {
                            sl.add_cell(x, alpha);
                        }
                        x += 1;
                    }
                    if num_cells > 0 && cur_cell.x > x {
                        let alpha = self.calculate_alpha(cover << (POLY_SUBPIXEL_SHIFT + 1));
                        if alpha > 0 {
                            sl.add_span(x, cur_cell.x - x, alpha);
                        }
                    }
                }
            }
            if sl.num_spans() != 0 {
                break;
            }
            self.scan_y += 1;
        }
        sl.finalize(self.scan_y);
        self.scan_y += 1;
        true
    }

    /// Return minimum x value from the `RasterizerCell`.
    #[inline]
    #[must_use]
    pub fn min_x(&self) -> i64 {
        self.outline.min_x
    }

    /// Return maximum x value from the `RasterizerCell`.
    #[inline]
    #[must_use]
    pub fn max_x(&self) -> i64 {
        self.outline.max_x
    }

    /// Sets the gamma function
    ///
    /// Values are set as:
    /// ```txt
    /// gamma = gfunc( v / mask ) * mask
    /// where v = 0 to 255
    /// ```
    pub fn gamma<F: Fn(f64) -> f64>(&mut self, gfunc: F) {
        let aa_shift = 8;
        let aa_scale = 1 << aa_shift;
        let aa_mask = f64::from(aa_scale - 1);

        self.gamma = (0..256)
            .map(|i| gfunc(f64::from(i) / aa_mask))
            .map(|v| (v * aa_mask).round() as u64)
            .collect();
    }

    /// Set Clip Box
    pub fn clip_box(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.clipper.clip_box(
            RasConvInt::upscale(x1),
            RasConvInt::upscale(y1),
            RasConvInt::upscale(x2),
            RasConvInt::upscale(y2),
        );
    }

    /// Moves to point (x,y).
    ///
    /// Sets point as the initial point.
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.x0 = RasConvInt::upscale(x);
        self.y0 = RasConvInt::upscale(y);
        self.clipper.move_to(self.x0, self.y0);
        self.status = PathStatus::MoveTo;
    }

    /// Draws a line from previous point to new point (x,y).
    pub fn line_to(&mut self, x: f64, y: f64) {
        let x = RasConvInt::upscale(x);
        let y = RasConvInt::upscale(y);
        self.clipper.line_to(&mut self.outline, x, y);
        self.status = PathStatus::LineTo;
    }

    /// Closes the current polygon.
    ///
    /// Draw a line from current point to initial "move to" point
    pub fn close_polygon(&mut self) {
        if self.status == PathStatus::LineTo {
            self.clipper.line_to(&mut self.outline, self.x0, self.y0);
            self.status = PathStatus::Closed;
        }
    }

    /// Calculates alpha term based on area.
    #[must_use]
    fn calculate_alpha(&self, area: i64) -> u64 {
        let aa_shift = 8;
        let aa_scale = 1 << aa_shift;
        let aa_scale2 = aa_scale * 2;
        let aa_mask = aa_scale - 1;
        let aa_mask2 = aa_scale2 - 1;

        let mut cover = area >> (POLY_SUBPIXEL_SHIFT * 2 + 1 - aa_shift);
        cover = cover.abs();
        if self.filling_rule == FillingRule::EvenOdd {
            cover &= aa_mask2;
            if cover > aa_scale {
                cover = aa_scale2 - cover;
            }
        }
        cover = max(0, min(cover, aa_mask));
        self.gamma[cover as usize]
    }
}

/// TODO
pub(crate) fn len_i64(a: &Vertex<i64>, b: &Vertex<i64>) -> i64 {
    len_i64_xy(a.x, a.y, b.x, b.y)
}

/// TODO
pub(crate) fn len_i64_xy(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    let dx = x1 as f64 - x2 as f64;
    let dy = y1 as f64 - y2 as f64;
    (dx * dx + dy * dy).sqrt().round() as i64
}

// /// MAYBE
// #[derive(Debug,PartialEq,Copy,Clone)]
// pub enum LineJoin {
//     Round,
//     None,
//     Miter,
//     MiterAccurate,
// }
