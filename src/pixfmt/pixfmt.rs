//

use super::RenderingBuffer;
#[cfg(feature = "std")]
use crate::read_file;
use crate::{color::*, util::*, Color, Pixel, PixelSource};
use devela::{iif, PhantomData, Vec};

/// Pixel format wrapper around raw pixel component data.
///
/// Provides methods for creating and manipulating pixel data with various color
/// and line drawing operations. Utilizes a row-major order for storage.
#[must_use]
#[derive(Clone, Debug, Default)]
pub struct Pixfmt<T> {
    /// Underlying rendering buffer storing pixel data.
    pub(super) rbuf: RenderingBuffer,
    phantom: PhantomData<T>,
}

/// # Common Pixfmt methods.
impl<T> Pixfmt<T>
where
    Pixfmt<T>: Pixel,
{
    /// Creates a new `Pixfmt` with the specified `width` and `height`.
    ///
    /// Allocates memory for pixel data based on `width * height * bpp`.
    /// # Panics
    ///
    /// Panics if `width` or `height` is zero.
    #[inline]
    pub fn new(width: usize, height: usize) -> Self {
        if width == 0 || height == 0 {
            panic!("Cannot create pixfmt with 0 width or height");
        }
        Self { rbuf: RenderingBuffer::new(width, height, Self::bpp()), phantom: PhantomData }
    }
    // /// Fill with a color
    // pub fn fill<C: Color>(&mut self, color: C) {
    //     let (w,h) = (self.width(), self.height());
    //     for i in 0 .. w {
    //         for j in 0 .. h {
    //             self.set((i,j),color);
    //         }
    //     }
    // }

    /// Returns the size of the rendering buffer in bytes, calculated as `width * height * bpp`.
    #[inline]
    #[must_use]
    pub fn size(&self) -> usize {
        self.rbuf.len()
    }

    /// Clears the rendering buffer, setting all color components to `255` (typically white).
    ///
    /// # Examples
    /// ```
    /// use agrega::{Pixfmt, Rgb8, Rgba8, PixelSource};
    ///
    /// // Pixfmt with Rgb8, not Alpha Component
    /// let mut pix = Pixfmt::<Rgb8>::new(2,2);
    /// pix.clear();
    /// let empty = Rgba8 { r:255, g:255, b:255, a:255};
    /// assert_eq!(pix.get((0,0)), empty);
    /// assert_eq!(pix.get((0,1)), empty);
    /// assert_eq!(pix.get((1,0)), empty);
    /// assert_eq!(pix.get((1,1)), empty);
    ///
    /// // Pixfmt with Rgba8, including Alpha Component
    /// let mut pix = Pixfmt::<Rgba8>::new(2,2);
    /// pix.clear();
    /// let empty = Rgba8 { r:255, g:255, b:255, a:255};
    /// assert_eq!(pix.get((0,0)), empty);
    /// assert_eq!(pix.get((0,1)), empty);
    /// assert_eq!(pix.get((1,0)), empty);
    /// assert_eq!(pix.get((1,1)), empty);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.rbuf.clear();
    }

    //pub fn from(rbuf: RenderingBuffer) -> Self {
    //    Self { rbuf, phantom: PhantomData }
    //}

    /// Copies the given `color` to the pixel at `(x, y)`.
    ///
    /// Coordinates outside the buffer range are ignored.
    ///
    /// # Examples
    /// ```
    /// use agrega::{Pixfmt, Rgba8, PixelSource};
    ///
    /// let mut pix = Pixfmt::<Rgba8>::new(1,2);
    /// let black = Rgba8::black();
    /// pix.copy_pixel(0,1, black);
    /// assert_eq!(pix.get((0,0)), Rgba8{r:0, g:0, b:0, a:0});
    /// assert_eq!(pix.get((0,1)), black);
    ///
    /// pix.copy_pixel(10,10, black); // Ignored, outside of range
    /// ```
    pub fn copy_pixel<C: Color>(&mut self, x: usize, y: usize, color: C) {
        iif![x >= self.rbuf.width || y >= self.rbuf.height; return];
        self.set((x, y), color);
    }

    /// Copies the specified `color` horizontally from `(x, y)` over `n` pixels.
    ///
    /// Pixels outside the buffer range are ignored.
    ///
    /// # Examples
    /// ```
    /// use agrega::{Pixfmt, Rgb8, Rgba8, PixelSource};
    ///
    /// let mut pix = Pixfmt::<Rgb8>::new(10,1);
    /// let black = Rgba8::black();
    /// pix.copy_hline(0,0,10, black);
    /// assert_eq!(pix.get((0,0)), black);
    /// assert_eq!(pix.get((1,0)), black);
    /// assert_eq!(pix.get((9,0)), black);
    ///
    /// pix.copy_hline(1,1,10, black); // Ignored, outside of range
    /// ```
    pub fn copy_hline<C: Color>(&mut self, x: usize, y: usize, n: usize, color: C) {
        iif![y >= self.rbuf.height || x >= self.rbuf.width || n == 0; return];
        let n = iif![x + n >= self.rbuf.width; self.rbuf.width - x; n];
        for i in 0..n {
            self.set((x + i, y), color);
        }
    }

    /// Copies the given `color` vertically from `(x, y)` over `n` pixels.
    ///
    /// Locations outside of the region are ignored
    ///
    /// # Examples
    /// ```
    /// use agrega::{Pixfmt, Rgba8, Rgba32, PixelSource};
    ///
    /// let mut pix = Pixfmt::<Rgba32>::new(1,10);
    /// let black  = Rgba32::new(0.,0.,0.,1.);
    /// pix.copy_vline(0,0,10, black);
    ///
    /// let black8 = Rgba8::from_trait(black); // pix.get() returns Rgba8
    /// assert_eq!(pix.get((0,0)), black8);
    /// assert_eq!(pix.get((0,1)), black8);
    /// assert_eq!(pix.get((0,9)), black8);
    ///
    /// pix.copy_vline(1,1,10, black); // Ignored, outside of range
    /// ```
    pub fn copy_vline<C: Color>(&mut self, x: usize, y: usize, n: usize, color: C) {
        iif![y >= self.rbuf.height || x >= self.rbuf.width || n == 0; return];
        let n = iif![y + n >= self.rbuf.height; self.rbuf.height - y; n];
        for i in 0..n {
            self.set((x, y + i), color);
        }
    }

    /// Loads pixel data from a file.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub fn from_file<P: AsRef<std::path::Path>>(filename: P) -> Result<Self, image::ImageError> {
        let (buf, w, h) = read_file(filename)?;
        Ok(Self { rbuf: RenderingBuffer::from_vec(buf, w, h, 3), phantom: PhantomData })
    }
}

