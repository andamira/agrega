// agrega::scanline::rend
//
//! Renderer

use crate::{
    len_i64_xy, ClipSide, Color, DistanceInterpolator4, DrawOutline, LineInterpolator,
    LineParameters, Pixel, PixelSource, Pixfmt, RasterizerScanline, Rectangle, RenderingBase,
    Rgba8, ScanlineU8, SpanGradient, Subpixel, VertexSource, MAX_HALF_WIDTH, POLY_SUBPIXEL_MASK,
    POLY_SUBPIXEL_SCALE, POLY_SUBPIXEL_SHIFT,
};
use alloc::{vec, vec::Vec};
use devela::iif;
#[allow(unused_imports)]
use devela::ExtFloat;

pub(crate) const LINE_MAX_LENGTH: i64 = 1 << (POLY_SUBPIXEL_SHIFT + 10);

/* pub trait Scale<T> {
    fn upscale(v: f64)   -> T;
    fn downscale(v: i64) -> T;
}*/

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

/// Aliased Renderer
#[derive(Debug)]
pub struct RenderingScanlineBinSolid<'a, T: 'a> {
    pub base: &'a mut RenderingBase<T>,
    pub color: Rgba8,
}
/// Anti-Aliased Renderer
#[derive(Debug)]
pub struct RenderingScanlineAASolid<'a, T: 'a> {
    base: &'a mut RenderingBase<T>,
    color: Rgba8,
}

#[derive(Debug)]
pub struct RenderingScanlineAA<'a, T> {
    base: &'a mut RenderingBase<T>,
    span: SpanGradient,
}

/// Render a single Scanline (y-row) without Anti-Aliasing (Binary?)
fn render_scanline_bin_solid<T: Pixel, C: Color>(
    sl: &ScanlineU8,
    ren: &mut RenderingBase<T>,
    color: C,
) {
    let cover_full = 255;
    for span in &sl.spans {
        ren.blend_hline(span.x, sl.y, span.x - 1 + span.len.abs(), color, cover_full);
    }
}

/// Render a single Scanline (y-row) with Anti Aliasing
fn render_scanline_aa_solid<T: Pixel, C: Color>(
    sl: &ScanlineU8,
    ren: &mut RenderingBase<T>,
    color: C,
) {
    let y = sl.y;
    for span in &sl.spans {
        let x = span.x;
        if span.len > 0 {
            ren.blend_solid_hspan(x, y, span.len, color, &span.covers);
        } else {
            ren.blend_hline(x, y, x - span.len - 1, color, span.covers[0]);
        }
    }
}

/// Render a single Scanline (y-row) with Anti-Aliasing
fn render_scanline_aa<T: Pixel>(
    sl: &ScanlineU8,
    ren: &mut RenderingBase<T>,
    span_gen: &SpanGradient,
) {
    let y = sl.y;
    for span in &sl.spans {
        let x = span.x;
        let mut len = span.len;
        let covers = &span.covers;
        if len < 0 {
            len = -len;
        }
        //dbg!(x);
        //dbg!(y);
        //dbg!(len);
        let colors = span_gen.generate(x, y, len as usize);
        //dbg!(&colors);
        ren.blend_color_hspan(
            x,
            y,
            len,
            &colors,
            if span.len < 0 { &[] } else { covers },
            covers[0],
        );
    }
}

/// TODO
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RenderData {
    sl: ScanlineU8,
}
impl RenderData {
    /// TODO
    #[inline]
    pub fn new() -> Self {
        Self { sl: ScanlineU8::new() }
    }
}
impl Default for RenderData {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Pixel> Render for RenderingScanlineAASolid<'_, T> {
    /// Render a single Scanline Row
    #[inline]
    fn render(&mut self, data: &RenderData) {
        render_scanline_aa_solid(&data.sl, self.base, self.color);
    }
    /// Set the current color
    #[inline]
    fn color<C: Color>(&mut self, color: C) {
        self.color = Rgba8::new(color.red8(), color.green8(), color.blue8(), color.alpha8());
    }
}
impl<T: Pixel> Render for RenderingScanlineBinSolid<'_, T> {
    /// Render a single Scanline Row
    #[inline]
    fn render(&mut self, data: &RenderData) {
        render_scanline_bin_solid(&data.sl, self.base, self.color);
    }
    /// Set the current Color
    #[inline]
    fn color<C: Color>(&mut self, color: C) {
        self.color = Rgba8::new(color.red8(), color.green8(), color.blue8(), color.alpha8());
    }
}
impl<T: Pixel> Render for RenderingScanlineAA<'_, T> {
    /// Render a single Scanline Row
    #[inline]
    fn render(&mut self, data: &RenderData) {
        render_scanline_aa(&data.sl, self.base, &self.span);
    }
    /// Set the current Color
    #[inline]
    fn color<C: Color>(&mut self, _color: C) {
        unimplemented!("oops");
    }
}

