use super::{PixelSource, Pixfmt};
use crate::{color::*, util::*};
#[cfg(feature = "std")]
use {std::path::Path, crate::file::write_file};

macro_rules! impl_pixel_common {
    () => {
        /// Height of rendering buffer in pixels
        #[inline]
        fn height(&self) -> usize {
            self.rbuf.height
        }

        /// Width of rendering buffer in pixels
        #[inline]
        fn width(&self) -> usize {
            self.rbuf.width
        }

        /// Return a underlying raw pixel/component data
        #[inline]
        fn as_bytes(&self) -> &[u8] {
            &self.rbuf.data
        }
    };
}

/// Drawing and pixel-related routines.
#[cfg(any(feature = "std", feature = "no_std"))]
pub trait Pixel {
    /// Returns the mask used to cover a pixel, representing full coverage.
    #[must_use]
    fn cover_mask() -> u64;

    /// Returns the bytes per pixel (bpp) for the current format.
    #[must_use]
    fn bpp() -> usize;

    /// Returns the pixel data as a byte slice.
    #[must_use]
    fn as_bytes(&self) -> &[u8];

    /// Saves the pixel data to the given file path.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    fn to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), image::ImageError>;

    /// Returns the width of the pixel buffer in pixels.
    #[must_use]
    fn width(&self) -> usize;
    /// Returns the height of the pixel buffer in pixels.
    #[must_use]
    fn height(&self) -> usize;

    /// Sets the pixel at the given `id` to the given `color`.
    fn set<C: Color>(&mut self, id: (usize, usize), color: C);

    /// Sets `n` pixels starting at the given `id` to the given `color`.
    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, color: C);

    /// Blends the given `color` into the pixel at `id` with the given `cover` value.
    fn blend_pix<C: Color>(&mut self, id: (usize, usize), color: C, cover: u64);

    /// Fills the entire buffer with the given `color`.
    fn fill<C: Color>(&mut self, color: C);

    /// Copies or blends the pixel at `id` with the given color `color`.
    ///
    /// - If `color` is opaque, it directly sets the pixel.
    /// - If `color` is transparent, no action is taken.
    /// - Otherwise, `color` is blended with the pixel at `id`.
    fn copy_or_blend_pix<C: Color>(&mut self, id: (usize, usize), color: C) {
        if !color.is_transparent() {
            if color.is_opaque() {
                self.set(id, color);
            } else {
                self.blend_pix(id, color, 255);
            }
        }
    }

    /// Copies or blends the pixel at `id` with the given color `color` and `cover` value.
    ///
    /// - If `color` is opaque and `cover` equals the cover mask, it directly sets the pixel.
    /// - If `color` is transparent, no action is taken.
    /// - Otherwise, `color` is blended with the pixel at `id`, adjusted by `cover`.
    ///
    /// ```
    /// # #[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
    /// # {
    /// use agrega::{PixelSource, Pixfmt, Rgb8, Rgba8, Pixel};
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
    fn copy_or_blend_pix_with_cover<C: Color>(&mut self, id: (usize, usize), color: C, cover: u64) {
        if !color.is_transparent() {
            if color.is_opaque() && cover == Self::cover_mask() {
                self.set(id, color);
            } else {
                self.blend_pix(id, color, cover);
            }
        }
    }

    /// Blends a horizontal line of pixels from `(x, y)` for `len` pixels with the given `color` and `cover`.
    ///
    /// - If `color` is opaque and `cover` equals the cover mask, it sets all pixels directly.
    /// - Otherwise, `color` is blended with each pixel in the line.
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

    /// Blends a horizontal span of pixels from `(x, y)` for `len` pixels using `color` and `covers`.
    ///
    /// The `covers` array specifies individual coverage for each pixel in the span.
    fn blend_solid_hspan<C: Color>(&mut self, x: i64, y: i64, len: i64, color: C, covers: &[u64]) {
        assert_eq!(len as usize, covers.len());
        for (i, &cover) in covers.iter().enumerate() {
            self.blend_hline(x + i as i64, y, 1, color, cover);
        }
    }

    /// Blends a vertical line of pixels from `(x, y)` for `len` pixels with the given color `c` and `cover`.
    ///
    /// - If `c` is opaque and `cover` equals the cover mask, it sets all pixels directly.
    /// - Otherwise, `c` is blended with each pixel in the line.
    fn blend_vline<C: Color>(&mut self, x: i64, y: i64, len: i64, color: C, cover: u64) {
        if color.is_transparent() {
            return;
        }
        let (x, y, len) = (x as usize, y as usize, len as usize);
        if color.is_opaque() && cover == Self::cover_mask() {
            for i in 0..len {
                self.set((x, y + i), color);
            }
        } else {
            for i in 0..len {
                self.blend_pix((x, y + i), color, cover);
            }
        }
    }

    /// Blends a vertical span of pixels from `(x, y)` for `len` pixels using `color` and `covers`.
    ///
    /// The `covers` array specifies individual coverage for each pixel in the span.
    fn blend_solid_vspan<C: Color>(&mut self, x: i64, y: i64, len: i64, color: C, covers: &[u64]) {
        assert_eq!(len as usize, covers.len());
        for (i, &cover) in covers.iter().enumerate() {
            self.blend_vline(x, y + i as i64, 1, color, cover);
        }
    }

    
    /// Blends a horizontal span of pixels from `(x, y)`
    /// for `len` pixels with `colors` and either `covers` or `cover`.
    ///
    /// - If `covers` is provided, it specifies individual coverage for each pixel in the span.
    /// - Otherwise, the span uses a single `cover` value.
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

    /// Blends a vertical span of pixels from `(x, y)`
    /// for `len` pixels with `colors` and either `covers` or `cover`.
    ///
    /// - If `covers` is provided, it specifies individual coverage for each pixel in the span.
    /// - Otherwise, the span uses a single `cover` value.
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

