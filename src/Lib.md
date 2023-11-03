Anti Grain Geometry - Rust implementation

Originally derived from version 2.4 of [AGG](https://franko.github.io/antigrain/)

This crate implments the drawing / painting 2D algorithms developed in the Anti
Grain Geometry C++ library. Quoting from the author in the documentation:

> **Anti-Grain Geometry** is not a solid graphic library and it's not very easy
  to use. I consider **AGG** as a **"tool to create other tools"**. It means
  that there's no **"Graphics"** object or something like that, instead,
  **AGG** consists of a number of loosely coupled algorithms that can be used
  together or separately. All of them have well defined interfaces and absolute
  minimum of implicit or explicit dependencies.


# Anti-Aliasing and Subpixel Accuracy

One primary strenght of AGG are the combination of drawing with subpixel
accuracy with anti-aliasing effects.  There are many examples within the
documentation and reproduced here.

# Drawing

There are multiple ways to put / draw pixels including:

  - Scanline Renderers
    - Antialiased or Aliased (Binary)
  - Outline Renderer, possibly with Images
  - Raw Pixel Manipulation

# Scanline Renderer

 The multitude of renderers here include [`render_scanlines`],
   [`render_all_paths`], [`render_scanlines_aa_solid`] and
   [`render_scanlines_bin_solid`]

      use agg::Render;

      // Create a blank image 10x10 pixels
      let pix = agg::Pixfmt::<agg::Rgb8>::new(100,100);
      let mut ren_base = agg::RenderingBase::new(pix);
      ren_base.clear(agg::Rgba8::white());

      // Draw a polygon from (10,10) - (50,90) - (90,10)
      let mut ras = agg::RasterizerScanline::new();
      ras.move_to(10.0, 10.0);
      ras.line_to(50.0, 90.0);
      ras.line_to(90.0, 10.0);

      // Render the line to the image
      let mut ren = agg::RenderingScanlineAASolid::with_base(&mut ren_base);
      ren.color(agg::Rgba8::black());
      agg::render_scanlines(&mut ras, &mut ren);

      // Save the image to a file
      ren_base.to_file("little_black_triangle.png").unwrap();


# Outline AntiAlias Renderer

       use agg::{Pixfmt,Rgb8,Rgba8,RenderingBase,DrawOutline};
       use agg::{RendererOutlineAA,RasterizerOutlineAA};
       let pix = Pixfmt::<Rgb8>::new(100,100);
       let mut ren_base = agg::RenderingBase::new(pix);
       ren_base.clear( Rgba8::new(255, 255, 255, 255) );

       let mut ren = RendererOutlineAA::with_base(&mut ren_base);
       ren.color(agg::Rgba8::new(102,77,26,255));
       ren.width(3.0);

       let mut path = agg::Path::new();
       path.move_to(10.0, 10.0);
       path.line_to(50.0, 90.0);
       path.line_to(90.0, 10.0);

       let mut ras = RasterizerOutlineAA::with_renderer(&mut ren);
       ras.add_path(&path);
       ren_base.to_file("outline_aa.png").unwrap();

# Primitive Renderer

Render for primitive shapes: lines, rectangles, and ellipses; filled or
   outlined.

       use agg::{Pixfmt,Rgb8,Rgba8,RenderingBase,DrawOutline};
       use agg::{RendererPrimitives,RasterizerOutline};

       let pix = Pixfmt::<Rgb8>::new(100,100);
       let mut ren_base = agg::RenderingBase::new(pix);
       ren_base.clear( Rgba8::new(255, 255, 255, 255) );

       let mut ren = RendererPrimitives::with_base(&mut ren_base);
       ren.line_color(agg::Rgba8::new(0,0,0,255));

       let mut path = agg::Path::new();
       path.move_to(10.0, 10.0);
       path.line_to(50.0, 90.0);
       path.line_to(90.0, 10.0);

       let mut ras = RasterizerOutline::with_primitive(&mut ren);
       ras.add_path(&path);
       ren_base.to_file("primitive.png").unwrap();


# Raw Pixel Manipulation

  **Note:** Functions here are a somewhat low level interface and probably not what
    you want to use.

  Functions to set pixel color through [`Pixfmt`] are [`clear`], [`set`],
  [`copy_pixel`], [`copy_hline`], [`copy_vline`], [`fill`]

  Functions to blend colors with existing pixels through [`Pixfmt`] are
  [`copy_or_blend_pix`], [`copy_or_blend_pix_with_cover`], [`blend_hline`],
  [`blend_vline`], [`blend_solid_hspan`], [`blend_solid_vspan`],
  [`blend_color_hspan`], [`blend_color_vspan`]


[`Pixfmt`]: pixfmt/struct.Pixfmt.html
[`clear`]: pixfmt/struct.Pixfmt.html#method.clear
[`set`]: pixfmt/struct.Pixfmt.html#method.set
[`copy_pixel`]: pixfmt/struct.Pixfmt.html#method.copy_pixel
[`copy_hline`]: pixfmt/struct.Pixfmt.html#method.copy_hline
[`copy_vline`]: pixfmt/struct.Pixfmt.html#method.copy_vline
[`fill`]: pixfmt/trait.PixelDraw.html#method.fill
[`copy_or_blend_pix`]: pixfmt/trait.PixelDraw.html#method.copy_or_blend_pix
[`copy_or_blend_pix_with_cover`]: pixfmt/trait.PixelDraw.html#method.copy_or_blend_pix_with_cover
[`blend_hline`]: pixfmt/trait.PixelDraw.html#method.blend_hline
[`blend_vline`]: pixfmt/trait.PixelDraw.html#method.blend_vline
[`blend_solid_hspan`]: pixfmt/trait.PixelDraw.html#method.blend_solid_hspan
[`blend_solid_vspan`]: pixfmt/trait.PixelDraw.html#method.blend_solid_vspan
[`blend_color_hspan`]: pixfmt/trait.PixelDraw.html#method.blend_color_hspan
[`blend_color_vspan`]: pixfmt/trait.PixelDraw.html#method.blend_color_vspan
[`render_scanlines`]: render/fn.render_scanlines.html
[`render_all_paths`]: render/fn.render_all_paths.html
[`render_scanlines_aa_solid`]: render/fn.render_scanlines_aa_solid.html
[`render_scanlines_bin_solid`]: render/fn.render_scanlines_bin_solid.html