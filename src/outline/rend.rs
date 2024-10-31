// agrega::outline::rend

use crate::{
    BresehamInterpolator, Color, Pixel, RenderingBase, Rgba8, Subpixel, POLY_SUBPIXEL_SCALE,
};
#[allow(unused_imports)]
use devela::ExtFloat;

/// Renderer for drawing primitive shapes on a pixel grid.
#[derive(Debug)]
pub struct RendererOutline<'a, T: 'a> {
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

impl<'a, T: Pixel> RendererOutline<'a, T> {
    /// Creates a new `RendererOutline` with the provided `RenderingBase`.
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

    /* private */

    /// Converts a floating-point coordinate to `Subpixel` units.
    #[inline]
    #[must_use]
    pub(crate) fn coord(&self, c: f64) -> Subpixel {
        Subpixel::from((c * POLY_SUBPIXEL_SCALE as f64).round() as i64)
    }

    /// Moves the current drawing position to `(x, y)`.
    #[inline]
    pub(crate) fn move_to_sp(&mut self, x: Subpixel, y: Subpixel) {
        self.x = x;
        self.y = y;
    }
    /// Draws a line from the current position to `(x, y)`, then updates the position.
    pub(crate) fn line_to_sp(&mut self, x: Subpixel, y: Subpixel) {
        self.line_sp(self.x, self.y, x, y);
        self.x = x;
        self.y = y;
    }

    /// Draws a line between `(x1, y1)` and `(x2, y2)` with the current line color.
    ///
    /// Uses a Bresenham interpolator for efficient rasterization and applies
    /// subpixel masking based on the pixel format's cover mask.
    fn line_sp(&mut self, x1: Subpixel, y1: Subpixel, x2: Subpixel, y2: Subpixel) {
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