impl Pixel for Pixfmt<Rgba8> {
    impl_pixel_common!();

    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, color: C) {
        let bpp = Self::bpp();
        let color = Rgba8::from_trait(color).into_array3();
        let p = &mut self.rbuf[id][..n * bpp];
        for chunk in p.chunks_mut(bpp) {
            chunk.copy_from_slice(&color);
        }
    }
    fn bpp() -> usize {
        4
    }
    fn cover_mask() -> u64 {
        255
    }
    fn set<C: Color>(&mut self, id: (usize, usize), color: C) {
        let color = Rgba8::from_trait(color);
        assert!(!self.rbuf.data.is_empty());
        self.rbuf[id][0] = color.red8();
        self.rbuf[id][1] = color.green8();
        self.rbuf[id][2] = color.blue8();
        self.rbuf[id][3] = color.alpha8();
    }

    /// Compute **over** operator with coverage
    ///
    /// # Arguments
    ///   - id    - pixel at (`x`,`y`) - Premultiplied
    ///   - color - Color of Overlaying pixel, not premultiplied
    ///   - cover - Coverage of overlaying pixel, percent in 0p8 format
    ///
    /// # Output
    ///   - lerp(pixel(x,y), color, cover * alpha(color))
    fn blend_pix<C: Color>(&mut self, id: (usize, usize), color: C, cover: u64) {
        let alpha = multiply_u8(color.alpha8(), cover as u8);
        let pix0 = self.get(id); // Rgba8
        let pix = self.over(pix0, Rgba8::from_trait(color), alpha);
        self.set(id, pix);
    }
    fn fill<C: Color>(&mut self, color: C) {
        let n = 4;
        let bpp = Self::bpp();
        let c = Rgba8::from_trait(color).into_array4();
        let c2 = [
            c[0], c[1], c[2], c[3], c[0], c[1], c[2], c[3], c[0], c[1], c[2], c[3], c[0], c[1],
            c[2], c[3],
        ];
        let mut chunks = self.rbuf.data.chunks_exact_mut(bpp * n);
        for chunk in chunks.by_ref() {
            chunk.copy_from_slice(&c2);
        }
        for chunk in chunks.into_remainder().chunks_mut(bpp) {
            chunk.copy_from_slice(&c);
        }
    }

    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    fn to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), image::ImageError> {
        write_file(
            self.as_bytes(),
            self.width(),
            self.height(),
            filename,
            image::ColorType::Rgba8,
        )
    }
}

