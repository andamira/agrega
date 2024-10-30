// agrega::outline::rast
//

use crate::{PathCommand, Pixel, RendererOutline, Subpixel, VertexSource};

/// Rasterizer for Outlined Shapes.
///
/// The rendering is directly attached and drawing is done immediately.
#[derive(Debug)]
#[must_use]
pub struct RasterizerOutline<'a, T: Pixel> {
    ren: &'a mut RendererOutline<'a, T>,
    start_x: Subpixel,
    start_y: Subpixel,
    vertices: usize,
}
impl<'a, T: Pixel> RasterizerOutline<'a, T> {
    /// Create a new RasterizerOutline with a Renderer
    #[inline]
    pub fn with_primitive(ren: &'a mut RendererOutline<'a, T>) -> Self {
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
            self.line_to_sp(x, y);
        }
        self.vertices = 0;
    }

    /// Moves the current position to (`x`,`y`).
    pub fn move_to_d(&mut self, x: f64, y: f64) {
        let (xs, ys) = (self.ren.coord(x), self.ren.coord(y));
        self.move_to_sp(xs, ys);
    }

    /// Draws a line from the current position to position (`x`,`y`).
    #[inline]
    pub fn line_to_d(&mut self, x: f64, y: f64) {
        let (x, y) = (self.ren.coord(x), self.ren.coord(y));
        self.line_to_sp(x, y);
    }

    /// Move the current position to (`x`,`y`).
    #[inline]
    fn move_to_sp(&mut self, x: Subpixel, y: Subpixel) {
        self.vertices = 1;
        self.start_x = x;
        self.start_y = y;
        self.ren.move_to_sp(x, y);
    }

    /// Draws a line from the current position to position (`x`,`y`).
    #[inline]
    fn line_to_sp(&mut self, x: Subpixel, y: Subpixel) {
        self.vertices += 1;
        self.ren.line_to_sp(x, y);
    }
}
