#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
use crate::{LineParameters, RenderData, Rgba8, Vertex};
// #[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg(any(feature = "std", feature = "no_std"))]
use crate::Color;

/// A source of vertex points.
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
// MAYBE
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
#[cfg(any(feature = "std", feature = "no_std"))]
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
    /// # #[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
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

/// Functions for Drawing Outlines.
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "no_std", feature = "alloc"))))
)]
// pub trait DrawOutline: Lines + AccurateJoins + SetColor {} // MAYBE
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