impl Pixel for Pixfmt<Rgb8> {
    impl_pixel_common!();

    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, color: C) {
        let bpp = Self::bpp();
        let color = Rgb8::from_trait(color).into_array3();
        let p = &mut self.rbuf[id][..bpp * n];
        for chunk in p.chunks_mut(bpp) {
            chunk.copy_from_slice(&color);
        }
    }
    fn set<C: Color>(&mut self, id: (usize, usize), color: C) {
        let color = Rgb8::from_trait(color).into_array3();
        let p = &mut self.rbuf[id][..3];
        p.copy_from_slice(&color);
        //p[0] = color.red8();
        //p[1] = color.green8();
        //p[2] = color.blue8();
    }
    fn bpp() -> usize {
        3
    }
    fn cover_mask() -> u64 {
        255
    }
    fn blend_pix<C: Color>(&mut self, id: (usize, usize), color: C, cover: u64) {
        let pix0 = self.raw(id);
        let pix = self.over(pix0, Rgb8::from_trait(color), color.alpha8(), cover);
        self.set(id, pix);
    }
    fn fill<C: Color>(&mut self, color: C) {
        let n = 4;
        let bpp = Self::bpp();
        let c = Rgb8::from_trait(color).into_array3();
        let c2 = [c[0], c[1], c[2], c[0], c[1], c[2], c[0], c[1], c[2], c[0], c[1], c[2]];
        let mut chunks = self.rbuf.data.chunks_exact_mut(bpp * n);
        for chunk in chunks.by_ref() {
            chunk.copy_from_slice(&c2);
        }
        for chunk in chunks.into_remainder().chunks_mut(bpp) {
            chunk.copy_from_slice(&c);
        }
    }

    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    fn to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), image::ImageError> {
        write_file(
            self.as_bytes(),
            self.width(),
            self.height(),
            filename,
            image::ColorType::Rgb8,
        )
    }
}

impl Pixel for Pixfmt<Rgba8pre> {
    impl_pixel_common!();

    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, color: C) {
        let bpp = Self::bpp();
        let color = Rgba8pre::from_trait(color).into_array4();
        let p = &mut self.rbuf[id][..n * bpp];
        for chunk in p.chunks_mut(bpp) {
            chunk.copy_from_slice(&color);
        }
    }
    fn set<C: Color>(&mut self, id: (usize, usize), color: C) {
        //let color = Rgba8pre::from(color);
        self.rbuf[id][0] = color.red8();
        self.rbuf[id][1] = color.green8();
        self.rbuf[id][2] = color.blue8();
        self.rbuf[id][3] = color.alpha8();
    }
    fn bpp() -> usize {
        4
    }
    fn cover_mask() -> u64 {
        255
    }
    fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
        let p = self.get(id);
        let p0 = Rgba8pre::new(p.red8(), p.green8(), p.blue8(), p.alpha8());
        let c0 = Rgba8pre::new(c.red8(), c.green8(), c.blue8(), c.alpha8());
        let p = self.over(p0, c0, c.alpha8(), cover);
        self.set(id, p);
    }
    fn fill<C: Color>(&mut self, color: C) {
        let n = 4;
        let bpp = Self::bpp();
        let c = Rgba8pre::from_trait(color).into_array4();
        let c2 = [
            c[0], c[1], c[2], c[3], c[0], c[1], c[2], c[3], c[0], c[1], c[2], c[3], c[0], c[1],
            c[2], c[3],
        ];
        let mut chunks = self.rbuf.data.chunks_exact_mut(bpp * n);
        for chunk in chunks.by_ref() {
            chunk.copy_from_slice(&c2);
        }
        for chunk in chunks.into_remainder().chunks_mut(bpp) {
            chunk.copy_from_slice(&c);
        }
    }
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    fn to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), image::ImageError> {
        write_file(
            self.as_bytes(),
            self.width(),
            self.height(),
            filename,
            image::ColorType::Rgba8,
        )
    }
}

