use super::buffer::RenderingBuffer;
use crate::{color::*, util::*, Color, Pixel, Source};
use alloc::vec::Vec;
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::path::Path;

/// Pixel Format Wrapper around raw pixel component data.
#[derive(Debug)]
pub struct Pixfmt<T> {
    pub(super) rbuf: RenderingBuffer,
    phantom: PhantomData<T>,
}

impl<T> Pixfmt<T>
where
    Pixfmt<T>: Pixel,
{
    /// Create new Pixel Format of width * height * bpp
    ///
    /// Allocates memory of width * height * bpp
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

    /// Size of Rendering Buffer in bytes; width * height * bpp
    pub fn size(&self) -> usize {
        self.rbuf.len()
    }

    /// Clear the Image
    ///
    /// All color components are set to 255, including `alpha` if present
    ///
    /// # Examples
    /// ```
    /// use agrega::{Pixfmt, Rgb8, Rgba8, Source};
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
    /// let mut pix = Pixfmt::<Rgb8>::new(2,2);
    /// pix.clear();
    /// let empty = Rgba8 { r:255, g:255, b:255, a:255};
    /// assert_eq!(pix.get((0,0)), empty);
    /// assert_eq!(pix.get((0,1)), empty);
    /// assert_eq!(pix.get((1,0)), empty);
    /// assert_eq!(pix.get((1,1)), empty);
    /// ```
    pub fn clear(&mut self) {
        self.rbuf.clear();
    }

    //pub fn from(rbuf: RenderingBuffer) -> Self {
    //    Self { rbuf, phantom: PhantomData }
    //}

    /// Copies the [Color] `c` to pixel at (`x`,`y`)
    ///
    /// Locations outside of the region are igorned
    ///
    /// # Examples
    /// ```
    /// use agrega::{Pixfmt, Rgba8, Source};
    ///
    /// let mut pix = Pixfmt::<Rgba8>::new(1,2);
    /// let black = Rgba8::black();
    /// pix.copy_pixel(0,1, black);
    /// assert_eq!(pix.get((0,0)), Rgba8{r:0, g:0, b:0, a:0});
    /// assert_eq!(pix.get((0,1)), black);
    ///
    /// pix.copy_pixel(10,10, black); // Ignored, outside of range
    /// ```
    /// [Color]: ../trait.Color.html
    pub fn copy_pixel<C: Color>(&mut self, x: usize, y: usize, c: C) {
        if x >= self.rbuf.width || y >= self.rbuf.height {
            return;
        }
        self.set((x, y), c);
    }

    /// Copies the [Color] `c` to pixels from (`x`,`y`) to (`x+n-1`,y)
    ///
    /// Locations outside of the region are ignored
    ///
    /// # Examples
    /// ```
    /// use agrega::{Pixfmt, Rgb8, Rgba8, Source};
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
    /// [Color]: ../trait.Color.html
    pub fn copy_hline<C: Color>(&mut self, x: usize, y: usize, n: usize, c: C) {
        if y >= self.rbuf.height || x >= self.rbuf.width || n == 0 {
            return;
        }
        let n = if x + n >= self.rbuf.width {
            self.rbuf.width - x
        } else {
            n
        };
        for i in 0..n {
            self.set((x + i, y), c);
        }
    }

    /// Copies the [Color] `c` to pixels from (`x`,`y`) to (`x`,`y+n-1`)
    ///
    /// Locations outside of the region are ignored
    ///
    /// # Examples
    /// ```
    /// use agrega::{Pixfmt, Rgba8, Rgba32, Source};
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
    ///
    /// [Color]: ../trait.Color.html
    /// [Rgba8]: ../Color/struct.Rgba8.html
    pub fn copy_vline<C: Color>(&mut self, x: usize, y: usize, n: usize, c: C) {
        if y >= self.rbuf.height || x >= self.rbuf.width || n == 0 {
            return;
        }
        let n = if y + n >= self.rbuf.height {
            self.rbuf.height - y
        } else {
            n
        };
        for i in 0..n {
            self.set((x, y + i), c);
        }
    }

    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self, image::ImageError> {
        let (buf, w, h) = crate::ppm::read_file(filename)?;
        Ok(Self { rbuf: RenderingBuffer::from_buf(buf, w, h, 3), phantom: PhantomData })
    }
}