impl Pixfmt<Gray8> {
    /// Mixes the given grayscale color `c` with the pixel at `id`, using `alpha` for blending.
    ///
    /// Returns the blended color.
    #[inline]
    #[must_use]
    pub fn over(&mut self, id: (usize, usize), c: Gray8, alpha: u8) -> Gray8 {
        let p = Gray8::from_slice(&self.rbuf[id]);
        Gray8::new_with_alpha(lerp_u8(p.value, c.value, alpha), alpha)
    }

    /// Retrieves the raw grayscale pixel at the given position `id`.
    #[inline]
    #[must_use]
    pub fn raw(&self, id: (usize, usize)) -> Gray8 {
        Gray8::from_slice(&self.rbuf[id])
    }
}

impl Pixfmt<Rgb8> {
    /// Retrieves the raw RGB pixel at the given position `id`.
    #[inline]
    #[must_use]
    pub fn raw(&self, id: (usize, usize)) -> Rgb8 {
        let p = &self.rbuf[id];
        Rgb8::new(p[0], p[1], p[2])
    }

    /// Computes the **over** operator for blending.
    ///
    /// # Arguments
    /// - `p`: The current pixel (premultiplied).
    /// - `c`: The overlaying pixel (not premultiplied).
    /// - `alpha`: Alpha value for blending.
    /// - `cover`: Coverage factor for the overlay.
    ///
    /// # Output
    /// Returns a new `Rgb8` blended color.
    #[must_use]
    pub fn over(&mut self, p: Rgb8, c: Rgb8, alpha: u8, cover: u64) -> Rgb8 {
        let a = multiply_u8(alpha, cover as u8);
        let (r, g, b) = (lerp_u8(p.r, c.r, a), lerp_u8(p.g, c.g, a), lerp_u8(p.b, c.b, a));
        Rgb8::new(r, g, b)
    }
}

impl Pixfmt<Rgba8> {
    /// Computes the **over** operator for RGBA blending.
    ///
    /// # Arguments
    /// - `p`: The current pixel (premultiplied).
    /// - `c`: The overlaying pixel (not premultiplied).
    /// - `alpha`: Alpha value for blending.
    ///
    /// # Output
    /// Returns a new `Rgba8` blended color.
    #[must_use]
    pub fn over(&mut self, p: Rgba8, c: Rgba8, alpha: u8) -> Rgba8 {
        let red = lerp_u8(p.r, c.r, alpha);
        let green = lerp_u8(p.g, c.g, alpha);
        let blue = lerp_u8(p.b, c.b, alpha);
        let alpha = prelerp_u8(p.a, alpha, alpha);
        Rgba8::new(red, green, blue, alpha)
    }

    /// Blends the given color `c` into the pixel at `id`, with the specified `cover` factor.
    fn _blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
        let alpha = multiply_u8(c.alpha8(), cover as u8);
        let pix0 = self.get(id);
        let pix = self.over(pix0, Rgba8::from_trait(c), alpha);
        self.set(id, pix);
    }
}

impl Pixfmt<Rgba8pre> {
    /// Computes the **over** operator for premultiplied RGBA blending.
    ///
    /// # Arguments
    /// - `p`: The current pixel (premultiplied).
    /// - `c`: The overlaying pixel (premultiplied).
    /// - `alpha`: Alpha value for blending.
    /// - `cover`: Coverage factor for the overlay.
    ///
    /// # Output
    /// Returns a new `Rgba8pre` blended color.
    #[must_use]
    pub fn over(&mut self, p: Rgba8pre, c: Rgba8pre, alpha: u8, cover: u64) -> Rgba8pre {
        let alpha = multiply_u8(alpha, cover as u8);
        let red = multiply_u8(c.r, cover as u8);
        let green = multiply_u8(c.g, cover as u8);
        let blue = multiply_u8(c.b, cover as u8);

        let red = prelerp_u8(p.r, red, alpha);
        let green = prelerp_u8(p.g, green, alpha);
        let blue = prelerp_u8(p.b, blue, alpha);
        let alpha = prelerp_u8(p.a, alpha, alpha);
        Rgba8pre::new(red, green, blue, alpha)
    }

    /// Drops the alpha component, creating a `Pixfmt<Rgb8>` without transparency.
    ///
    /// This method filters out the alpha component in the underlying data buffer.
    pub fn drop_alpha(&self) -> Pixfmt<Rgb8> {
        let buf: Vec<_> = self
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 4 < 3)
            .map(|(_, x)| *x)
            .collect();
        Pixfmt::<Rgb8> {
            rbuf: RenderingBuffer::from_vec(buf, self.width(), self.height(), 3),
            phantom: PhantomData,
        }
    }
}
