//! Alphamask Adapator

use crate::{
    color::{Gray8, Rgb8, Rgba8},
    pixfmt::Pixfmt,
    util::{lerp_u8, multiply_u8},
    Color, Pixel, PixelSource,
};

/// Alpha Mask Adaptor
pub struct AlphaMaskAdaptor<T>
where
    Pixfmt<T>: Pixel + PixelSource,
{
    pub rgb: Pixfmt<T>,
    pub alpha: Pixfmt<Gray8>,
}

impl<T> AlphaMaskAdaptor<T>
where
    Pixfmt<T>: Pixel + PixelSource,
{
    /// Create a new Alpha Mask Adapator from a two PixelFormats
    pub fn new(rgb: Pixfmt<T>, alpha: Pixfmt<Gray8>) -> Self {
        Self { rgb, alpha }
    }
    /// Blend a set of colors starting at (x,y) with a length
    ///
    /// Background color is from the rgb image and
    /// alpha form the gray scale
    ///
    /// Calls [`blend_pix`].
    //
    // From https://stackoverflow.com/a/746937 :
    // out = alpha * new + (1 - alpha) * old
    //   p[j]  = out
    //   alpha = alpha
    //   new   = c
    //   old   = p[j]
    pub fn blend_color_hspan(
        &mut self,
        x: usize,
        y: usize,
        n: usize,
        colors: &[Rgb8],
        _cover: usize,
    ) {
        //for i in 0 .. n {
        //assert!(1==2);
        assert_eq!(n, colors.len());
        for (i, color) in colors.iter().enumerate() {
            let pix = &mut self.rgb.get((x + i, y));
            let alpha = u64::from(self.alpha.raw((x + i, y)).value);
            let pix = blend_pix(pix, color, alpha);
            self.rgb.set((x + i, y), pix);
        }
    }
}
/// Blends foreground and background pixels with a cover value.
///
/// Color components are computed by:
///
/// out = (alpha * cover) * (c - p)
///
/// Computations are conducted using fixed point math
///
/// see [Alpha Compositing](https://en.wikipedia.org/wiki/Alpha_compositing)
#[inline]
#[must_use]
pub fn blend_pix<C1: Color, C2: Color>(p: &C1, c: &C2, cover: u64) -> Rgba8 {
    assert!(c.alpha() >= 0.0);
    assert!(c.alpha() <= 1.0);

    let alpha = multiply_u8(c.alpha8(), cover as u8);

    let red = lerp_u8(p.red8(), c.red8(), alpha);
    let green = lerp_u8(p.green8(), c.green8(), alpha);
    let blue = lerp_u8(p.blue8(), c.blue8(), alpha);
    let alpha = lerp_u8(p.alpha8(), c.alpha8(), alpha);

    Rgba8::new(red, green, blue, alpha)
}