impl Source for Pixfmt<Rgba8> {
    fn get(&self, id: (usize, usize)) -> Rgba8 {
        let p = &self.rbuf[id];
        Rgba8::new(p[0], p[1], p[2], p[3])
    }
}
impl Source for Pixfmt<Rgba8pre> {
    fn get(&self, id: (usize, usize)) -> Rgba8 {
        let p = &self.rbuf[id];
        Rgba8::new(p[0], p[1], p[2], p[3])
    }
}
impl Source for Pixfmt<Rgb8> {
    fn get(&self, id: (usize, usize)) -> Rgba8 {
        let p = &self.rbuf[id];
        Rgba8::new(p[0], p[1], p[2], 255)
    }
}
impl Source for Pixfmt<Rgba32> {
    fn get(&self, id: (usize, usize)) -> Rgba8 {
        //let n = (id.0 + id.1 * self.rbuf.width) * Pixfmt::<Rgba32>::bpp();
        let p = &self.rbuf[id];
        let red: f32 = f32::from_ne_bytes([p[0], p[1], p[2], p[3]]);
        let green: f32 = f32::from_ne_bytes([p[4], p[5], p[6], p[7]]);
        let blue: f32 = f32::from_ne_bytes([p[8], p[9], p[10], p[11]]);
        let alpha: f32 = f32::from_ne_bytes([p[12], p[13], p[14], p[15]]);
        let c = Rgba32::new(red, green, blue, alpha);
        Rgba8::from_trait(c)
    }
}

macro_rules! impl_pixel {
    () => {
        /// Height of rendering buffer in pixels
        fn height(&self) -> usize {
            self.rbuf.height
        }
        /// Width of rendering buffer in pixels
        fn width(&self) -> usize {
            self.rbuf.width
        }
        /// Return a underlying raw pixel/component data
        fn as_bytes(&self) -> &[u8] {
            &self.rbuf.data
        }
    };
}

impl Pixel for Pixfmt<Rgba8> {
    impl_pixel!();
    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, c: C) {
        let bpp = Self::bpp();
        let c = Rgba8::from_trait(c).into_slice();
        let p = &mut self.rbuf[id][..n * bpp];
        for chunk in p.chunks_mut(bpp) {
            chunk.copy_from_slice(&c);
        }
    }
    fn bpp() -> usize {
        4
    }
    fn cover_mask() -> u64 {
        255
    }
    fn set<C: Color>(&mut self, id: (usize, usize), c: C) {
        let c = Rgba8::from_trait(c);
        assert!(!self.rbuf.data.is_empty());
        self.rbuf[id][0] = c.red8();
        self.rbuf[id][1] = c.green8();
        self.rbuf[id][2] = c.blue8();
        self.rbuf[id][3] = c.alpha8();
    }

    /// Compute **over** operator with coverage
    ///
    /// # Arguments
    ///   - id   - pixel at (`x`,`y`) - Premultiplied
    ///   - c    - Color of Overlaying pixel, not premultiplied
    ///   - cover - Coverage of overlaying pixel, percent in 0p8 format
    ///
    /// # Output
    ///   - lerp(pixel(x,y), color, cover * alpha(color))
    ///
    fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
        let alpha = multiply_u8(c.alpha8(), cover as u8);
        let pix0 = self.get(id); // Rgba8
        let pix = self.mix_pix(pix0, Rgba8::from_trait(c), alpha);
        self.set(id, pix);
    }
    fn fill<C: Color>(&mut self, color: C) {
        let n = 4;
        let bpp = Self::bpp();
        let c = Rgba8::from_trait(color).into_slice();
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
        crate::ppm::write_file(
            self.as_bytes(),
            self.width(),
            self.height(),
            filename,
            image::ColorType::Rgba8,
        )
    }
}