impl<'a, T: Pixel> RenderingScanlineBinSolid<'a, T> {
    /// Create a new Renderer from a Rendering Base
    #[inline]
    pub fn with_base(base: &'a mut RenderingBase<T>) -> Self {
        let color = Rgba8::black();
        Self { base, color }
    }
    /// TODO
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.base.as_bytes()
    }
    /// TODO
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub fn to_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), image::ImageError> {
        self.base.to_file(filename)
    }
}
impl<'a, T: Pixel> RenderingScanlineAA<'a, T> {
    /// TODO
    #[inline]
    pub fn new(base: &'a mut RenderingBase<T>, span: SpanGradient) -> Self {
        Self { base, span }
    }
}
impl<'a, T: Pixel> RenderingScanlineAASolid<'a, T> {
    /// Create a new Renderer from a Rendering Base
    #[inline]
    pub fn with_base(base: &'a mut RenderingBase<T>) -> Self {
        let color = Rgba8::black();
        Self { base, color }
    }
    /// TODO
    #[inline]
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.base.as_bytes()
    }
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub fn to_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), image::ImageError> {
        self.base.to_file(filename)
    }
}

/// Render rasterized data to an image using a single color, Binary
pub fn render_scanlines_bin_solid<C: Color, T: Pixel>(
    ras: &mut RasterizerScanline,
    ren: &mut RenderingBase<T>,
    color: C,
) {
    let mut sl = ScanlineU8::new();
    if ras.rewind_scanlines() {
        sl.reset(ras.min_x(), ras.max_x());
        while ras.sweep_scanline(&mut sl) {
            render_scanline_bin_solid(&sl, ren, color);
        }
    }
}

/// Render rasterized data to an image using a single color, Anti-aliased
pub fn render_scanlines_aa_solid<C: Color, T: Pixel>(
    ras: &mut RasterizerScanline,
    ren: &mut RenderingBase<T>,
    color: C,
) {
    let mut sl = ScanlineU8::new();
    if ras.rewind_scanlines() {
        sl.reset(ras.min_x(), ras.max_x());
        while ras.sweep_scanline(&mut sl) {
            render_scanline_aa_solid(&sl, ren, color);
        }
    }
}

/// Render rasterized data to an image using the current color
pub fn render_scanlines<REN>(ras: &mut RasterizerScanline, ren: &mut REN)
where
    REN: Render,
{
    let mut data = RenderData::new();
    if ras.rewind_scanlines() {
        data.sl.reset(ras.min_x(), ras.max_x());
        ren.prepare();
        while ras.sweep_scanline(&mut data.sl) {
            ren.render(&data);
        }
    }
}

/// Render paths after rasterizing to an image using a set of colors
pub fn render_all_paths<REN, VS, C>(
    ras: &mut RasterizerScanline,
    ren: &mut REN,
    paths: &[VS],
    colors: &[C],
) where
    C: Color,
    REN: Render,
    VS: VertexSource,
{
    debug_assert!(paths.len() == colors.len());
    for (path, color) in paths.iter().zip(colors.iter()) {
        ras.reset();
        ras.add_path(path);
        ren.color(*color);
        render_scanlines(ras, ren);
    }
}

pub(crate) struct BresehamInterpolator {
    /// First point, x position
    pub x1: i64,
    /// First point, y position
    pub y1: i64,
    /// Second point, x position
    pub x2: i64,
    /// Second point, y position
    pub y2: i64,
    /// Line is primarilly vertical
    pub ver: bool,
    pub len: i64,
    inc: i64,
    func: LineInterpolator,
}

impl BresehamInterpolator {
    pub fn new(x1_hr: Subpixel, y1_hr: Subpixel, x2_hr: Subpixel, y2_hr: Subpixel) -> Self {
        let x1 = i64::from(x1_hr);
        let x2 = i64::from(x2_hr);
        let y1 = i64::from(y1_hr);
        let y2 = i64::from(y2_hr);
        let dy = (y2 - y1).abs();
        let dx = (x2 - x1).abs();
        let ver = dy > dx;
        let len = if ver { dy } else { dx };
        let inc = if ver {
            if y2 > y1 {
                1
            } else {
                -1
            }
        } else if x2 > x1 {
            1
        } else {
            -1
        };
        let (z1, z2) = if ver { (x1_hr, x2_hr) } else { (y1_hr, y2_hr) };
        // XXX  - value() should not be used
        let func = LineInterpolator::new(z1.value(), z2.value(), len);
        let y2 = func.y >> POLY_SUBPIXEL_SHIFT;
        let x2 = func.y >> POLY_SUBPIXEL_SHIFT;
        Self { x1, y1, x2, y2, ver, len, inc, func }
    }
    pub fn vstep(&mut self) {
        self.func.inc();
        self.y1 += self.inc;
        self.x2 = self.func.y >> POLY_SUBPIXEL_SHIFT;
    }
    pub fn hstep(&mut self) {
        self.func.inc();
        self.x1 += self.inc;
        self.y2 = self.func.y >> POLY_SUBPIXEL_SHIFT;
    }
}

