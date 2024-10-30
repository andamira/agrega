// agrega::base
//
//! Rendering Base

use crate::{Color, Pixel, PixelSource};
use core::cmp::{max, min};

/// Rendering Base
#[must_use]
#[derive(Clone, Debug)]
pub struct RenderingBase<T> {
    /// Pixel Format
    pub pixf: T,
}

impl<T: Pixel> RenderingBase<T> {
    /// Creates new a new rendering base from the given pixel format.
    #[inline]
    pub const fn new(pixf: T) -> RenderingBase<T> {
        RenderingBase { pixf }
    }

    /// Returns the rendering base as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.pixf.as_bytes()
    }

    /// Writes the image to a file.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub fn to_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), image::ImageError> {
        self.pixf.to_file(filename)
    }

    /// Sets the image to a single color.
    //
    // MAYBE: IMPROVE:
    #[inline]
    pub fn clear<C: Color>(&mut self, color: C) {
        self.pixf.fill(color);
    }

    /// Returns the image size limits.
    pub fn limits(&self) -> (i64, i64, i64, i64) {
        let w = self.pixf.width() as i64;
        let h = self.pixf.height() as i64;
        (0, w - 1, 0, h - 1)
    }

    /// Blends a color along y-row from x1 to x2.
    pub fn blend_hline<C: Color>(&mut self, x1: i64, y: i64, x2: i64, c: C, cover: u64) {
        let (xmin, xmax, ymin, ymax) = self.limits();
        let (x1, x2) = if x2 > x1 { (x1, x2) } else { (x2, x1) };
        if y > ymax || y < ymin || x1 > xmax || x2 < xmin {
            return;
        }
        let x1 = max(x1, xmin);
        let x2 = min(x2, xmax);
        self.pixf.blend_hline(x1, y, x2 - x1 + 1, c, cover);
    }

    /// Blends a color from (x,y) with variable covers.
    pub fn blend_solid_hspan<C: Color>(&mut self, x: i64, y: i64, len: i64, c: C, covers: &[u64]) {
        let (xmin, xmax, ymin, ymax) = self.limits();
        if y > ymax || y < ymin {
            return;
        }
        let (mut x, mut len, mut off) = (x, len, 0);
        if x < xmin {
            len -= xmin - x;
            if len <= 0 {
                return;
            }
            off = off + xmin - x; // Woah!!!!
            x = xmin;
        }
        if x + len > xmax {
            len = xmax - x + 1;
            if len <= 0 {
                return;
            }
        }
        let covers_win = &covers[off as usize..(off + len) as usize];
        assert!(len as usize <= covers[off as usize..].len());
        self.pixf.blend_solid_hspan(x, y, len, c, covers_win);
    }

    /// Blends a color from (x,y) with variable covers.
    pub fn blend_solid_vspan<C: Color>(&mut self, x: i64, y: i64, len: i64, c: C, covers: &[u64]) {
        let (xmin, xmax, ymin, ymax) = self.limits();
        if x > xmax || x < xmin {
            return;
        }
        let (mut y, mut len, mut off) = (y, len, 0);
        if y < ymin {
            len -= ymin - y;
            if len <= 0 {
                return;
            }
            off = off + ymin - y; // Woah!!!!
            y = ymin;
        }
        if y + len > ymax {
            len = ymax - y + 1;
            if len <= 0 {
                return;
            }
        }
        let covers_win = &covers[off as usize..(off + len) as usize];
        assert!(len as usize <= covers[off as usize..].len());
        self.pixf.blend_solid_vspan(x, y, len, c, covers_win);
    }

    /// Blends colors (TODO):
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
        if x > xmax || x < xmin {
            return;
        }
        let (mut y, mut len, mut off) = (y, len, 0);
        if y < ymin {
            len -= ymin - y;
            if len <= 0 {
                return;
            }
            off = off + ymin - y; // Woah!!!!
            y = ymin;
        }
        if y + len > ymax {
            len = ymax - y + 1;
            if len <= 0 {
                return;
            }
        }
        let covers_win = if covers.is_empty() {
            &[]
        } else {
            &covers[off as usize..(off + len) as usize]
        };
        let colors_win = &colors[off as usize..(off + len) as usize];
        self.pixf.blend_color_vspan(x, y, len, colors_win, covers_win, cover);
    }

    /// Blends colors (TODO):
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
        if y > ymax || y < ymin {
            return;
        }
        let (mut x, mut len, mut off) = (x, len, 0);
        if x < xmin {
            len -= xmin - x;
            if len <= 0 {
                return;
            }
            off = off + xmin - x; // Woah!!!!
            x = xmin;
        }
        if x + len > xmax {
            len = xmax - x + 1;
            if len <= 0 {
                return;
            }
        }
        let covers_win = if covers.is_empty() {
            &[]
        } else {
            &covers[off as usize..(off + len) as usize]
        };
        let colors_win = &colors[off as usize..(off + len) as usize];
        self.pixf.blend_color_hspan(x, y, len, colors_win, covers_win, cover);
    }

    /// Blends from (TODO):
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