impl Pixel for Pixfmt<Rgb8> {
    impl_pixel!();
    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, c: C) {
        let bpp = Self::bpp();
        let c = Rgb8::from_trait(c).into_slice();
        let p = &mut self.rbuf[id][..bpp * n];
        for chunk in p.chunks_mut(bpp) {
            chunk.copy_from_slice(&c);
        }
    }
    fn set<C: Color>(&mut self, id: (usize, usize), c: C) {
        let c = Rgb8::from_trait(c).into_slice();
        let p = &mut self.rbuf[id][..3];
        p.copy_from_slice(&c);
        //p[0] = c.red8();
        //p[1] = c.green8();
        //p[2] = c.blue8();
    }
    fn bpp() -> usize {
        3
    }
    fn cover_mask() -> u64 {
        255
    }
    fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
        let pix0 = self.raw(id);
        let pix = self.mix_pix(pix0, Rgb8::from_trait(c), c.alpha8(), cover);
        self.set(id, pix);
    }
    fn fill<C: Color>(&mut self, color: C) {
        let n = 4;
        let bpp = Self::bpp();
        let c = Rgb8::from_trait(color).into_slice();
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
        crate::ppm::write_file(
            self.as_bytes(),
            self.width(),
            self.height(),
            filename,
            image::ColorType::Rgb8,
        )
    }
}
impl Pixfmt<Gray8> {
    fn mix_pix(&mut self, id: (usize, usize), c: Gray8, alpha: u8) -> Gray8 {
        let p = Gray8::from_slice(&self.rbuf[id]);
        Gray8::new_with_alpha(lerp_u8(p.value, c.value, alpha), alpha)
    }
    pub fn raw(&self, id: (usize, usize)) -> Gray8 {
        Gray8::from_slice(&self.rbuf[id])
    }
}

impl Pixfmt<Rgba8> {
    /// Computer **over** operator
    ///
    /// # Arguments
    ///   - p     - Current pixel, premultipled
    ///   - c     - Overlaying pixel, not premultipled
    ///   - alpha - Alpha Channel
    ///
    /// # Output
    ///   - lerp(p, c, alpha)
    ///
    /// **Change function name to over**
    fn mix_pix(&mut self, p: Rgba8, c: Rgba8, alpha: u8) -> Rgba8 {
        let red = lerp_u8(p.r, c.r, alpha);
        let green = lerp_u8(p.g, c.g, alpha);
        let blue = lerp_u8(p.b, c.b, alpha);
        let alpha = prelerp_u8(p.a, alpha, alpha);
        Rgba8::new(red, green, blue, alpha)
    }
    fn _blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
        let alpha = multiply_u8(c.alpha8(), cover as u8);
        let pix0 = self.get(id);
        let pix = self.mix_pix(pix0, Rgba8::from_trait(c), alpha);
        self.set(id, pix);
    }
}
impl Pixel for Pixfmt<Rgba8pre> {
    impl_pixel!();
    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, c: C) {
        let bpp = Self::bpp();
        let c = Rgba8pre::from_trait(c).into_slice();
        let p = &mut self.rbuf[id][..n * bpp];
        for chunk in p.chunks_mut(bpp) {
            chunk.copy_from_slice(&c);
        }
    }
    fn set<C: Color>(&mut self, id: (usize, usize), c: C) {
        //let c = Rgba8pre::from(c);
        self.rbuf[id][0] = c.red8();
        self.rbuf[id][1] = c.green8();
        self.rbuf[id][2] = c.blue8();
        self.rbuf[id][3] = c.alpha8();
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
        let p = self.mix_pix(p0, c0, c.alpha8(), cover);
        self.set(id, p);
    }
    fn fill<C: Color>(&mut self, color: C) {
        let n = 4;
        let bpp = Self::bpp();
        let c = Rgba8pre::from_trait(color).into_slice();
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
        crate::ppm::write_file(
            self.as_bytes(),
            self.width(),
            self.height(),
            filename,
            image::ColorType::Rgba8,
        )
    }
}