/// Clip a Line segment to a Rectangle
///
/// # Arguments
///   - x1 - Starting x point of line
///   - y1 - Starting y point of line
///   - x2 - Ending x point of line
///   - y1 - Ending y point of line
///   - clip_box - Rectangle to clip line to
///
/// # Return
///   - Clipped line segment (x1,y1,x2,y2) and flag descringing the clip
///   - flag
///     - 0 - Not Clipped, line is fully within the clip box
///     - 4 - Line fully outside of the clip box
///     - 1 - First Point Clipped
///     - 2 - Second Point Clipped
///     - 3 - Both Points Clipped
///
pub(crate) fn clip_line_segment(
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    clip_box: Rectangle<i64>,
) -> (i64, i64, i64, i64, u8) {
    let f1 = clip_box.clip_flags_i64(x1, y1);
    let f2 = clip_box.clip_flags_i64(x2, y2);
    let mut ret = 0;
    if f1 == ClipSide::INSIDE && f2 == ClipSide::INSIDE {
        return (x1, y1, x2, y2, 0);
    }
    let x_side = ClipSide::LEFT | ClipSide::RIGHT;
    let y_side = ClipSide::TOP | ClipSide::BOTTOM;
    if f1 & x_side != 0 && f1 & x_side == f2 & x_side {
        return (x1, y1, x2, y2, 4); // Outside
    }
    if f1 & y_side != 0 && f1 & y_side == f2 & y_side {
        return (x1, y1, x2, y2, 4); // Outside
    }
    let (mut x1, mut y1) = (x1, y1);
    let (mut x2, mut y2) = (x2, y2);
    if f1 != 0 {
        if let Some((x, y)) = clip_move_point(x1, y1, x2, y2, clip_box, x1, y1, f1) {
            x1 = x;
            y1 = y;
        } else {
            return (x1, y1, x2, y2, 4);
        }
        if x1 == x2 && y1 == y2 {
            return (x1, y1, x2, y2, 4);
        }
        ret |= 1;
    }
    if f2 != 0 {
        if let Some((x, y)) = clip_move_point(x1, y1, x2, y2, clip_box, x2, y2, f2) {
            x2 = x;
            y2 = y;
        } else {
            return (x1, y1, x2, y2, 4);
        }
        if x1 == x2 && y1 == y2 {
            return (x1, y1, x2, y2, 4);
        }
        ret |= 2;
    }
    (x1, y1, x2, y2, ret)
}

#[allow(clippy::too_many_arguments)]
fn clip_move_point(
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    clip_box: Rectangle<i64>,
    x: i64,
    y: i64,
    flags: u8,
) -> Option<(i64, i64)> {
    let (mut x, mut y) = (x, y);
    if flags & (ClipSide::LEFT | ClipSide::RIGHT) != 0 {
        if x1 == x2 {
            return None;
        } else {
            let x = if flags & ClipSide::LEFT != 0 {
                clip_box.x1()
            } else {
                clip_box.x2()
            };
            y = ((x - x1) as f64 * (y2 - y1) as f64 / (x2 - x1) as f64 + y1 as f64) as i64;
        }
    }
    let flags = clip_box.clip_flags_i64(x, y);
    if flags & (ClipSide::TOP | ClipSide::BOTTOM) != 0 {
        if y1 == y2 {
            return None;
        } else {
            let y = if flags & ClipSide::BOTTOM != 0 {
                clip_box.y1()
            } else {
                clip_box.y2()
            };
            x = ((y - y1) as f64 * (x2 - x1) as f64 / (y2 - y1) as f64 + x1 as f64) as i64;
        }
    }
    Some((x, y))
}

