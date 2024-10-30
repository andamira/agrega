//! Rendering Outline, not Anti-Aliased
//!
//! ```
//! use agrega::{
//!     Path, Pixfmt, Rgb8, Rgba8, RasterizerOutline, RenderingBase, RendererPrimitives,
//! };
//! let pix = Pixfmt::<Rgb8>::new(100,100);
//! let mut ren_base = RenderingBase::new(pix);
//! ren_base.clear(Rgba8::new(255, 255, 255, 255) );
//!
//! let mut ren = RendererPrimitives::with_base(&mut ren_base);
//! ren.line_color(Rgba8::new(0,0,0,255));
//!
//! let mut path = Path::new();
//! path.move_to(10.0, 10.0);
//! path.line_to(50.0, 90.0);
//! path.line_to(90.0, 10.0);
//!
//! let mut ras = RasterizerOutline::with_primitive(&mut ren);
//! ras.add_path(&path);
//! # #[cfg(feature = "std")]
//! ren_base.to_file("tests/std/tmp/primitive.png").unwrap();
//! ```
//!
//! The above code produces:<br/>
//! ![Output](https://raw.githubusercontent.com/andamira/agrega/master/tests/images/primitive.png)

use crate::{
    base::RenderingBase, color::Rgba8, paths::PathCommand, render::BresehamInterpolator, Color,
    Pixel, VertexSource, POLY_SUBPIXEL_SCALE, POLY_SUBPIXEL_SHIFT,
};
#[allow(unused_imports)]
use devela::ExtFloat;

/// Represents a coordinate with subpixel precision, stored as a fixed-point integer.
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub(crate) struct Subpixel(i64);
impl Subpixel {
    /// Returns the raw subpixel value as an `i64`.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn value(self) -> i64 { self.0 }
}
impl From<i64> for Subpixel {
    /// Creates a `Subpixel` from an `i64`.
    #[inline] #[rustfmt::skip]
    fn from(v: i64) -> Self { Subpixel(v) }
}
impl From<Subpixel> for i64 {
    /// Converts a `Subpixel` to an `i64`.
    #[inline] #[rustfmt::skip]
    fn from(v: Subpixel) -> Self { v.0 >> POLY_SUBPIXEL_SHIFT }
}

/// Rasterizer for Outlined Shapes.
///
/// The rendering is directly attached and drawing is done immediately.
#[derive(Debug)]
#[must_use]
pub struct RasterizerOutline<'a, T: Pixel> {
    ren: &'a mut RendererPrimitives<'a, T>,
    start_x: Subpixel,
    start_y: Subpixel,
    vertices: usize,
}
impl<'a, T: Pixel> RasterizerOutline<'a, T> {
    /// Create a new RasterizerOutline with a Renderer
    #[inline]
    pub fn with_primitive(ren: &'a mut RendererPrimitives<'a, T>) -> Self {
        Self { start_x: Subpixel::from(0), start_y: Subpixel::from(0), vertices: 0, ren }
    }

    /// Add a path and render
    pub fn add_path<VS: VertexSource>(&mut self, path: &VS) {
        for v in path.xconvert().iter() {
            match v.cmd {
                PathCommand::MoveTo => self.move_to_d(v.x, v.y),
                PathCommand::LineTo => self.line_to_d(v.x, v.y),
                PathCommand::Close => self.close(),
                PathCommand::Stop => unimplemented!("stop encountered"),
            }
        }
    }

    /// Closes the current path.
    pub fn close(&mut self) {
        if self.vertices > 2 {
            let (x, y) = (self.start_x, self.start_y);
            self.line_to(x, y);
        }
        self.vertices = 0;
    }

    /// Moves the current position to (`x`,`y`).
    pub fn move_to_d(&mut self, x: f64, y: f64) {
        let (x, y) = (self.ren.coord(x), self.ren.coord(y));
        self.move_to(x, y);
    }

