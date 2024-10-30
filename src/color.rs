//! Colors
//
// TOC
// - utilities, mostly private
// - public definitions
// - implementations
// - tests

use crate::util::multiply_u8;
#[allow(unused_imports)]
use devela::{iif, ExtFloat};

/* utils */

/// Convert an f64 \[0‥1\] component to a u8 \[0‥255\] component
#[inline] #[must_use] #[rustfmt::skip]
fn cu8(v: f64) -> u8 { (v * 255.0).round() as u8 }

/// Convert from sRGB to RGB for a single component
#[inline] #[must_use] #[rustfmt::skip]
fn srgb_to_rgb(x: f64) -> f64 {
    if x <= 0.04045 { x / 12.92 } else { ((x + 0.055) / 1.055).powf(2.4) }
}
/// Convert from RGB to sRGB for a single component
#[inline] #[must_use] #[rustfmt::skip]
fn rgb_to_srgb(x: f64) -> f64 {
    if x <= 0.003_130_8 { x * 12.92 } else { 1.055 * x.powf(1.0 / 2.4) - 0.055 }
}

// Converts a `u8` color component to `f64` in the range [0.0, 1.0].
#[inline] #[must_use] #[rustfmt::skip]
pub(crate) fn color_u8_to_f64(x: u8) -> f64 { f64::from(x) / 255.0 }

// Computes the luminance of an RGB color in `u8` and returns it as `u8`.
#[inline] #[must_use] #[rustfmt::skip]
fn luminance_u8(red: u8, green: u8, blue: u8) -> u8 {
    (luminance(color_u8_to_f64(red), color_u8_to_f64(green), color_u8_to_f64(blue)) * 255.0).round()
        as u8
}

/// Returns the luminance.
#[inline] #[must_use] #[rustfmt::skip]
pub const fn luminance(red: f64, green: f64, blue: f64) -> f64 {
    0.2126 * red + 0.7152 * green + 0.0722 * blue
}

/// Returns the lightness.
///
/// $ (\text{max}(R, G, B) + \text{min}(R, G, B)) / 2 $
#[inline] #[must_use] #[rustfmt::skip]
pub const fn lightness(red: f64, green: f64, blue: f64) -> f64 {
    let (mut cmax, mut cmin) = (red, red);
    if green > cmax { cmax = green; }
    if blue > cmax { cmax = blue; }
    if green < cmin { cmin = green; }
    if blue < cmin { cmin = blue; }
    (cmax + cmin) / 2.0
}

/// Returns the average.
#[inline] #[must_use] #[rustfmt::skip]
pub const fn average(red: f64, green: f64, blue: f64) -> f64 {
    (red + green + blue) / 3.0
}

/* public definitions */

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

/// Grayscale color with optional transparency.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Gray8 {
    /// Grayscale intensity (0-255).
    pub value: u8,
    /// Alpha transparency (0-255).
    pub alpha: u8,
}

impl Gray8 {
    /// Creates a new opaque grayscale color.
    #[inline]
    #[must_use]
    pub const fn new(value: u8) -> Self {
        Self { value, alpha: 255 }
    }
    /// Creates a grayscale color with specified alpha.
    #[inline]
    #[must_use]
    pub const fn new_with_alpha(value: u8, alpha: u8) -> Self {
        Self { value, alpha }
    }

    /// Converts a `Color` trait type to grayscale.
    #[inline]
    #[must_use]
    pub fn from_trait<C: Color>(c: C) -> Self {
        let lum = luminance_u8(c.red8(), c.green8(), c.blue8());
        Self::new_with_alpha(lum, c.alpha8())
    }
    /// Converts a two-element slice `[value, alpha]` to a grayscale color.
    ///
    /// # Panics
    /// Panics if the slice length is less than 2.
    #[inline]
    #[must_use]
    pub const fn from_slice(v: &[u8]) -> Self {
        Self::new_with_alpha(v[0], v[1])
    }

    /// Converts a two-element array `[value, alpha]` to a grayscale color.
    #[inline]
    #[must_use]
    pub const fn from_array2(v: [u8; 2]) -> Self {
        Self::new_with_alpha(v[0], v[1])
    }