/// TODO
#[derive(Debug)]
pub struct RendererOutlineImg<'a, T> {
    ren: &'a mut RenderingBase<T>,
    pattern: LineImagePatternPow2,
    start: i64,
    scale_x: f64,
    clip_box: Option<Rectangle<i64>>,
}
impl<T: Pixel> DrawOutline for RendererOutlineImg<'_, T> {
    fn accurate_join_only(&self) -> bool {
        true
    }

    fn color<C: Color>(&mut self, _color: C) {
        unimplemented!("no color for outline img");
    }

    fn line0(&mut self, _lp: &LineParameters) {}
    fn line1(&mut self, _lp: &LineParameters, _sx: i64, _sy: i64) {}
    fn line2(&mut self, _lp: &LineParameters, _ex: i64, _ey: i64) {}
    fn line3(&mut self, lp: &LineParameters, sx: i64, sy: i64, ex: i64, ey: i64) {
        if let Some(clip_box) = self.clip_box {
            let x1 = lp.x1;
            let y1 = lp.y1;
            let x2 = lp.x2;
            let y2 = lp.y2;
            let (x1, y1, x2, y2, flags) = clip_line_segment(x1, y1, x2, y2, clip_box);
            let start = self.start;
            let (mut sx, mut sy, mut ex, mut ey) = (sx, sy, ex, ey);
            if (flags & 4) == 0 {
                if flags != 0 {
                    let lp2 = LineParameters::new(x1, y1, x2, y2, len_i64_xy(x1, y1, x2, y2));
                    if flags & 1 != 0 {
                        self.start +=
                            (len_i64_xy(lp.x1, lp.y1, x1, y1) as f64 / self.scale_x).round() as i64;
                        sx = x1 + (y2 - y1);
                        sy = y1 - (x2 - x1);
                    } else {
                        while (sx - lp.x1).abs() + (sy - lp.y1).abs() > lp2.len {
                            sx = (lp.x1 + sx) >> 1;
                            sy = (lp.y1 + sy) >> 1;
                        }
                    }
                    if flags & 2 != 0 {
                        ex = x2 + (y2 - y1);
                        ey = y2 - (x2 - x1);
                    } else {
                        while (ex - lp.x2).abs() + (ey - lp.y2).abs() > lp2.len {
                            ex = (lp.x2 + ex) >> 1;
                            ey = (lp.y2 + ey) >> 1;
                        }
                    }
                    self.line3_no_clip(&lp2, sx, sy, ex, ey);
                } else {
                    self.line3_no_clip(lp, sx, sy, ex, ey);
                }
            }
            self.start = start + (lp.len as f64 / self.scale_x).round() as i64;
        } else {
            self.line3_no_clip(lp, sx, sy, ex, ey);
        }
    }
    fn semidot<F: Fn(i64) -> bool>(&mut self, _cmp: F, _xc1: i64, _yc1: i64, _xc2: i64, _yc2: i64) {
    }
    fn pie(&mut self, _xc: i64, _y: i64, _x1: i64, _y1: i64, _x2: i64, _y2: i64) {}
}

impl<'a, T: Pixel> RendererOutlineImg<'a, T> {
    pub fn with_base_and_pattern(
        ren: &'a mut RenderingBase<T>,
        pattern: LineImagePatternPow2,
    ) -> Self {
        Self { ren, pattern, start: 0, scale_x: 1.0, clip_box: None }
    }
    pub fn scale_x(&mut self, scale_x: f64) {
        self.scale_x = scale_x;
    }
    pub fn start_x(&mut self, s: f64) {
        self.start = (s * POLY_SUBPIXEL_SCALE as f64).round() as i64;
    }
    fn subpixel_width(&self) -> i64 {
        self.pattern.line_width()
    }
    fn pattern_width(&self) -> i64 {
        self.pattern.pattern_width()
    }
    // fn width(&self) -> f64 {
    //     self.subpixel_width() as f64 / POLY_SUBPIXEL_SCALE as f64
    // }
    fn pixel(&mut self, x: i64, y: i64) -> Rgba8 {
        self.pattern.pixel(x, y)
    }
    fn blend_color_hspan(&mut self, x: i64, y: i64, len: i64, colors: &[Rgba8]) {
        self.ren.blend_color_hspan(x, y, len, colors, &[], 255);
    }
    fn blend_color_vspan(&mut self, x: i64, y: i64, len: i64, colors: &[Rgba8]) {
        self.ren.blend_color_vspan(x, y, len, colors, &[], 255);
    }
    fn line3_no_clip(&mut self, lp: &LineParameters, sx: i64, sy: i64, ex: i64, ey: i64) {
        if lp.len > LINE_MAX_LENGTH {
            let (lp1, lp2) = lp.divide();
            let mx = lp1.x2 + (lp1.y2 - lp1.y1);
            let my = lp1.y2 - (lp1.x2 - lp1.x1);
            self.line3_no_clip(&lp1, (lp.x1 + sx) >> 1, (lp.y1 + sy) >> 1, mx, my);
            self.line3_no_clip(&lp2, mx, my, (lp.x2 + ex) >> 1, (lp.y2 + ey) >> 1);
            return;
        }
        let (sx, sy) = lp.fix_degenerate_bisectrix_start(sx, sy);
        let (ex, ey) = lp.fix_degenerate_bisectrix_end(ex, ey);
        let mut li = lp.interp_image(
            sx,
            sy,
            ex,
            ey,
            self.subpixel_width(),
            self.start,
            self.pattern_width(),
            self.scale_x,
        );
        if li.vertical() {
            while li.step_ver(self) {}
        } else {
            while li.step_hor(self) {}
        }
        self.start += (lp.len as f64 / self.scale_x).round() as i64;
    }
}