impl Pixel for Pixfmt<Rgba32> {
    impl_pixel_common!();

    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, color: C) {
        for i in 0..n {
            self.set((id.0 + i, id.1), color);
        }
    }
    fn set<C: Color>(&mut self, id: (usize, usize), color: C) {
        let c = Rgba32::from_trait(color);
        assert!(!self.rbuf.data.is_empty());
        self.rbuf[id][0..4].copy_from_slice(&c.r.to_ne_bytes());
        self.rbuf[id][4..8].copy_from_slice(&c.g.to_ne_bytes());
        self.rbuf[id][8..12].copy_from_slice(&c.b.to_ne_bytes());
        self.rbuf[id][12..16].copy_from_slice(&c.a.to_ne_bytes());
    }
    fn bpp() -> usize {
        4 * 4
    }
    fn cover_mask() -> u64 {
        unimplemented!("no cover mask")
    }

    /// Applies the **over** operator with coverage for `Rgba32` (4x `f32` channels).
    ///
    /// # Arguments
    /// - `id`: Position of the pixel at `(x, y)`, premultiplied.
    /// - `color`: Overlaying pixel color, not premultiplied.
    /// - `cover`: Coverage of the overlaying pixel, percent in 0p8 format.
    fn blend_pix<C: Color>(&mut self, id: (usize, usize), color: C, cover: u64) {
        let cover_f64 = cover as f64 / 255.0;
        let alpha = color.alpha() * cover_f64;

        let pix0 = self.get(id);
        let r = color_u8_to_f64(pix0.r);
        let g = color_u8_to_f64(pix0.g);
        let b = color_u8_to_f64(pix0.b);
        let a = color_u8_to_f64(pix0.a);
        let pix = Rgba32::new(
            (r + (color.red() - r) * alpha) as f32,
            (g + (color.green() - g) * alpha) as f32,
            (b + (color.blue() - b) * alpha) as f32,
            (a + (color.alpha() - a) * alpha) as f32,
        );

        self.set(id, pix);
    }

    fn fill<C: Color>(&mut self, color: C) {
        let (w, h) = (self.width(), self.height());
        for i in 0..h {
            self.copy_hline(0, i, w, color);
        }
    }

    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    fn to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), image::ImageError> {
        write_file(
            self.as_bytes(),
            self.width(),
            self.height(),
            filename,
            image::ColorType::Rgba8,
        )
    }
}

impl Pixel for Pixfmt<Gray8> {
    impl_pixel_common!();

    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, color: C) {
        let bpp = Self::bpp();
        let c = Gray8::from_trait(color).into_array2();
        let p = &mut self.rbuf[id][..n * bpp];
        for chunk in p.chunks_mut(bpp) {
            chunk.copy_from_slice(&c);
        }
    }
    fn set<C: Color>(&mut self, id: (usize, usize), c: C) {
        let c = Gray8::from_trait(c);
        self.rbuf[id][0] = c.value;
        self.rbuf[id][1] = c.alpha;
    }
    fn cover_mask() -> u64 {
        255
    }
    fn bpp() -> usize {
        2
    }
    fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
        let alpha = multiply_u8(c.alpha8(), cover as u8);
        let p0 = self.over(id, Gray8::from_trait(c), alpha);
        self.set(id, p0);
    }
    fn fill<C: Color>(&mut self, color: C) {
        let n = 4;
        let bpp = Self::bpp();
        let c = Gray8::from_trait(color).into_array2();
        let c2 = [c[0], c[1], c[0], c[1], c[0], c[1], c[0], c[1]];
        let mut chunks = self.rbuf.data.chunks_exact_mut(bpp * n);
        for chunk in chunks.by_ref() {
            chunk.copy_from_slice(&c2);
        }
        for chunk in chunks.into_remainder().chunks_mut(bpp) {
            chunk.copy_from_slice(&c);
        }
    }

    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    fn to_file<P: AsRef<Path>>(&self, filename: P) -> Result<(), image::ImageError> {
        write_file(
            self.as_bytes(),
            self.width(),
            self.height(),
            filename,
            image::ColorType::L8,
        )
    }
}