    /// Draws a line from the current position to position (`x`,`y`).
    pub fn line_to_d(&mut self, x: f64, y: f64) {
        let (x, y) = (self.ren.coord(x), self.ren.coord(y));
        self.line_to(x, y);
    }

    /// Move the current position to (`x`,`y`).
    fn move_to(&mut self, x: Subpixel, y: Subpixel) {
        self.vertices = 1;
        self.start_x = x;
        self.start_y = y;
        self.ren.move_to(x, y);
    }
    /// Draws a line from the current position to position (`x`,`y`).
    fn line_to(&mut self, x: Subpixel, y: Subpixel) {
        self.vertices += 1;
        self.ren.line_to(x, y);
    }
}

/// Renderer for drawing primitive shapes on a pixel grid.
#[derive(Debug)]
pub struct RendererPrimitives<'a, T: 'a> {
    /// Reference to the base rendering engine.
    base: &'a mut RenderingBase<T>,
    /// Fill color used for filled shapes.
    fill_color: Rgba8,
    /// Line color used for outline strokes.
    line_color: Rgba8,
    /// X coordinate in subpixel precision.
    x: Subpixel,
    /// Y coordinate in subpixel precision.
    y: Subpixel,
}

impl<'a, T: Pixel> RendererPrimitives<'a, T> {
    /// Creates a new `RendererPrimitives` with the provided `RenderingBase`.
    ///
    /// Sets default colors (black) for fill and line, and initializes coordinates to `(0, 0)`.
    pub fn with_base(base: &'a mut RenderingBase<T>) -> Self {
        let [fill_color, line_color] = [Rgba8::black(); 2];
        let [x, y] = [Subpixel::from(0); 2];
        Self { base, fill_color, line_color, x, y }
    }

    /// Sets the color for outline strokes.
    #[inline]
    pub fn line_color<C: Color>(&mut self, line_color: C) {
        self.line_color = Rgba8::from_trait(line_color);
    }
    /// Sets the fill color for shapes.
    #[inline]
    pub fn fill_color<C: Color>(&mut self, fill_color: C) {
        self.fill_color = Rgba8::from_trait(fill_color);
    }

    /// Converts a floating-point coordinate to `Subpixel` units.
    #[inline]
    pub(crate) fn coord(&self, c: f64) -> Subpixel {
        Subpixel::from((c * POLY_SUBPIXEL_SCALE as f64).round() as i64)
    }
    /// Moves the current drawing position to `(x, y)`.
    #[inline]
    pub(crate) fn move_to(&mut self, x: Subpixel, y: Subpixel) {
        self.x = x;
        self.y = y;
    }
    /// Draws a line from the current position to `(x, y)`, then updates the position.
    pub(crate) fn line_to(&mut self, x: Subpixel, y: Subpixel) {
        self.line(self.x, self.y, x, y);
        self.x = x;
        self.y = y;
    }

    /// Draws a line between `(x1, y1)` and `(x2, y2)` with the current line color.
    ///
    /// Uses a Bresenham interpolator for efficient rasterization and applies
    /// subpixel masking based on the pixel format's cover mask.
    fn line(&mut self, x1: Subpixel, y1: Subpixel, x2: Subpixel, y2: Subpixel) {
        let mut li = BresehamInterpolator::new(x1, y1, x2, y2);
        if li.len == 0 {
            return;
        }

        //let cover_shift = POLY_SUBPIXEL_SCALE;
        //let cover_size = 1 << cover_shift;
        //let cover_mask = cover_size - 1;
        //let cover_full = cover_mask;

        let mask = T::cover_mask();
        let color = self.line_color;

        if li.ver {
            for _ in 0..li.len {
                //self.base.pixf.set((li.x2 as usize, li.y1 as usize), color);
                self.base.blend_hline(li.x2, li.y1, li.x2, color, mask);
                li.vstep();
            }
        } else {
            for _ in 0..li.len {
                self.base.blend_hline(li.x1, li.y2, li.x1, color, mask);
                li.hstep();
            }
        }
    }
}
