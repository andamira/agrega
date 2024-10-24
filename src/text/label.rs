// agrega::font::label

use crate::{_dep::freetype, Pixel, RenderingBase, Rgba8};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

/// Returns the string width using the given `font_face`.
pub fn string_width(txt: &str, font_face: &freetype::Face) -> f64 {
    let mut width = 0.0;
    for c in txt.chars() {
        let glyph_index = font_face.get_char_index(c as usize).unwrap();
        font_face.load_glyph(glyph_index, freetype::face::LoadFlag::DEFAULT).unwrap();
        let glyph = font_face.glyph();
        glyph.render_glyph(freetype::RenderMode::Normal).unwrap();
        let adv = glyph.advance();
        width += adv.x as f64
    }
    width / 64.0
}

/// Returns the line height of the given `font_face`.
pub fn line_height(font_face: &freetype::Face) -> f64 {
    let met = font_face.size_metrics().unwrap();
    (met.ascender - met.descender) as f64 / 64.0
}

/// Horizontal text alignment.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum XAlign {
    Left,
    Center,
    Right,
}
/// Vertical text alignment.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum YAlign {
    Top,
    Center,
    Bottom,
}

/// Used for rendering text, rasterized with `freetype`.
pub struct Label<'a> {
    txt: String,
    x: f64,
    y: f64,
    xa: XAlign,
    ya: YAlign,
    color: Rgba8,
    font: &'a freetype::Face,
    #[allow(dead_code)]
    size: f64,
}

impl<'a> Label<'a> {
    /// Returns a new `Label`.
    #[inline]
    pub fn new(
        txt: &str,
        x: f64,
        y: f64,
        size: f64,
        font: &'a freetype::Face,
    ) -> Result<Self, AggFontError> {
        let resolution = 72;
        font.set_char_size((size * 64.0) as isize, 0, resolution, 0)?;
        Ok(Self {
            txt: txt.to_string(),
            x,
            y,
            xa: XAlign::Left,
            ya: YAlign::Bottom,
            color: Rgba8::black(),
            size,
            font,
        })
    }
    /// Returns the `(width, height)` of the text.
    #[inline]
    pub fn size(&self) -> (f64, f64) {
        let w = string_width(&self.txt, self.font);
        let h = line_height(self.font);
        (w, h)
    }
    /// Sets the `horizontal_alignment` and returns itself.
    #[inline]
    pub const fn xalign(mut self, horizontal_alignment: XAlign) -> Self {
        self.xa = horizontal_alignment;
        self
    }
    /// Sets the `vertical_alignment` and returns itself.
    #[inline]
    pub const fn yalign(mut self, vertical_alignment: YAlign) -> Self {
        self.ya = vertical_alignment;
        self
    }
    /// Sets the `color` and returns itself.
    #[inline]
    pub const fn color(mut self, color: Rgba8) -> Self {
        self.color = color;
        self
    }

    /// Draws the text on the given `rendering_base`.
    ///
    /// It rounds the floating-point coordinates towards 0.
    #[inline]
    pub fn draw<T: Pixel>(&mut self, rendering_base: &mut RenderingBase<T>) {
        draw_text(&self.txt, self.x as i64, self.y as i64, self.color, self.font, rendering_base);
    }

    /// Draws the text on the given `rendering_base`, with subpixel positioning.
    #[inline]
    pub fn draw_subpixel<T: Pixel>(&mut self, rendering_base: &mut RenderingBase<T>) {
        draw_text_subpixel(
            &self.txt,
            self.x,
            self.y,
            self.xa,
            self.ya,
            self.color,
            self.font,
            rendering_base,
        );
    }
}

