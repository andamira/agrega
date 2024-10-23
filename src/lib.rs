//
#![doc = include_str!("./Lib.md")]
#![warn(clippy::all)]
#![allow(clippy::doc_lazy_continuation, clippy::module_inception)]
#![cfg_attr(not(any(feature = "std", feature = "no_std")), allow(unused))]
// nightly, safety, environment
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguarding: environment, safety
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("You can't enable `safe` and `unsafe*` features at the same time.");

#[doc(hidden)]
#[cfg(all(feature = "std", feature = "freetype-rs"))]
pub use freetype as ft;

pub mod _gallery {
    #![doc = include_str!("./Gallery.md")]
}

pub mod math;

/* alloc */

// private
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub(crate) mod cell;
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub(crate) mod scan;

/* std || no_std + alloc */

// private, few items
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
mod alphamask;
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
mod base; // uses ::color
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
mod pixfmt; // uses color

// public
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod clip;
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod color;
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod interp;
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod outline;
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod outline_aa;
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod paths;
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod raster;
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod render;
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod stroke;
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub mod transform;

#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
pub use {alphamask::*, pixfmt::*};
#[doc(hidden)]
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
pub use {
    base::*, clip::*, color::*, interp::*, outline::*, outline_aa::*, paths::*, raster::*,
    render::*, stroke::*, transform::*,
};

/* std */

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub mod ppm;

/* std & freetype-rs */

#[cfg(all(feature = "std", feature = "freetype-rs"))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "std", feature = "freetype-rs")))
)]
pub mod text;
#[doc(hidden)]
#[cfg(all(feature = "std", feature = "freetype-rs"))]
pub use text::*;

#[cfg(feature = "alloc")]
const POLY_SUBPIXEL_SHIFT: i64 = 8;
#[cfg(feature = "alloc")]
const POLY_SUBPIXEL_SCALE: i64 = 1 << POLY_SUBPIXEL_SHIFT;
#[cfg(feature = "alloc")]
const POLY_SUBPIXEL_MASK: i64 = POLY_SUBPIXEL_SCALE - 1;
#[cfg(feature = "alloc")]
const POLY_MR_SUBPIXEL_SHIFT: i64 = 4;
#[cfg(feature = "alloc")]
const MAX_HALF_WIDTH: usize = 64;

/// Source of vertex points
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "no_std", feature = "alloc"))))
)]
pub trait VertexSource {
    /// Rewind the vertex source (unused)
    fn rewind(&self) {}
    /// Get values from the source
    ///
    /// This could be turned into an iterator
    fn xconvert(&self) -> alloc::vec::Vec<Vertex<f64>>;
}

/// Access Color properties and compoents
pub trait Color: core::fmt::Debug + Copy {
    /// Get red value [0..=1] as f64
    fn red(&self) -> f64;
    /// Get green value [0..=1] as f64
    fn green(&self) -> f64;
    /// Get blue value [0..=1] as f64
    fn blue(&self) -> f64;
    /// Get alpha value [0..=1] as f64
    fn alpha(&self) -> f64;
    /// Get red value [0..=255] as u8
    fn red8(&self) -> u8;
    /// Get green value [0..=255] as u8
    fn green8(&self) -> u8;
    /// Get blue value [0..=255] as u8
    fn blue8(&self) -> u8;
    /// Get alpha value [0..=255] as u8
    fn alpha8(&self) -> u8;
    /// Return if the color is completely transparent, alpha = 0.0
    fn is_transparent(&self) -> bool {
        self.alpha() == 0.0
    }
    /// Return if the color is completely opaque, alpha = 1.0
    fn is_opaque(&self) -> bool {
        self.alpha() >= 1.0
    }
    /// Return if the color has been premultiplied
    fn is_premultiplied(&self) -> bool;
}
/// Render scanlines to Image
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "no_std", feature = "alloc"))))
)]
pub trait Render {
    /// Render a single scanlines to the image
    fn render(&mut self, data: &RenderData);
    /// Set the Color of the Renderer
    fn color<C: Color>(&mut self, color: C);
    /// Prepare the Renderer
    fn prepare(&self) {}
}
/*
/// Rasterize lines, path, and other things to scanlines
pub trait Rasterize {
    /// Setup Rasterizer, returns if data is available
    fn rewind_scanlines(&mut self) -> bool;
    /// Sweeps cells in a scanline for data, returns if data is available
    fn sweep_scanline(&mut self, sl: &mut ScanlineU8) -> bool;
    /// Return maximum x value of rasterizer
    fn min_x(&self) -> i64;
    /// Return maximum x value of rasterizer
    fn max_x(&self) -> i64;
    /// Resets the rasterizer, clearing content
    fn reset(&mut self);
    /// Rasterize a path
    fn add_path<VS: VertexSource>(&mut self, path: &VS);
}
*/

/// Access Pixel source color
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "no_std", feature = "alloc"))))
)]
pub trait Source {
    fn get(&self, id: (usize, usize)) -> Rgba8;
}

