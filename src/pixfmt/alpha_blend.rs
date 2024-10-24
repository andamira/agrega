use super::Pixfmt;
use crate::base::RenderingBase;
use crate::{color::*, util::*, Color, Pixel};
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::path::Path;

/// TODO
pub struct PixfmtAlphaBlend<'a, T, C>
where
    T: Pixel,
{
    ren: &'a mut RenderingBase<T>,
    offset: usize,
    //step: usize,
    phantom: PhantomData<C>,
}

impl<'a, T, C> PixfmtAlphaBlend<'a, T, C>
where
    T: Pixel,
{
    pub fn new(ren: &'a mut RenderingBase<T>, offset: usize) -> Self {
        //let step = T::bpp();
        Self { ren, offset, phantom: PhantomData }
    }
}
impl PixfmtAlphaBlend<'_, Pixfmt<Rgb8>, Gray8> {
    fn component(&self, c: Rgb8) -> Gray8 {
        match self.offset {
            0 => Gray8::new(c.r),
            1 => Gray8::new(c.g),
            2 => Gray8::new(c.b),
            _ => unreachable!("incorrect offset for Rgb8"),
        }
    }
    fn mix_pix(&mut self, id: (usize, usize), c: Gray8, alpha: u8) -> Gray8 {
        let p = self.component(Rgb8::from_slice(&self.ren.pixf.rbuf[id]));
        Gray8::new_with_alpha(lerp_u8(p.value, c.value, alpha), alpha)
    }
}

impl Pixel for PixfmtAlphaBlend<'_, Pixfmt<Rgb8>, Gray8> {
    fn width(&self) -> usize {
        self.ren.pixf.width()
    }
    fn height(&self) -> usize {
        self.ren.pixf.height()
    }
    fn as_bytes(&self) -> &[u8] {
        self.ren.pixf.as_bytes()
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
    fn fill<C: Color>(&mut self, color: C) {
        let (w, h) = (self.width(), self.height());
        for i in 0..w {
            for j in 0..h {
                self.set((i, j), color);
            }
        }
    }
    fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, c: C) {
        let c = Rgb8::from_trait(c);
        for i in 0..n {
            self.ren.pixf.rbuf[(id.0 + i, id.1)][self.offset] = self.component(c).value;
        }
    }
    fn set<C: Color>(&mut self, id: (usize, usize), c: C) {
        let c = Rgb8::from_trait(c);
        self.ren.pixf.rbuf[id][self.offset] = self.component(c).value;
    }
    fn cover_mask() -> u64 {
        Pixfmt::<Rgb8>::cover_mask()
    }
    fn bpp() -> usize {
        Pixfmt::<Rgb8>::bpp()
    }
    fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
        let alpha = multiply_u8(c.alpha8(), cover as u8);

        let c = Rgb8::from_trait(c);
        let c0 = self.component(c);
        let p0 = self.mix_pix(id, c0, alpha);
        self.set(id, p0);
    }

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
        } else if cover == Self::cover_mask() {
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