#[derive(Debug)]
pub struct LineImagePattern {
    pix: Pixfmt<Rgba8>,
    filter: PatternFilterBilinear,
    dilation: u64,
    dilation_hr: i64,
    //data: Vec<u8>,
    width: u64,
    height: u64,
    width_hr: i64,
    half_height_hr: i64,
    offset_y_hr: i64,
}

impl LineImagePattern {
    pub fn new(filter: PatternFilterBilinear) -> Self {
        let dilation = filter.dilation() + 1;
        let dilation_hr = (dilation as i64) << POLY_SUBPIXEL_SHIFT;
        Self {
            filter,
            dilation,
            dilation_hr,
            width: 0,
            height: 0,
            width_hr: 0,
            half_height_hr: 0,
            offset_y_hr: 0,
            pix: Pixfmt::new(1, 1),
        }
    }
    pub fn create<T: PixelSource + Pixel>(&mut self, src: &T) {
        self.height = src.height() as u64;
        self.width = src.width() as u64;
        self.width_hr = src.width() as i64 * POLY_SUBPIXEL_SCALE;
        self.half_height_hr = src.height() as i64 * POLY_SUBPIXEL_SCALE / 2;
        self.offset_y_hr = self.dilation_hr + self.half_height_hr - POLY_SUBPIXEL_SCALE / 2;
        self.half_height_hr += POLY_SUBPIXEL_SCALE / 2;

        self.pix = Pixfmt::<Rgba8>::new(
            (self.width + self.dilation * 2) as usize,
            (self.height + self.dilation * 2) as usize,
        );
        for y in 0..self.height as usize {
            let x1 = self.dilation as usize;
            let y1 = y + self.dilation as usize;
            for x in 0..self.width as usize {
                self.pix.set((x1 + x, y1), src.get((x, y)));
            }
        }
        //const color_type* s1;
        //const color_type* s2;
        let none = Rgba8::new(0, 0, 0, 0);
        let dill = self.dilation as usize;
        for y in 0..dill {
            //s1 = self.buf.row_ptr(self.height + self.dilation - 1) + self.dilation;
            //s2 = self.buf.row_ptr(self.dilation) + self.dilation;
            //let d1 = self.buf.row_ptr(self.dilation + self.height + y) + self.dilation;
            //let d2 = self.buf.row_ptr(self.dilation - y - 1) + self.dilation;
            let (x1, y1) = (dill, dill + y + self.height as usize);
            let (x2, y2) = (dill, dill - y - 1);
            for x in 0..self.width as usize {
                //*d1++ = color_type(*s1++, 0);
                //*d2++ = color_type(*s2++, 0);
                //*d1++ = color_type::no_color();
                //*d2++ = color_type::no_color();
                self.pix.set((x1 + x, y1), none);
                self.pix.set((x2 + x, y2), none);
            }
        }
        let h = self.height + self.dilation * 2;
        for y in 0..h as usize {
            let sx1 = self.dilation as usize;
            let sx2 = (self.dilation + self.width) as usize;
            let dx1 = sx2;
            let dx2 = sx1;
            //s1 = self.buf.row_ptr(y) + self.dilation;
            //s2 = self.buf.row_ptr(y) + self.dilation + self.width;
            //d1 = self.buf.row_ptr(y) + self.dilation + self.width;
            //d2 = self.buf.row_ptr(y) + self.dilation;

            for x in 0..self.dilation as usize {
                //*d1++ = *s1++;
                //*--d2 = *--s2;
                self.pix.set((dx1 + x, y), self.pix.get((sx1 + x, y)));
                self.pix.set((dx2 - x - 1, y), self.pix.get((sx2 - x - 1, y)));
            }
        }
    }
    pub fn pattern_width(&self) -> i64 {
        self.width_hr
    }
    pub fn line_width(&self) -> i64 {
        self.half_height_hr
    }
    #[allow(clippy::misnamed_getters)] // CHECK
    pub fn width(&self) -> u64 {
        self.height
    }
}

#[derive(Debug)]
pub struct LineImagePatternPow2 {
    base: LineImagePattern,
    mask: u64,
}