/// Draws text.
pub fn draw_text<T: Pixel>(
    txt: &str,
    x: i64,
    y: i64,
    color: Rgba8,
    font: &freetype::Face,
    ren_base: &mut RenderingBase<T>,
) {
    let (mut x, mut y) = (x, y);
    let width = string_width(txt, font);
    let height = line_height(font);
    // Shift to center justification, x and y
    let dx = (width / 2.0).round() as i64;
    let dy = (height / 2.0).round() as i64;
    x -= dx;
    y += dy;
    for c in txt.chars() {
        let glyph_index = font.get_char_index(c as usize).unwrap();
        font.load_glyph(glyph_index, freetype::face::LoadFlag::DEFAULT).unwrap();
        font.glyph().render_glyph(freetype::RenderMode::Normal).unwrap();
        let g = font.glyph().bitmap();
        let left = font.glyph().bitmap_left() as i64;
        let top = font.glyph().bitmap_top() as i64;
        let buf: Vec<_> = g.buffer().iter().map(|&x| x as u64).collect();
        let rows = g.rows() as i64;
        let pitch = g.pitch().unsigned_abs() as usize;
        let width = g.width() as i64;
        for i in 0..rows {
            ren_base.blend_solid_hspan(
                x + left,
                y - top + i,
                width,
                color,
                &buf[pitch * i as usize..],
            );
        }
        let adv = font.glyph().advance();
        x += (adv.x as f64 / 64.0).round() as i64;
        y += (adv.y as f64 / 64.0).round() as i64;
    }
}

/// Draws text with subpixel-positioning.
///
/// - [Freetype Subpixel positioning](https://freetype.org/freetype2/docs/glyphs/glyphs-5.html#section-2)
#[allow(clippy::too_many_arguments)]
pub fn draw_text_subpixel<T: Pixel>(
    txt: &str,
    x: f64,
    y: f64,
    xalign: XAlign,
    yalign: YAlign,
    color: Rgba8,
    font: &freetype::Face,
    ren_base: &mut RenderingBase<T>,
) {
    let (mut x, mut y) = (x, y);
    let width = string_width(txt, font);

    let asc = font.size_metrics().unwrap().ascender as f64 / 64.0;
    x += match xalign {
        XAlign::Left => 0.0,
        XAlign::Right => -width,
        XAlign::Center => -width / 2.0,
    };
    y += match yalign {
        YAlign::Top => asc,
        YAlign::Bottom => 0.0,
        YAlign::Center => asc / 2.0,
    };

    for c in txt.chars() {
        let glyph_index = font.get_char_index(c as usize).unwrap();
        font.load_glyph(glyph_index, freetype::face::LoadFlag::DEFAULT).unwrap();

        let glyph = font.glyph().get_glyph().unwrap();
        let dt = freetype::Vector {
            x: ((x - x.floor()) * 64.0).round() as i64,
            y: ((y - y.floor()) * 64.0).round() as i64,
        };
        glyph.transform(None, Some(dt)).unwrap();
        let g = glyph.to_bitmap(freetype::RenderMode::Normal, None).unwrap();
        let left = g.left() as i64;
        let top = g.top() as i64;
        let bit = g.bitmap();
        let buf: Vec<_> = bit.buffer().iter().map(|&x| x as u64).collect();
        let rows = bit.rows() as i64;
        let width = bit.width() as i64;
        let pitch = bit.pitch().unsigned_abs() as usize;
        for i in 0..rows {
            ren_base.blend_solid_hspan(
                x.floor() as i64 + left,
                y.floor() as i64 + i - top,
                width,
                color,
                &buf[pitch * i as usize..],
            );
        }

        x += glyph.advance_x() as f64 / 65536.0;
        y += glyph.advance_y() as f64 / 65536.0;
    }
}

/// A font error.
#[derive(Debug)]
pub enum AggFontError {
    /// Freetype Error
    Ft(freetype::error::Error),
    Io(String),
}
impl From<freetype::error::Error> for AggFontError {
    fn from(err: freetype::error::Error) -> Self {
        AggFontError::Ft(err)
    }
}
impl From<String> for AggFontError {
    fn from(err: String) -> Self {
        AggFontError::Io(err)
    }
}

// FIXME
// pub fn font(_name: &str) -> Result<freetype::Face, AggFontError> {
//     //let prop = font_loader::system_fonts::FontPropertyBuilder::new().family(name).build();
//     //let (font, _) = font_loader::system_fonts::get(&prop).ok_or("error loading font".to_string())?;
//     //let lib = freetype::Library::init()?;
//     //let face = lib.new_memory_face(font, 0)?;
//     //Ok(face)
//     Err(AggFontError::Io("??".to_string()))
// }