    /// Returns the grayscale and alpha components as an array `[value, alpha]`.
    #[inline]
    #[must_use]
    pub const fn into_array2(&self) -> [u8; 2] {
        [self.value, self.alpha]
    }
}

/// RGB color with Red, Green, and Blue components.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rgb8 {
    /// Red channel (0-255).
    pub r: u8,
    /// Green channel (0-255).
    pub g: u8,
    /// Blue channel (0-255).
    pub b: u8,
}

impl Rgb8 {
    /// Creates a new `Rgb8` color.
    #[inline]
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb8 { r, g, b }
    }

    /// Converts a `Color` trait type to RGB.
    #[inline]
    #[must_use]
    pub fn from_trait<C: Color>(c: C) -> Self {
        Self::new(c.red8(), c.green8(), c.blue8())
    }

    /// Creates an `Rgb8` color from wavelength and gamma correction.
    #[must_use]
    pub fn from_wavelength_gamma(w: f64, gamma: f64) -> Self {
        let (r, g, b) = if (380.0..=440.0).contains(&w) {
            (-1.0 * (w - 440.0) / (440.0 - 380.0), 0.0, 1.0)
        } else if (440.0..=490.0).contains(&w) {
            (0.0, (w - 440.0) / (490.0 - 440.0), 1.0)
        } else if (490.0..=510.0).contains(&w) {
            (0.0, 1.0, -1.0 * (w - 510.0) / (510.0 - 490.0))
        } else if (510.0..=580.0).contains(&w) {
            ((w - 510.0) / (580.0 - 510.0), 1.0, 0.0)
        } else if (580.0..=645.0).contains(&w) {
            (1.0, -1.0 * (w - 645.0) / (645.0 - 580.0), 0.0)
        } else if (645.0..=780.0).contains(&w) {
            (1.0, 0.0, 0.0)
        } else {
            (0., 0., 0.)
        };
        let scale = if w > 700.0 {
            0.3 + 0.7 * (780.0 - w) / (780.0 - 700.0)
        } else if w < 420.0 {
            0.3 + 0.7 * (w - 380.0) / (420.0 - 380.0)
        } else {
            1.0
        };
        let r = (r * scale).powf(gamma) * 255.0;
        let g = (g * scale).powf(gamma) * 255.0;
        let b = (b * scale).powf(gamma) * 255.0;
        Self::new(r as u8, g as u8, b as u8)
    }

    /// Returns pure white color `(255, 255, 255)`.
    #[inline]
    #[must_use]
    pub const fn white() -> Self {
        Self::new(255, 255, 255)
    }
    /// Returns pure black color `(0, 0, 0)`.
    #[inline]
    #[must_use]
    pub const fn black() -> Self {
        Self::new(0, 0, 0)
    }
    /// Returns a grayscale color `(g, g, g)` based on a single intensity value.
    #[inline]
    #[must_use]
    pub const fn gray(g: u8) -> Self {
        Self::new(g, g, g)
    }

    /// Converts a slice `[r, g, b]` to an `Rgb8` color.
    ///
    /// # Panics
    /// Panics if the slice length is less than 3.
    #[inline]
    #[must_use]
    pub const fn from_slice(v: &[u8]) -> Self {
        Rgb8 { r: v[0], g: v[1], b: v[2] }
    }

    /// Converts a three-element array `[r, g, b]` to an `Rgba8` color.
    #[inline]
    #[must_use]
    pub const fn from_array3(v: [u8; 3]) -> Self {
        Self::new(v[0], v[1], v[2])
    }
    /// Converts a three-element array `[r, g, b]` to an `Rgba8` color.
    #[inline]
    #[must_use]
    pub const fn from_array4(v: [u8; 4]) -> Self {
        Self::new(v[0], v[1], v[2])
    }

    /// Returns the color components as an array `[r, g, b]`.
    #[inline]
    #[must_use]
    pub const fn into_array3(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
    /// Returns the color components as an array `[r, g, b]`.
    #[inline]
    #[must_use]
    pub const fn into_array4(&self) -> [u8; 4] {
        [self.r, self.g, self.b, u8::MAX]
    }
}