impl LineImagePatternPow2 {
    pub fn new(filter: PatternFilterBilinear) -> Self {
        let base = LineImagePattern::new(filter);
        Self { base, mask: POLY_SUBPIXEL_MASK as u64 }
    }
    pub fn create<T: PixelSource + Pixel>(&mut self, src: &T) {
        self.base.create(src);
        self.mask = 1;
        while self.mask < self.base.width {
            self.mask <<= 1;
            self.mask |= 1;
        }
        self.mask <<= POLY_SUBPIXEL_SHIFT - 1;
        self.mask |= POLY_SUBPIXEL_MASK as u64;
        self.base.width_hr = (self.mask + 1) as i64;
    }
    pub fn pattern_width(&self) -> i64 {
        self.base.width_hr
    }
    pub fn line_width(&self) -> i64 {
        self.base.half_height_hr
    }
    #[allow(clippy::misnamed_getters)] // CHECK
    pub fn width(&self) -> u64 {
        self.base.height
    }
    pub fn pixel(&self, x: i64, y: i64) -> Rgba8 {
        self.base.filter.pixel_high_res(
            &self.base.pix,
            (x & self.mask as i64) + self.base.dilation_hr,
            y + self.base.offset_y_hr,
        )
    }
}

#[derive(Debug, Default)]
pub struct PatternFilterBilinear();

impl PatternFilterBilinear {
    pub fn new() -> Self {
        Self {}
    }
    pub fn dilation(&self) -> u64 {
        1
    }
    pub fn pixel_low_res(&self, pix: &Pixfmt<Rgba8>, x: i64, y: i64) -> Rgba8 {
        pix.get((x as usize, y as usize))
    }
    pub fn pixel_high_res(&self, pix: &Pixfmt<Rgba8>, x: i64, y: i64) -> Rgba8 {
        let (mut red, mut green, mut blue, mut alpha) = (0i64, 0i64, 0i64, 0i64);

        let x_lr = (x as usize) >> POLY_SUBPIXEL_SHIFT;
        let y_lr = (y as usize) >> POLY_SUBPIXEL_SHIFT;

        let x = x & POLY_SUBPIXEL_MASK;
        let y = y & POLY_SUBPIXEL_MASK;

        let ptr = pix.get((x_lr, y_lr));

        let weight = (POLY_SUBPIXEL_SCALE - x) * (POLY_SUBPIXEL_SCALE - y);
        red += weight * i64::from(ptr.r);
        green += weight * i64::from(ptr.g);
        blue += weight * i64::from(ptr.b);
        alpha += weight * i64::from(ptr.a);
        let ptr = pix.get((x_lr + 1, y_lr));
        let weight = x * (POLY_SUBPIXEL_SCALE - y);
        red += weight * i64::from(ptr.r);
        green += weight * i64::from(ptr.g);
        blue += weight * i64::from(ptr.b);
        alpha += weight * i64::from(ptr.a);
        let ptr = pix.get((x_lr, y_lr + 1));
        let weight = (POLY_SUBPIXEL_SCALE - x) * y;
        red += weight * i64::from(ptr.r);
        green += weight * i64::from(ptr.g);
        blue += weight * i64::from(ptr.b);
        alpha += weight * i64::from(ptr.a);
        let ptr = pix.get((x_lr + 1, y_lr + 1));
        let weight = x * y;
        red += weight * i64::from(ptr.r);
        green += weight * i64::from(ptr.g);
        blue += weight * i64::from(ptr.b);
        alpha += weight * i64::from(ptr.a);
        let red = (red >> (POLY_SUBPIXEL_SHIFT * 2)) as u8;
        let green = (green >> (POLY_SUBPIXEL_SHIFT * 2)) as u8;
        let blue = (blue >> (POLY_SUBPIXEL_SHIFT * 2)) as u8;
        let alpha = (alpha >> (POLY_SUBPIXEL_SHIFT * 2)) as u8;
        Rgba8::new(red, green, blue, alpha)
    }
}

/// TODO
#[derive(Clone, Debug)]
pub struct LineInterpolatorImage {
    lp: LineParameters,
    li: LineInterpolator,
    di: DistanceInterpolator4,
    //pub plen: i64,
    x: i64,
    y: i64,
    old_x: i64,
    old_y: i64,
    count: i64,
    width: i64,
    #[allow(dead_code)]
    max_extent: i64,
    start: i64,
    step: i64,
    //pub dist_pos: [i64; MAX_HALF_WIDTH + 1],
    dist_pos: Vec<i64>,
    //pub colors: [Rgba8; MAX_HALF_WIDTH * 2 + 4],
    colors: Vec<Rgba8>,
}

