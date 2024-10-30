// agrega::scanline
//
//! # Examples
//!
//! ## Rasterizer for Outlines with Anti-Aliasing
//! <img style="margin: 16; display:block; margin:auto;"
//! src="https://raw.githubusercontent.com/andamira/agrega/master/tests/images/little_black_triangle.png">
//! ```
//! # #[cfg(feature = "std")]
//! # {
//! # use agrega::{
//! #   Pixfmt, RasterizerScanline, Render, RenderingBase, RenderingScanlineAASolid,
//! #    Rgb8, Rgba8, render_scanlines,
//! # };
//! // Create a blank image 10x10 pixels
//! let pix = Pixfmt::<Rgb8>::new(100,100);
//! let mut ren_base = RenderingBase::new(pix);
//! ren_base.clear(Rgba8::white());
//!
//! // Draw a polygon from (10,10) - (50,90) - (90,10)
//! let mut ras = RasterizerScanline::new();
//! ras.move_to(10.0, 10.0);
//! ras.line_to(50.0, 90.0);
//! ras.line_to(90.0, 10.0);
//!
//! // Render the line to the image
//! let mut ren = RenderingScanlineAASolid::with_base(&mut ren_base);
//! ren.color(Rgba8::black());
//! render_scanlines(&mut ras, &mut ren);
//!
//! // Save the image to a file
//! ren_base.to_file("tests/std/tmp/little_black_triangle.png").unwrap();
//! # }
//! ```

mod scan;
#[allow(unused_imports)]
pub use scan::*;

#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
items! {
    mod rast;
    mod rend;
    pub use {rast::*, rend::*};
}
