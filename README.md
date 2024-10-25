# agrega

[![Crate](https://img.shields.io/crates/v/agrega.svg)](https://crates.io/crates/agrega)
[![API](https://docs.rs/agrega/badge.svg)](https://docs.rs/agrega/)
[![MSRV: 1.82.0](https://flat.badgen.net/badge/MSRV/1.82.0/purple)](https://releases.rs/docs/1.82.0/)
<br/>
[![Code](https://tokei.rs/b1/github/andamira/agrega?category=code)](https://github.com/andamira/agrega)
[![Documentation](https://tokei.rs/b1/github/andamira/agrega?category=comments)](https://docs.rs/agrega/)
[![Files](https://tokei.rs/b1/github/andamira/agrega?category=files)](https://github.com/andamira/agrega/tree/main/)
---

A Rust port of [Anti-Grain Geometry](https://agg.sourceforge.net/antigrain.com/).

> A High Fidelity and Quality 2D Graphics Rendering Engine

## Example

![Little Black Triangle](https://github.com/andamira/agrega/blob/master/tests/std/tmp/little_black_triangle.png)

```rust
use agrega::{
  Pixfmt, RasterizerScanline, Render, RenderingBase, RenderingScanlineAASolid,
  Rgb8, Rgba8, render_scanlines
};

// Create a blank image 10x10 pixels
let pix = Pixfmt::<Rgb8>::new(100, 100);
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
```

## Features
  - Anti-Aliased Drawing
  - Sub-pixel Accuracy
  - Rendering of Arbitrary Polygons
  - Text/Font Rendering (with [Freetype](https://www.freetype.org/))


## Complexity

Quoting the original C++ library:

> **Anti-Grain Geometry** is not a solid graphic library and it's not very easy
  to use. I consider **AGG** as a **"tool to create other tools"**. It means
  that there's no **"Graphics"** object or something like that, instead,
  **AGG** consists of a number of loosely coupled algorithms that can be used
  together or separately. All of them have well defined interfaces and absolute
  minimum of implicit or explicit dependencies.

## License
This project is licensed under the **BSD 2-Clause License**. Additionally,
modifications made in this Rust port are available under your choice of
**Apache License, Version 2.0** or the **MIT License**.

### License Note
This Rust project is based on the `agg-2.4` (2005) C++ library, which was originally
licensed under a dual-license model. The prior Rust port of `agg-2.4` selected
the BSD 2-Clause License as its sole license, which has been retained here,
alongside the Apache-2.0 and MIT licenses which apply to the new code.
