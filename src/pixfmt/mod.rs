//! Pixel Format

#[cfg(test)]
mod tests;

mod alpha_blend;
mod buffer;
mod pixel;
mod pixfmt;
pub use {alpha_blend::*, buffer::*, pixel::*, pixfmt::*};

// -----
use crate::color::*;

/// Access Pixel source color
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "no_std", feature = "alloc"))))
)]
pub trait PixelSource {
    // TODO:RETHINK
    fn get(&self, id: (usize, usize)) -> Rgba8;
}
impl PixelSource for Pixfmt<Rgb8> {
    fn get(&self, id: (usize, usize)) -> Rgba8 {
        let p = &self.rbuf[id];
        Rgba8::new(p[0], p[1], p[2], 255)
    }
}
impl PixelSource for Pixfmt<Rgba8> {
    fn get(&self, id: (usize, usize)) -> Rgba8 {
        let p = &self.rbuf[id];
        Rgba8::new(p[0], p[1], p[2], p[3])
    }
}
impl PixelSource for Pixfmt<Rgba8pre> {
    fn get(&self, id: (usize, usize)) -> Rgba8 {
        let p = &self.rbuf[id];
        Rgba8::new(p[0], p[1], p[2], p[3])
    }
}
impl PixelSource for Pixfmt<Rgba32> {
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