impl LineInterpolatorImage {
    #[allow(clippy::too_many_arguments)] #[rustfmt::skip]
    pub fn new(
        lp: LineParameters, sx: i64, sy: i64, ex: i64, ey: i64,
        subpixel_width: i64, pattern_start: i64, pattern_width: i64, scale_x: f64,
    ) -> Self {
        let n = iif![lp.vertical; (lp.y2 - lp.y1).abs(); (lp.x2 - lp.x1).abs() + 1];
        let y1 = if lp.vertical {
            (lp.x2 - lp.x1) << POLY_SUBPIXEL_SHIFT
        } else {
            (lp.y2 - lp.y1) << POLY_SUBPIXEL_SHIFT
        };
        let mut m_li = LineInterpolator::new_back_adjusted_2(y1, n);
        let mut x = lp.x1 >> POLY_SUBPIXEL_SHIFT;
        let mut y = lp.y1 >> POLY_SUBPIXEL_SHIFT;
        let (mut old_x, mut old_y) = (x, y);
        let count = if lp.vertical {
            ((lp.y2 >> POLY_SUBPIXEL_SHIFT) - y).abs()
        } else {
            ((lp.x2 >> POLY_SUBPIXEL_SHIFT) - x).abs()
        };
        let width = subpixel_width;
        let max_extent = (width + POLY_SUBPIXEL_SCALE) >> POLY_SUBPIXEL_SHIFT;
        let mut step = 0;
        let start = pattern_start + (max_extent + 2) * pattern_width;
        let mut dist_pos = vec![0i64; MAX_HALF_WIDTH + 1];
        let colors = vec![Rgba8::black(); MAX_HALF_WIDTH * 2 + 4];
        let mut di = DistanceInterpolator4::new(
            lp.x1, lp.y1, lp.x2, lp.y2, sx, sy, ex, ey, lp.len, scale_x,
            lp.x1 & !POLY_SUBPIXEL_MASK, lp.y1 & !POLY_SUBPIXEL_MASK,
        );
        let dd = if lp.vertical {
            lp.dy << POLY_SUBPIXEL_SHIFT
        } else {
            lp.dx << POLY_SUBPIXEL_SHIFT
        };
        let mut li = LineInterpolator::new(0, dd, lp.len);

        let stop = width + POLY_SUBPIXEL_SCALE * 2;
        for item in dist_pos.iter_mut().take(MAX_HALF_WIDTH) {
            *item = li.y;
            iif![*item >= stop; break];
            li.inc();
        }
        dist_pos[MAX_HALF_WIDTH] = 0x7FFF_0000;

        let mut npix = 1;
        if lp.vertical {
            loop {
                m_li.dec();
                y -= lp.inc;
                x = (lp.x1 + m_li.y) >> POLY_SUBPIXEL_SHIFT;

                iif![lp.inc > 0; di.dec_y_by(x - old_x); di.inc_y_by(x - old_x)];

                old_x = x;

                let mut dist1_start = di.dist_start;
                let mut dist2_start = di.dist_start;

                let mut dx = 0;
                iif![dist1_start < 0; npix += 1];
                loop {
                    dist1_start += di.dy_start;
                    dist2_start -= di.dy_start;
                    iif![dist1_start < 0; npix += 1];
                    iif![dist2_start < 0; npix += 1];
                    dx += 1;
                    iif![dist_pos[dx] > width; break];
                }
                iif![npix == 0; break];
                npix = 0;
                step -= 1;
                iif![step < -max_extent; break];
            }
        } else {
            loop {
                m_li.dec();

                x -= lp.inc;
                y = (lp.y1 + m_li.y) >> POLY_SUBPIXEL_SHIFT;

                iif![lp.inc > 0; di.dec_x_by(y - old_y); di.inc_x_by(y - old_y)];
                old_y = y;

                let mut dist1_start = di.dist_start;
                let mut dist2_start = di.dist_start;

                let mut dy = 0;
                iif![dist1_start < 0; npix += 1];
                loop {
                    dist1_start -= di.dx_start;
                    dist2_start += di.dx_start;
                    iif![dist1_start < 0; npix += 1];
                    iif![dist2_start < 0; npix += 1];
                    dy += 1;
                    iif![dist_pos[dy] > width; break];
                }
                iif![npix == 0; break];
                npix = 0;
                step -= 1;
                iif![step < -max_extent; break];
            }
        }
        m_li.adjust_forward();
        step -= max_extent;

        Self {
            lp, x, y, old_x, old_y, count, width, max_extent,
            step, dist_pos, colors, di, start, li: m_li,
        }
    }

    #[inline]
    #[must_use]
    const fn vertical(&self) -> bool {
        self.lp.vertical
    }