/// Drawing and pixel related routines
pub trait Pixel {
    fn cover_mask() -> u64;
    fn bpp() -> usize;
    fn as_bytes(&self) -> &[u8];
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    fn to_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), image::ImageError>;
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn set<C: Color>(&mut self, id: (usize, usize), c: C);
    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, c: C);
    fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64);
    /// Fill the data with the specified `color`
    fn fill<C: Color>(&mut self, color: C);
    /// Copy or blend a pixel at `id` with `color`
    ///
    /// If `color` [`is_opaque`], the color is copied directly to the pixel,
    ///   otherwise the color is blended with the pixel at `id`
    ///
    /// If `color` [`is_transparent`] nothing is done
    ///
    /// [`is_opaque`]: ../trait.Color.html#method.is_opaque
    /// [`is_transparent`]: ../trait.Color.html#method.is_transparent
    fn copy_or_blend_pix<C: Color>(&mut self, id: (usize, usize), color: C) {
        if !color.is_transparent() {
            if color.is_opaque() {
                self.set(id, color);
            } else {
                self.blend_pix(id, color, 255);
            }
        }
    }
    /// Copy or blend a pixel at `id` with `color` and a `cover`
    ///
    /// If `color` [`is_opaque`] *and* `cover` equals [`cover_mask`] then
    ///   the color is copied to the pixel at `id', otherwise the `color`
    ///   is blended with the pixel at `id' considering the amount of `cover`
    ///
    /// If `color` [`is_transparent`] nothing is done
    ///
    /// ```
    /// # #[cfg(feature = "std")]
    /// # {
    /// use agrega::{Source, Pixfmt, Rgb8, Rgba8, Pixel};
    ///
    /// let mut pix = Pixfmt::<Rgb8>::new(1,1);
    /// let black  = Rgba8::black();
    /// let white  = Rgba8::white();
    /// pix.copy_pixel(0,0,black);
    /// assert_eq!(pix.get((0,0)), black);
    ///
    /// let (alpha, cover) = (255, 255); // Copy Pixel
    /// let color = Rgba8::new(255,255,255,alpha);
    /// pix.copy_or_blend_pix_with_cover((0,0), color, cover);
    /// assert_eq!(pix.get((0,0)), white);
    ///
    /// let (alpha, cover) = (255, 128); // Partial Coverage, Blend
    /// let color = Rgba8::new(255,255,255,alpha);
    /// pix.copy_pixel(0,0,black);
    /// pix.copy_or_blend_pix_with_cover((0,0), color, cover);
    /// assert_eq!(pix.get((0,0)), Rgba8::new(128,128,128,255));
    ///
    /// let (alpha, cover) = (128, 255); // Partial Coverage, Blend
    /// let color = Rgba8::new(255,255,255,alpha);
    /// pix.copy_pixel(0,0,black);
    /// pix.copy_or_blend_pix_with_cover((0,0), color, cover);
    /// assert_eq!(pix.get((0,0)), Rgba8::new(128,128,128,255));
    /// # }
    /// ```
    ///
    /// [`is_opaque`]: ../trait.Color.html#method.is_opaque
    /// [`is_transparent`]: ../trait.Color.html#method.is_transparent
    /// [`cover_mask`]: ../trait.Pixel.html#method.cover_mask
    fn copy_or_blend_pix_with_cover<C: Color>(&mut self, id: (usize, usize), color: C, cover: u64) {
        if !color.is_transparent() {
            if color.is_opaque() && cover == Self::cover_mask() {
                self.set(id, color);
            } else {
                self.blend_pix(id, color, cover);
            }
        }
    }
    /// Copy or Blend a single `color` from (`x`,`y`) to (`x+len-1`,`y`) with `cover`.
    fn blend_hline<C: Color>(&mut self, x: i64, y: i64, len: i64, color: C, cover: u64) {
        if color.is_transparent() {
            return;
        }
        let (x, y, len) = (x as usize, y as usize, len as usize);
        if color.is_opaque() && cover == Self::cover_mask() {
            self.setn((x, y), len, color);
        } else {
            for i in 0..len {
                self.blend_pix((x + i, y), color, cover);
            }
        }
    }
    /// Blend a single `color` from (`x`,`y`) to (`x+len-1`,`y`) with collection of `covers`.
    fn blend_solid_hspan<C: Color>(&mut self, x: i64, y: i64, len: i64, color: C, covers: &[u64]) {
        assert_eq!(len as usize, covers.len());
        for (i, &cover) in covers.iter().enumerate() {
            self.blend_hline(x + i as i64, y, 1, color, cover);
        }
    }
    /// Copy or Blend a single `color` from (`x`,`y`) to (`x`,`y+len-1`) with `cover`.
    fn blend_vline<C: Color>(&mut self, x: i64, y: i64, len: i64, c: C, cover: u64) {
        if c.is_transparent() {
            return;
        }
        let (x, y, len) = (x as usize, y as usize, len as usize);
        if c.is_opaque() && cover == Self::cover_mask() {
            for i in 0..len {
                self.set((x, y + i), c);
            }
        } else {
            for i in 0..len {
                self.blend_pix((x, y + i), c, cover);
            }
        }
    }
    /// Blend a single `color` from (`x`,`y`) to (`x`,`y+len-1`) with collection of `covers`.
    fn blend_solid_vspan<C: Color>(&mut self, x: i64, y: i64, len: i64, c: C, covers: &[u64]) {
        assert_eq!(len as usize, covers.len());
        for (i, &cover) in covers.iter().enumerate() {
            self.blend_vline(x, y + i as i64, 1, c, cover);
        }
    }
    /// Blend a collection of `colors` from (`x`,`y`) to (`x+len-1`,`y`) with
    /// either a collection of `covers` or a single `cover`.
    ///
    /// A collection of `covers` takes precedence over a single `cover`.
    fn blend_color_hspan<C: Color>(
        &mut self,
        x: i64,
        y: i64,
        len: i64,
        colors: &[C],
        covers: &[u64],
        cover: u64,
    ) {
        assert_eq!(len as usize, colors.len());
        let (x, y) = (x as usize, y as usize);
        if !covers.is_empty() {
            assert_eq!(colors.len(), covers.len());
            for (i, (&color, &cover)) in colors.iter().zip(covers.iter()).enumerate() {
                self.copy_or_blend_pix_with_cover((x + i, y), color, cover);
            }
        } else if cover == 255 {
            for (i, &color) in colors.iter().enumerate() {
                self.copy_or_blend_pix((x + i, y), color);
            }
        } else {
            for (i, &color) in colors.iter().enumerate() {
                self.copy_or_blend_pix_with_cover((x + i, y), color, cover);
            }
        }
    }
    /// Blend a collection of `colors` from (`x`,`y`) to (`x`,`y+len-1`) with
    /// either a collection of `covers` or a single `cover`.
    ///
    /// A collection of `covers` takes precedence over a single `cover`.
    fn blend_color_vspan<C: Color>(
        &mut self,
        x: i64,
        y: i64,
        len: i64,
        colors: &[C],
        covers: &[u64],
        cover: u64,
    ) {
        assert_eq!(len as usize, colors.len());
        let (x, y) = (x as usize, y as usize);
        if !covers.is_empty() {
            assert_eq!(colors.len(), covers.len());
            for (i, (&color, &cover)) in colors.iter().zip(covers.iter()).enumerate() {
                self.copy_or_blend_pix_with_cover((x, y + i), color, cover);
            }
        } else if cover == 255 {
            for (i, &color) in colors.iter().enumerate() {
                self.copy_or_blend_pix((x, y + i), color);
            }
        } else {
            for (i, &color) in colors.iter().enumerate() {
                self.copy_or_blend_pix_with_cover((x, y + i), color, cover);
            }
        }
    }
}