/// Represents a color with Red, Green, Blue, and Alpha (`u8` values).
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rgba8 {
    /// Red channel (0-255).
    pub r: u8,
    /// Green channel (0-255).
    pub g: u8,
    /// Blue channel (0-255).
    pub b: u8,
    /// Alpha channel (0-255).
    pub a: u8,
}

impl Rgba8 {
    /// Creates a new `Rgba8` color from red, green, blue, and alpha components.
    #[inline]
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Rgba8 { r, g, b, a }
    }
    /// Constructs `Rgba8` from any type implementing `Color` trait.
    #[inline]
    #[must_use]
    pub fn from_trait<C: Color>(c: C) -> Self {
        Self::new(c.red8(), c.green8(), c.blue8(), c.alpha8())
    }
    /// Creates a color from a wavelength and gamma correction factor.
    #[inline]
    #[must_use]
    pub fn from_wavelength_gamma(w: f64, gamma: f64) -> Self {
        let c = Rgb8::from_wavelength_gamma(w, gamma);
        Self::from_trait(c) // IMPROVE
    }

    /// Converts a four-element array `[r, g, b, alpha]` to an `Rgba8` color.
    #[inline]
    pub const fn from_array(v: [u8; 4]) -> Self {
        Self::from_array4(v)
    }
    /// Converts a four-element array `[r, g, b, alpha]` to an `Rgba8` color.
    #[inline]
    pub const fn from_array3(v: [u8; 4]) -> Self {
        Self::new(v[0], v[1], v[2], v[3])
    }
    /// Converts a four-element array `[r, g, b, alpha]` to an `Rgba8` color.
    #[inline]
    pub const fn from_array4(v: [u8; 4]) -> Self {
        Self::new(v[0], v[1], v[2], v[3])
    }

    /// Returns pure white (`255, 255, 255, 255`).
    #[inline]
    #[must_use]
    pub const fn white() -> Self {
        Self::new(255, 255, 255, 255)
    }
    /// Returns pure black (`0, 0, 0, 255`).
    #[inline]
    #[must_use]
    pub const fn black() -> Self {
        Self::new(0, 0, 0, 255)
    }

    /// Returns the color premultiplied by its alpha.
    #[must_use]
    pub const fn premultiply(self) -> Rgba8pre {
        match self.a {
            255 => Rgba8pre::new(self.r, self.g, self.b, self.a),
            0 => Rgba8pre::new(0, 0, 0, self.a),
            _ => {
                let r = multiply_u8(self.r, self.a);
                let g = multiply_u8(self.g, self.a);
                let b = multiply_u8(self.b, self.a);
                Rgba8pre::new(r, g, b, self.a)
            }
        }
    }

    /// Sets the color to fully transparent black (`0, 0, 0, 0`).
    #[inline] #[rustfmt::skip]
    pub fn clear(&mut self) {
        self.r = 0; self.g = 0; self.b = 0; self.a = 0;
    }

    /// Returns the color components as an array `[r, g, b, a]`.
    #[inline]
    #[must_use]
    pub const fn into_array(&self) -> [u8; 4] {
        self.into_array4()
    }
    /// Returns the color components as an array `[r, g, b, a]`.
    #[inline]
    #[must_use]
    pub const fn into_array3(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
    /// Returns the color components as an array `[r, g, b, a]`.
    #[inline]
    #[must_use]
    pub const fn into_array4(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

/// RGBA color with premultiplied components.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rgba8pre {
    /// Red channel with premultiplied alpha.
    pub r: u8,
    /// Green channel with premultiplied alpha.
    pub g: u8,
    /// Blue channel with premultiplied alpha.
    pub b: u8,
    /// Alpha channel.
    pub a: u8,
}

impl Rgba8pre {
    /// Creates a new premultiplied color from red, green, blue, and alpha values.
    #[inline]
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Converts a `Color` trait type to `Rgba8pre`.
    #[inline]
    #[must_use]
    pub fn from_trait<C: Color>(color: C) -> Self {
        Self { r: color.red8(), g: color.green8(), b: color.blue8(), a: color.alpha8() }
    }

    /// Converts a four-element array `[r, g, b, alpha]` to an `Rgba8pre` color.
    #[inline]
    #[must_use]
    pub const fn from_array(v: [u8; 4]) -> Self {
        Self::from_array4(v)
    }
    /// Converts a three-element array `[r, g, b]` to an `Rgba8pre` color.
    #[inline]
    #[must_use]
    pub const fn from_array3(v: [u8; 3]) -> Self {
        Self::new(v[0], v[1], v[2], u8::MAX)
    }
    /// Converts a four-element array `[r, g, b, alpha]` to an `Rgba8pre` color.
    #[inline]
    #[must_use]
    pub const fn from_array4(v: [u8; 4]) -> Self {
        Self::new(v[0], v[1], v[2], v[3])
    }

    /// Returns the premultiplied color components as an array `[r, g, b, a]`.
    #[inline]
    #[must_use]
    pub const fn into_array(&self) -> [u8; 4] {
        self.into_array4()
    }
    /// Returns the premultiplied color components as an array `[r, g, b]`.
    #[inline]
    #[must_use]
    pub const fn into_array3(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
    /// Returns the premultiplied color components as an array `[r, g, b, a]`.
    #[inline]
    #[must_use]
    pub const fn into_array4(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

/// Standard sRGB color with Red, Green, Blue, and Alpha components.
///
/// See <https://en.wikipedia.org/wiki/SRGB>
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Srgba8 {
    /// Red channel (0-255).
    r: u8,
    /// Green channel (0-255).
    g: u8,
    /// Blue channel (0-255).
    b: u8,
    /// Alpha channel (0-255).
    a: u8,
}

impl Srgba8 {
    /// Creates a new sRGB color.
    #[inline]
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Converts an RGB `Color` trait type to sRGB.
    #[inline]
    #[must_use]
    pub fn from_rgb<C: Color>(c: C) -> Self {
        let r = cu8(rgb_to_srgb(c.red()));
        let g = cu8(rgb_to_srgb(c.green()));
        let b = cu8(rgb_to_srgb(c.blue()));
        Self::new(r, g, b, cu8(c.alpha()))
    }

    /// Converts a four-element array `[r, g, b, alpha]` to an `Rgba8pre` color.
    #[inline]
    #[must_use]
    pub const fn from_array(v: [u8; 4]) -> Self {
        Self::new(v[0], v[1], v[2], v[3])
    }

    /// Returns the premultiplied color components as an array `[r, g, b, a]`.
    #[inline]
    #[must_use]
    pub const fn into_array(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Returns pure white color.
    #[inline]
    #[must_use]
    pub const fn white() -> Self {
        Self::new(255, 255, 255, 255)
    }
    /// Returns pure black color.
    #[inline]
    #[must_use]
    pub const fn black() -> Self {
        Self::new(0, 0, 0, 255)
    }
}

/// RGBA color with `f32` components for higher precision.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Rgba32 {
    /// Red channel (0.0-1.0).
    pub r: f32,
    /// Green channel (0.0-1.0).
    pub g: f32,
    /// Blue channel (0.0-1.0).
    pub b: f32,
    /// Alpha channel (0.0-1.0).
    pub a: f32,
}

impl Rgba32 {
    /// Creates a new `Rgba32` color.
    #[inline]
    #[must_use]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Converts a `Color` trait type to `Rgba32`.
    #[inline]
    #[must_use]
    pub fn from_trait<C: Color>(c: C) -> Self {
        Self::new(c.red() as f32, c.green() as f32, c.blue() as f32, c.alpha() as f32)
    }

    /// Converts a four-element array `[r, g, b, alpha]` to an `Rgba32` color.
    #[inline]
    #[must_use]
    pub const fn from_array(v: [f32; 4]) -> Self {
        Self::new(v[0], v[1], v[2], v[3])
    }

    /// Returns the premultiplied color components as an array `[r, g, b, a]`.
    #[inline]
    #[must_use]
    pub const fn into_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Returns the color premultiplied by its alpha channel.
    // todoconst: abs
    #[must_use]
    pub fn premultiply(&self) -> Self {
        if (self.a - 1.0).abs() <= f32::EPSILON {
            Rgba32::new(self.r, self.g, self.b, self.a)
        } else if self.a == 0.0 {
            Rgba32::new(0., 0., 0., self.a)
        } else {
            let r = self.r * self.a;
            let g = self.g * self.a;
            let b = self.b * self.a;
            Rgba32::new(r, g, b, self.a)
        }
    }
}

#[rustfmt::skip]
mod impl_color {
    use super::*;

    impl Color for Rgba8 {
        #[inline] fn red(&self) -> f64 { color_u8_to_f64(self.r) }
        #[inline] fn green(&self) -> f64 { color_u8_to_f64(self.g) }
        #[inline] fn blue(&self) -> f64 { color_u8_to_f64(self.b) }
        #[inline] fn alpha(&self) -> f64 { color_u8_to_f64(self.a) }
        #[inline] fn alpha8(&self) -> u8 { self.a }
        #[inline] fn red8(&self) -> u8 { self.r }
        #[inline] fn green8(&self) -> u8 { self.g }
        #[inline] fn blue8(&self) -> u8 { self.b }
        #[inline] fn is_premultiplied(&self) -> bool { false }
    }
    impl Color for Rgb8 {
        #[inline] fn red(&self) -> f64 { color_u8_to_f64(self.r) }
        #[inline] fn green(&self) -> f64 { color_u8_to_f64(self.g) }
        #[inline] fn blue(&self) -> f64 { color_u8_to_f64(self.b) }
        #[inline] fn alpha(&self) -> f64 { 1.0 }
        #[inline] fn alpha8(&self) -> u8 { 255 }
        #[inline] fn red8(&self) -> u8 { self.r }
        #[inline] fn green8(&self) -> u8 { self.g }
        #[inline] fn blue8(&self) -> u8 { self.b }
        #[inline] fn is_premultiplied(&self) -> bool { false }
    }
    impl Color for Rgba8pre {
        #[inline] fn red(&self) -> f64 { color_u8_to_f64(self.r) }
        #[inline] fn green(&self) -> f64 { color_u8_to_f64(self.g) }
        #[inline] fn blue(&self) -> f64 { color_u8_to_f64(self.b) }
        #[inline] fn alpha(&self) -> f64 { color_u8_to_f64(self.a) }
        #[inline] fn alpha8(&self) -> u8 { self.a }
        #[inline] fn red8(&self) -> u8 { self.r }
        #[inline] fn green8(&self) -> u8 { self.g }
        #[inline] fn blue8(&self) -> u8 { self.b }
        #[inline] fn is_premultiplied(&self) -> bool { true }
        #[inline] fn is_transparent(&self) -> bool { self.a == 0 }
    }
    impl Color for Srgba8 {
        #[inline] fn red(&self) -> f64 { srgb_to_rgb(color_u8_to_f64(self.r)) }
        #[inline] fn green(&self) -> f64 { srgb_to_rgb(color_u8_to_f64(self.g)) }
        #[inline] fn blue(&self) -> f64 { srgb_to_rgb(color_u8_to_f64(self.b)) }
        #[inline] fn alpha(&self) -> f64 { color_u8_to_f64(self.a) }
        #[inline] fn alpha8(&self) -> u8 { cu8(self.alpha()) }
        #[inline] fn red8(&self) -> u8 { cu8(self.red()) }
        #[inline] fn green8(&self) -> u8 { cu8(self.green()) }
        #[inline] fn blue8(&self) -> u8 { cu8(self.blue()) }
        #[inline] fn is_premultiplied(&self) -> bool { false }
    }
    impl Color for Rgba32 {
        #[inline] fn red(&self) -> f64 { f64::from(self.r) }
        #[inline] fn green(&self) -> f64 { f64::from(self.g) }
        #[inline] fn blue(&self) -> f64 { f64::from(self.b) }
        #[inline] fn alpha(&self) -> f64 { f64::from(self.a) }
        #[inline] fn alpha8(&self) -> u8 { cu8(self.alpha()) }
        #[inline] fn red8(&self) -> u8 { cu8(self.red()) }
        #[inline] fn green8(&self) -> u8 { cu8(self.green()) }
        #[inline] fn blue8(&self) -> u8 { cu8(self.blue()) }
        #[inline] fn is_premultiplied(&self) -> bool { false }
    }
    impl Color for Gray8 {
        #[inline] fn red(&self) -> f64 { color_u8_to_f64(self.value) }
        #[inline] fn green(&self) -> f64 { color_u8_to_f64(self.value) }
        #[inline] fn blue(&self) -> f64 { color_u8_to_f64(self.value) }
        #[inline] fn alpha(&self) -> f64 { color_u8_to_f64(self.alpha) }
        #[inline] fn alpha8(&self) -> u8 { self.alpha }
        #[inline] fn red8(&self) -> u8 { self.value }
        #[inline] fn green8(&self) -> u8 { self.value }
        #[inline] fn blue8(&self) -> u8 { self.value }
        #[inline] fn is_premultiplied(&self) -> bool { false }
    }
}

#[cfg(test)]
mod tests {
    use super::Gray8;
    use super::Rgb8;
    use super::Rgba8;
    use super::Rgba8pre;
    use super::Srgba8;

    #[test]
    fn rgb8_to_gray8_test() {
        let values = [
            [000, 000, 000, 0u8],
            [255, 255, 255, 255],
            [255, 000, 000, 054],
            [000, 255, 000, 182],
            [000, 000, 255, 018],
            [255, 255, 000, 237],
            [255, 000, 255, 073],
            [000, 255, 255, 201],
            [128, 128, 128, 128],
            [128, 000, 000, 027],
            [000, 128, 000, 092],
            [000, 000, 128, 009],
            [128, 128, 000, 119],
            [128, 000, 128, 036],
            [000, 128, 128, 101],
        ];
        for [r, g, b, z] in &values {
            let c = Rgb8::new(*r, *g, *b);
            let gray = Gray8::from_trait(c);
            assert_eq!(gray.value, *z);
        }
    }
    #[test]
    fn rgb8_test() {
        let w = Rgb8::white();
        assert_eq!(w, Rgb8 { r: 255, g: 255, b: 255 });
        let w = Rgb8::black();
        assert_eq!(w, Rgb8 { r: 0, g: 0, b: 0 });
        let w = Rgb8::gray(128);
        assert_eq!(w, Rgb8 { r: 128, g: 128, b: 128 });
        let w = Rgb8::from_slice(&[1, 2, 3]);
        assert_eq!(w, Rgb8 { r: 1, g: 2, b: 3 });
        let w = Rgb8::new(0, 90, 180);
        assert_eq!(w, Rgb8 { r: 0, g: 90, b: 180 });
    }
    #[test]
    fn gray_test() {
        let g = Gray8::new(34);
        assert_eq!(g, Gray8 { value: 34, alpha: 255 });
        let g = Gray8::new_with_alpha(134, 100);
        assert_eq!(g, Gray8 { value: 134, alpha: 100 });
        let g = Gray8::from_slice(&[10, 20]);
        assert_eq!(g, Gray8 { value: 10, alpha: 20 });
    }
    #[test]
    fn rgba8_test() {
        let c = Rgba8::white();
        assert_eq!(c, Rgba8 { r: 255, g: 255, b: 255, a: 255 });
        let c = Rgba8::black();
        assert_eq!(c, Rgba8 { r: 0, g: 0, b: 0, a: 255 });
        let c = Rgba8::new(255, 90, 84, 72);
        assert_eq!(c, Rgba8 { r: 255, g: 90, b: 84, a: 72 });
        let mut c = c;
        c.clear();
        assert_eq!(c, Rgba8 { r: 0, g: 0, b: 0, a: 0 });
        let c = Rgba8::new(255, 255, 255, 128);
        let p = c.premultiply();
        assert_eq!(p, Rgba8pre { r: 128, g: 128, b: 128, a: 128 })
    }
    #[test]
    fn srgb_test() {
        let s = Srgba8::new(50, 150, 250, 128);
        assert_eq!(s, Srgba8 { r: 50, g: 150, b: 250, a: 128 });
        let t = Rgba8::from_trait(s);
        assert_eq!(t, Rgba8 { r: 8, g: 78, b: 244, a: 128 });
    }
}