    fn step_ver<T: Pixel>(&mut self, ren: &mut RendererOutlineImg<T>) -> bool {
        self.li.inc();
        self.y += self.lp.inc;
        self.x = (self.lp.x1 + self.li.y) >> POLY_SUBPIXEL_SHIFT;

        if self.lp.inc > 0 {
            self.di.inc_y_by(self.x - self.old_x);
        } else {
            self.di.dec_y_by(self.x - self.old_x);
        }
        self.old_x = self.x;

        let mut s1 = self.di.dist / self.lp.len;
        let s2 = -s1;

        if self.lp.inc > 0 {
            s1 = -s1;
        }

        let mut dist_start = self.di.dist_start;
        let mut dist_pict = self.di.dist_pict + self.start;
        let mut dist_end = self.di.dist_end;
        let mut p0 = MAX_HALF_WIDTH + 2;
        let mut p1 = p0;
        let mut npix = 0;
        self.colors[p1].clear();
        if dist_end > 0 {
            if dist_start <= 0 {
                self.colors[p1] = ren.pixel(dist_pict, s2);
            }
            npix += 1;
        }
        p1 += 1;

        let mut dx = 1;
        let mut dist = self.dist_pos[dx];
        while dist - s1 <= self.width {
            dist_start += self.di.dy_start;
            dist_pict += self.di.dy_pict;
            dist_end += self.di.dy_end;
            self.colors[p1].clear();
            if dist_end > 0 && dist_start <= 0 {
                if self.lp.inc > 0 {
                    dist = -dist;
                }
                self.colors[p1] = ren.pixel(dist_pict, s2 + dist);
                npix += 1;
            }
            p1 += 1;
            dx += 1;
            dist = self.dist_pos[dx];
        }

        dx = 1;
        dist_start = self.di.dist_start;
        dist_pict = self.di.dist_pict + self.start;
        dist_end = self.di.dist_end;
        dist = self.dist_pos[dx];
        while dist + s1 <= self.width {
            dist_start -= self.di.dy_start;
            dist_pict -= self.di.dy_pict;
            dist_end -= self.di.dy_end;
            p0 -= 1;
            self.colors[p0].clear();
            if dist_end > 0 && dist_start <= 0 {
                if self.lp.inc > 0 {
                    dist = -dist;
                }
                self.colors[p0] = ren.pixel(dist_pict, s2 - dist);
                npix += 1;
            }
            dx += 1;
            dist = self.dist_pos[dx];
        }

        ren.blend_color_hspan(
            self.x - dx as i64 + 1,
            self.y,
            (p1 - p0) as i64,
            &self.colors[p0..p1],
        );
        self.step += 1;

        npix != 0 && self.step < self.count
    }
    fn step_hor<T>(&mut self, ren: &mut RendererOutlineImg<T>) -> bool
    where
        T: Pixel,
    {
        self.li.inc();
        self.x += self.lp.inc;
        self.y = (self.lp.y1 + self.li.y) >> POLY_SUBPIXEL_SHIFT;

        if self.lp.inc > 0 {
            self.di.inc_x_by(self.y - self.old_y);
        } else {
            self.di.dec_x_by(self.y - self.old_y);
        }

        self.old_y = self.y;

        let mut s1 = self.di.dist / self.lp.len;
        let s2 = -s1;

        if self.lp.inc < 0 {
            s1 = -s1;
        }

        let mut dist_start = self.di.dist_start;
        let mut dist_pict = self.di.dist_pict + self.start;
        let mut dist_end = self.di.dist_end;
        let mut p0 = MAX_HALF_WIDTH + 2;
        let mut p1 = p0;

        let mut npix = 0;
        self.colors[p1].clear();
        if dist_end > 0 {
            if dist_start <= 0 {
                self.colors[p1] = ren.pixel(dist_pict, s2);
            }
            npix += 1;
        }
        p1 += 1;

        let mut dy = 1;
        let mut dist = self.dist_pos[dy];
        while dist - s1 <= self.width {
            dist_start -= self.di.dx_start;
            dist_pict -= self.di.dx_pict;
            dist_end -= self.di.dx_end;
            self.colors[p1].clear();
            if dist_end > 0 && dist_start <= 0 {
                if self.lp.inc > 0 {
                    dist = -dist;
                }
                self.colors[p1] = ren.pixel(dist_pict, s2 - dist);
                npix += 1;
            }
            p1 += 1;
            dy += 1;
            dist = self.dist_pos[dy];
        }

        dy = 1;
        dist_start = self.di.dist_start;
        dist_pict = self.di.dist_pict + self.start;
        dist_end = self.di.dist_end;
        dist = self.dist_pos[dy];
        while dist + s1 <= self.width {
            dist_start += self.di.dx_start;
            dist_pict += self.di.dx_pict;
            dist_end += self.di.dx_end;
            p0 -= 1;
            self.colors[p0].clear();
            if dist_end > 0 && dist_start <= 0 {
                if self.lp.inc > 0 {
                    dist = -dist;
                }
                self.colors[p0] = ren.pixel(dist_pict, s2 + dist);
                npix += 1;
            }
            dy += 1;
            dist = self.dist_pos[dy];
        }
        ren.blend_color_vspan(
            self.x,
            self.y - dy as i64 + 1,
            (p1 - p0) as i64,
            &self.colors[p0..p1],
        );
        self.step += 1;
        npix != 0 && self.step < self.count
    }
}
