Anti Grain Geometry - Rust implementation

Originally derived from version 2.4 of [AGG](https://agg.sourceforge.net/antigrain.com/)

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

```
# #[cfg(feature = "std")]
# {
use agrega::{
    Pixfmt, RasterizerScanline, Render, RenderingBase, RenderingScanlineAASolid,
    Rgb8, Rgba8, render_scanlines,
};

// Create a blank image 10x10 pixels
let pix = Pixfmt::<Rgb8>::new(100,100);
let mut ren_base = RenderingBase::new(pix);
ren_base.clear(Rgba8::white());

// Draw a polygon from (10,10) - (50,90) - (90,10)
let mut ras = RasterizerScanline::new();
ras.move_to(10.0, 10.0);
ras.line_to(50.0, 90.0);
ras.line_to(90.0, 10.0);

// Render the line to the image
let mut ren = RenderingScanlineAASolid::with_base(&mut ren_base);
ren.color(Rgba8::black());
render_scanlines(&mut ras, &mut ren);

// Save the image to a file
ren_base.to_file("tests/std/tmp/little_black_triangle.png").unwrap();
# }
```

# Outline AntiAlias Renderer

```
# #[cfg(feature = "std")]
# {
use agrega::{
    Path, Pixfmt, Rgb8, Rgba8, DrawOutline, RasterizerOutlineAA,
    RendererOutlineAA, RenderingBase,
};
let pix = Pixfmt::<Rgb8>::new(100,100);
let mut ren_base = RenderingBase::new(pix);
ren_base.clear( Rgba8::new(255, 255, 255, 255) );

let mut ren = RendererOutlineAA::with_base(&mut ren_base);
ren.color(Rgba8::new(102,77,26,255));
ren.width(3.0);

let mut path = Path::new();
path.move_to(10.0, 10.0);
path.line_to(50.0, 90.0);
path.line_to(90.0, 10.0);

let mut ras = RasterizerOutlineAA::with_renderer(&mut ren);
ras.add_path(&path);
ren_base.to_file("tests/std/tmp/outline_aa.png").unwrap();
# }
```

# Primitive Renderer

Render for primitive shapes: lines, rectangles, and ellipses; filled or
   outlined.

```
# #[cfg(feature = "std")]
# {
use agrega::{
    Path, Pixfmt, Rgb8, Rgba8, RenderingBase, DrawOutline, RendererPrimitives,
    RasterizerOutline,
};

let pix = Pixfmt::<Rgb8>::new(100,100);
let mut ren_base = RenderingBase::new(pix);
ren_base.clear( Rgba8::new(255, 255, 255, 255) );

let mut ren = RendererPrimitives::with_base(&mut ren_base);
ren.line_color(Rgba8::new(0,0,0,255));

let mut path = Path::new();
path.move_to(10.0, 10.0);
path.line_to(50.0, 90.0);
path.line_to(90.0, 10.0);

let mut ras = RasterizerOutline::with_primitive(&mut ren);
ras.add_path(&path);
ren_base.to_file("tests/std/tmp/primitive.png").unwrap();
# }
```

# Raw Pixel Manipulation

  **Note:** Functions here are a somewhat low level interface and probably not what
    you want to use.

  Functions to set pixel color through [`Pixfmt`] are [`clear`], [`set`],
  [`copy_pixel`], [`copy_hline`], [`copy_vline`], [`fill`]

  Functions to blend colors with existing pixels through [`Pixfmt`] are
  - `Pixel::`[`copy_or_blend_pix`][Pixel#method.copy_or_blend_pix]
      ([`_with_cover`][Pixel#method.copy_or_blend_pix_with_cover])
  - `Pixel::blend_`[[`hline`][Pixel#method.blend_hline]|
      [`vline`][Pixel#method.blend_vline]]
  - `Pixel::blend_solid_`[[`hspan`][Pixel#method.blend_solid_vspan]|
      [`vspan`][Pixel#method.blend_solid_vspan]]
  - `Pixel::blend_color_`[[`hspan`][Pixel#method.blend_color_vspan]|
      [`vspan`][Pixel#method.blend_color_vspan]]

[`Pixfmt`]: struct.Pixfmt.html
[`clear`]: struct.Pixfmt.html#method.clear
[`set`]: struct.Pixfmt.html#method.set
[`copy_pixel`]: struct.Pixfmt.html#method.copy_pixel
[`copy_hline`]: struct.Pixfmt.html#method.copy_hline
[`copy_vline`]: struct.Pixfmt.html#method.copy_vline
[`fill`]: trait.PixelDraw.html#method.fill
[`render_scanlines`]: render/fn.render_scanlines.html
[`render_all_paths`]: render/fn.render_all_paths.html
[`render_scanlines_aa_solid`]: render/fn.render_scanlines_aa_solid.html
[`render_scanlines_bin_solid`]: render/fn.render_scanlines_bin_solid.html