impl Pixfmt<Rgb8> {
    pub fn raw(&self, id: (usize, usize)) -> Rgb8 {
        let p = &self.rbuf[id];
        Rgb8::new(p[0], p[1], p[2])
    }
    /// Compute **over** operator
    ///
    /// # Arguments
    ///   - p     - Current pixel, premultipled (wow that is confusing)
    ///   - c     - Overlaying pixel, not premultiplied
    ///   - alpha - Alpha channel
    ///   - cover - Coverage
    ///
    /// # Output
    ///   - lerp( p, c, alpha * cover)
    ///
    fn mix_pix(&mut self, p: Rgb8, c: Rgb8, alpha: u8, cover: u64) -> Rgb8 {
        let alpha = multiply_u8(alpha, cover as u8);
        let red = lerp_u8(p.r, c.r, alpha);
        let green = lerp_u8(p.g, c.g, alpha);
        let blue = lerp_u8(p.b, c.b, alpha);
        Rgb8::new(red, green, blue)
    }
}
impl Pixfmt<Rgba8pre> {
    /// Compute **over** operator
    ///
    /// # Arguments
    ///   - p     - Current pixel, premultipled
    ///   - c     - Overlaying pixel, premultiplied
    ///   - alpha - Alpha channel
    ///   - cover - Coverage
    ///
    /// # Output
    ///   - prelerp(p, c * cover, alpha * cover)
    ///
    fn mix_pix(&mut self, p: Rgba8pre, c: Rgba8pre, alpha: u8, cover: u64) -> Rgba8pre {
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
    pub fn drop_alpha(&self) -> Pixfmt<Rgb8> {
        let buf: Vec<_> = self
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 4 < 3)
            .map(|(_, x)| *x)
            .collect();
        Pixfmt::<Rgb8> {
            rbuf: RenderingBuffer::from_buf(buf, self.width(), self.height(), 3),
            phantom: PhantomData,
        }
    }
}

impl Pixel for Pixfmt<Rgba32> {
    impl_pixel!();
    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, c: C) {
        for i in 0..n {
            self.set((id.0 + i, id.1), c);
        }
    }
    fn set<C: Color>(&mut self, id: (usize, usize), c: C) {
        let c = Rgba32::from_trait(c);
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
    fn blend_pix<C: Color>(&mut self, _id: (usize, usize), _c: C, _cover: u64) {
        unimplemented!("no blending");
        /*
        let alpha = multiply_u8(c.alpha8(), cover as u8);
        let pix0 = self.get(id); // Rgba8
        let pix  = self.mix_pix(&pix0, &Rgba8::from(c), alpha);
        self.set(id, &pix);
         */
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
        crate::ppm::write_file(
            self.as_bytes(),
            self.width(),
            self.height(),
            filename,
            image::ColorType::Rgba8,
        )
    }
}

impl Pixel for Pixfmt<Gray8> {
    impl_pixel!();
    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, color: C) {
        let bpp = Self::bpp();
        let c = Gray8::from_trait(color).into_slice();
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
        let p0 = self.mix_pix(id, Gray8::from_trait(c), alpha);
        self.set(id, p0);
    }
    fn fill<C: Color>(&mut self, color: C) {
        let n = 4;
        let bpp = Self::bpp();
        let c = Gray8::from_trait(color).into_slice();
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
        crate::ppm::write_file(
            self.as_bytes(),
            self.width(),
            self.height(),
            filename,
            image::ColorType::L8,
        )
    }
}