// TODO
// pub(crate) trait LineInterp {
//     fn init(&mut self);
//     fn step_hor(&mut self);
//     fn step_ver(&mut self);
// }

pub(crate) trait RenderOutline {
    fn cover(&self, d: i64) -> u64;
    fn blend_solid_hspan(&mut self, x: i64, y: i64, len: i64, covers: &[u64]);
    fn blend_solid_vspan(&mut self, x: i64, y: i64, len: i64, covers: &[u64]);
}

/// Functions for Drawing Outlines.
//pub trait DrawOutline: Lines + AccurateJoins + SetColor {}
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "no_std", feature = "alloc"))))
)]
pub trait DrawOutline {
    /// Set the current Color
    fn color<C: Color>(&mut self, color: C);
    /// If Line Joins are Accurate
    fn accurate_join_only(&self) -> bool;
    fn line0(&mut self, lp: &LineParameters);
    fn line1(&mut self, lp: &LineParameters, sx: i64, sy: i64);
    fn line2(&mut self, lp: &LineParameters, ex: i64, ey: i64);
    fn line3(&mut self, lp: &LineParameters, sx: i64, sy: i64, ex: i64, ey: i64);
    fn semidot<F>(&mut self, cmp: F, xc1: i64, yc1: i64, xc2: i64, yc2: i64)
    where
        F: Fn(i64) -> bool;
    fn pie(&mut self, xc: i64, y: i64, x1: i64, y1: i64, x2: i64, y2: i64);
}

pub(crate) trait DistanceInterpolator {
    fn dist(&self) -> i64;
    fn inc_x(&mut self, dy: i64);
    fn inc_y(&mut self, dx: i64);
    fn dec_x(&mut self, dy: i64);
    fn dec_y(&mut self, dx: i64);
}

/// All items are flat re-exported here.
pub mod all {
    #[doc(inline)]
    pub use super::{math::*, Color, Pixel};

    #[doc(inline)]
    #[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
    pub use super::{
        alphamask::*, base::*, clip::*, color::*, interp::*, outline::*, outline_aa::*, paths::*,
        pixfmt::*, raster::*, render::*, stroke::*, transform::*, DrawOutline, Render, Source,
        VertexSource,
    };

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::ppm::*;

    #[doc(inline)]
    #[cfg(all(feature = "std", feature = "freetype-rs"))]
    pub use super::text::*;
}
