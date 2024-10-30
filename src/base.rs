// agrega::base
//
//! Rendering Base

use crate::{Color, Pixel, PixelSource};
use core::cmp::{max, min};
use devela::iif;

/// Rendering base that manages a pixel buffer.
#[must_use]
#[derive(Clone, Debug)]
pub struct RenderingBase<T> {
    /// Pixel format used by the rendering base.
    pub pixf: T,
}

impl<T: Pixel> RenderingBase<T> {
    /// Creates a new rendering base from the given pixel format.
    #[inline]
    pub const fn new(pixf: T) -> RenderingBase<T> {
        RenderingBase { pixf }
    }

    /// Returns the pixel buffer as a byte slice.
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.pixf.as_bytes()
    }

    /// Writes the pixel buffer to an image file.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub fn to_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), image::ImageError> {
        self.pixf.to_file(filename)
    }

    /// Fills the entire pixel buffer with the given color.
    #[inline]
    pub fn clear<C: Color>(&mut self, color: C) {
        self.pixf.fill(color);
    }

    /// Returns the image boundaries as `(xmin, xmax, ymin, ymax)`.
    // IMPROVE output type
    #[inline]
    #[must_use]
    pub fn limits(&self) -> (i64, i64, i64, i64) {
        let w = self.pixf.width() as i64;
        let h = self.pixf.height() as i64;
        (0, w - 1, 0, h - 1)
    }

    /// Blends a color horizontally from `x1` to `x2` at row `y` with the specified coverage.
    ///
    /// Ensures the blending stays within the image boundaries.
    pub fn blend_hline<C: Color>(&mut self, x1: i64, y: i64, x2: i64, c: C, cover: u64) {
        let (xmin, xmax, ymin, ymax) = self.limits();
        let (x1, x2) = if x2 > x1 { (x1, x2) } else { (x2, x1) };
        iif![y > ymax || y < ymin || x1 > xmax || x2 < xmin; return];
        let x1 = max(x1, xmin);
        let x2 = min(x2, xmax);
        self.pixf.blend_hline(x1, y, x2 - x1 + 1, c, cover);
    }

    /// Blends a horizontal span of pixels from `(x, y)` with individual coverage values.
    ///
    /// Coverage values in `covers` are applied to each pixel in the span.
    pub fn blend_solid_hspan<C: Color>(&mut self, x: i64, y: i64, len: i64, c: C, covers: &[u64]) {
        let (xmin, xmax, ymin, ymax) = self.limits();
        iif![y > ymax || y < ymin; return];
        let (mut x, mut len, mut off) = (x, len, 0);
        if x < xmin {
            len -= xmin - x;
            iif![len <= 0; return];
            off = off + xmin - x; // Adjust offset for out-of-bounds start
            x = xmin;
        }
        if x + len > xmax {
            len = xmax - x + 1;
            iif![len <= 0; return];
        }
        let covers_win = &covers[off as usize..(off + len) as usize];
        assert!(len as usize <= covers[off as usize..].len());
        self.pixf.blend_solid_hspan(x, y, len, c, covers_win);
    }

    /// Blends a vertical span of pixels from `(x, y)` with individual coverage values.
    ///
    /// Coverage values in `covers` are applied to each pixel in the span.
    pub fn blend_solid_vspan<C: Color>(&mut self, x: i64, y: i64, len: i64, c: C, covers: &[u64]) {
        let (xmin, xmax, ymin, ymax) = self.limits();
        if x > xmax || x < xmin {
            return;
        }
        let (mut y, mut len, mut off) = (y, len, 0);
        if y < ymin {
            len -= ymin - y;
            iif![len <= 0; return];
            off = off + ymin - y; // Adjust offset for out-of-bounds start
            y = ymin;
        }
        if y + len > ymax {
            len = ymax - y + 1;
            iif![len <= 0; return];
        }
        let covers_win = &covers[off as usize..(off + len) as usize];
        assert!(len as usize <= covers[off as usize..].len());
        self.pixf.blend_solid_vspan(x, y, len, c, covers_win);
    }

    /// Blends a vertical span of colors at `(x, y)` using optional per-pixel coverage values.
    ///
    /// - `colors` specifies the color for each pixel.
    /// - `covers` provides optional coverage values,
    ///   with a single `cover` applied if `covers` is empty.
    pub fn blend_color_vspan<C: Color>(
        &mut self,
        x: i64,
        y: i64,
        len: i64,
        colors: &[C],
        covers: &[u64],
        cover: u64,
    ) {
        let (xmin, xmax, ymin, ymax) = self.limits();
        iif![x > xmax || x < xmin; return];
        let (mut y, mut len, mut off) = (y, len, 0);
        if y < ymin {
            len -= ymin - y;
            iif![len <= 0; return];
            off = off + ymin - y; // Adjust offset for out-of-bounds start
            y = ymin;
        }
        if y + len > ymax {
            len = ymax - y + 1;
            iif![len <= 0; return];
        }
        let covers_win = if covers.is_empty() {
            &[]
        } else {
            &covers[off as usize..(off + len) as usize]
        };
        let colors_win = &colors[off as usize..(off + len) as usize];
        self.pixf.blend_color_vspan(x, y, len, colors_win, covers_win, cover);
    }

    /// Blends a horizontal span of colors at `(x, y)` using optional per-pixel coverage values.
    ///
    /// - `colors` specifies the color for each pixel.
    /// - `covers` provides optional coverage values,
    ///   with a single `cover` applied if `covers` is empty.
    pub fn blend_color_hspan<C: Color>(
        &mut self,
        x: i64,
        y: i64,
        len: i64,
        colors: &[C],
        covers: &[u64],
        cover: u64,
    ) {
        let (xmin, xmax, ymin, ymax) = self.limits();
        iif![y > ymax || y < ymin; return];
        let (mut x, mut len, mut off) = (x, len, 0);
        if x < xmin {
            len -= xmin - x;
            iif![len <= 0; return];
            off = off + xmin - x; // Adjust offset for out-of-bounds start
            x = xmin;
        }
        if x + len > xmax {
            len = xmax - x + 1;
            iif![len <= 0; return];
        }
        let covers_win = if covers.is_empty() {
            &[]
        } else {
            &covers[off as usize..(off + len) as usize]
        };
        let colors_win = &colors[off as usize..(off + len) as usize];
        self.pixf.blend_color_hspan(x, y, len, colors_win, covers_win, cover);
    }

    /// Blends from another pixel buffer, with the specified opacity.
    ///
    /// Ensures the buffers have matching dimensions.
    /// The opacity is applied as a multiplier for blending.
    pub fn blend_from<S: Pixel + PixelSource>(&mut self, other: &S, opacity: f64) {
        if self.pixf.width() != other.width() || self.pixf.height() != other.height() {
            panic!("wrong size");
        }
        for x in 0..self.pixf.width() {
            for y in 0..self.pixf.height() {
                let c = other.get((x, y));
                self.pixf.blend_pix((x, y), c, (opacity * 255.0) as u64);
            }
        }
    }
}
