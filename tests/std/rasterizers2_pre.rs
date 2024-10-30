#![cfg_attr(not(feature = "freetype"), allow(unused))]

use super::{text_h8_sw07 as text, Roundoff, Spiral};
use agrega::{
    img_diff, render_scanlines, DrawOutline, LineImagePatternPow2, PatternFilterBilinear, Pixel,
    Pixfmt, RasterizerOutline, RasterizerOutlineAA, RasterizerScanline, Render, RendererOutline,
    RendererOutlineAA, RendererOutlineImg, RenderingBase, RenderingScanlineAASolid, Rgb8, Rgba32,
    Rgba8pre, Srgba8, Stroke,
};

fn chain() -> Pixfmt<Rgba32> {
    let width = 16;
    let height = 7;
    let mut pix = Pixfmt::<Rgba32>::new(width, height);
    let raw: [u32; 16 * 7] = [
        0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0xb4c29999, 0xff9a5757, 0xff9a5757,
        0xff9a5757, 0xff9a5757, 0xff9a5757, 0xff9a5757, 0xb4c29999, 0x00ffffff, 0x00ffffff,
        0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x0cfbf9f9, 0xff9a5757, 0xff660000,
        0xff660000, 0xff660000, 0xff660000, 0xff660000, 0xff660000, 0xff660000, 0xff660000,
        0xb4c29999, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x5ae0cccc, 0xffa46767,
        0xff660000, 0xff975252, 0x7ed4b8b8, 0x5ae0cccc, 0x5ae0cccc, 0x5ae0cccc, 0x5ae0cccc,
        0xa8c6a0a0, 0xff7f2929, 0xff670202, 0x9ecaa6a6, 0x5ae0cccc, 0x00ffffff, 0xff660000,
        0xff660000, 0xff660000, 0xff660000, 0xff660000, 0xff660000, 0xa4c7a2a2, 0x3affff00,
        0x3affff00, 0xff975151, 0xff660000, 0xff660000, 0xff660000, 0xff660000, 0xff660000,
        0xff660000, 0x00ffffff, 0x5ae0cccc, 0xffa46767, 0xff660000, 0xff954f4f, 0x7ed4b8b8,
        0x5ae0cccc, 0x5ae0cccc, 0x5ae0cccc, 0x5ae0cccc, 0xa8c6a0a0, 0xff7f2929, 0xff670202,
        0x9ecaa6a6, 0x5ae0cccc, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x0cfbf9f9, 0xff9a5757,
        0xff660000, 0xff660000, 0xff660000, 0xff660000, 0xff660000, 0xff660000, 0xff660000,
        0xff660000, 0xb4c29999, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff,
        0x00ffffff, 0x00ffffff, 0xb4c29999, 0xff9a5757, 0xff9a5757, 0xff9a5757, 0xff9a5757,
        0xff9a5757, 0xff9a5757, 0xb4c29999, 0x00ffffff, 0x00ffffff, 0x00ffffff, 0x00ffffff,
    ];
    let mut colors = vec![];
    for v in raw.iter() {
        let r = ((v >> 16) & 0x0000_00ff_u32) as u8;
        let g = ((v >> 8) & 0x00ff_u32) as u8;
        let b = ((v) & 0x00ff_u32) as u8;
        let a = (v >> 24) as u8;
        let c = Rgba32::from_trait(Srgba8::new(r, g, b, a));
        colors.push(c.premultiply());
    }
    let mut k = 0;
    for j in 0..height {
        for i in 0..width {
            pix.set((i, j), colors[k]);
            k += 1;
        }
    }
    pix
}

#[test]
fn rasterizers2_pre() {
    let (w, h) = (500, 450);

    let pixf = Pixfmt::<Rgba8pre>::new(w, h);
    let mut ren_base = RenderingBase::new(pixf);

    ren_base.clear(Rgb8::new(255, 255, 242));

    let start_angle = 0.0;
    let line_width = 3.0;
    let _width = w as f64;
    let height = h as f64;
    let (r1, r2) = (5.0, 70.0);
    let step = 16.0;
    // Anti-aliased Scanline Spiral
    {
        let x = (w / 2) as f64;
        let y = (h - h / 4 + 20) as f64;
        let spiral = Spiral::new(x, y, r1, r2, step, start_angle);

        let mut ras_aa = RasterizerScanline::new();
        let mut ren_aa = RenderingScanlineAASolid::with_base(&mut ren_base);
        let mut stroke = Stroke::new(spiral);
        stroke.width(line_width);
        //stroke.cap(round_cap);
        ren_aa.color(Rgb8::new(102, 77, 26));
        ras_aa.add_path(&stroke);
        render_scanlines(&mut ras_aa, &mut ren_aa);
    }
    // Aliased Pixel Accuracy
    {
        let x = (w / 5) as f64;
        let y = (h / 4 + 50) as f64;
        let spiral = Spiral::new(x, y, r1, r2, step, start_angle);

        let mut ren_prim = RendererOutline::with_base(&mut ren_base);
        ren_prim.line_color(Rgb8::new(102, 77, 26));
        let mut ras_al = RasterizerOutline::with_primitive(&mut ren_prim);
        let trans = Roundoff::new(spiral);
        ras_al.add_path(&trans);
    }
    // Aliased Subpixel Accuracy
    {
        let x = (w / 2) as f64;
        let y = (h / 4 + 50) as f64;
        eprintln!("DDA SPIRAL: {} {} h {} h/4 {}", x, y, height, height / 4.0);
        let spiral = Spiral::new(x, y, r1, r2, step, start_angle);

        let mut ren_prim = RendererOutline::with_base(&mut ren_base);
        ren_prim.line_color(Rgb8::new(102, 77, 26));
        let mut ras_al = RasterizerOutline::with_primitive(&mut ren_prim);
        ras_al.add_path(&spiral);
    }
    // Anti-Aliased Outline
    {
        let x = (w / 5) as f64;
        let y = (h - h / 4 + 20) as f64;
        let spiral = Spiral::new(x, y, r1, r2, step, start_angle);

        let mut ren_oaa = RendererOutlineAA::with_base(&mut ren_base);
        ren_oaa.color(Rgb8::new(102, 77, 26));
        ren_oaa.width(3.0);
        let mut ras_oaa = RasterizerOutlineAA::with_renderer(&mut ren_oaa);
        ras_oaa.round_cap(true);
        ras_oaa.add_path(&spiral);
    }
    // Anti-Aliased Outline Image
    {
        let x = (w - w / 5) as f64;
        let y = (h - h / 4 + 20) as f64;
        let spiral = Spiral::new(x, y, r1, r2, step, start_angle);

        //let ren_oaa = RendererOutlineAA::with_base(&mut ren_base);

        let filter = PatternFilterBilinear::new();
        let mut pattern = LineImagePatternPow2::new(filter);
        let ch = chain();
        pattern.create(&ch);
        let mut ren_img = RendererOutlineImg::with_base_and_pattern(&mut ren_base, pattern);
        let mut ras_img = RasterizerOutlineAA::with_renderer(&mut ren_img);
        //ren_oaa.color(&Rgb8::new(102,77,26));
        ras_img.round_cap(true);
        ras_img.add_path(&spiral);
    }

    {
        let mut ras_aa = RasterizerScanline::new();
        let mut ren_aa = RenderingScanlineAASolid::with_base(&mut ren_base);
        text(&mut ras_aa, &mut ren_aa, 50.0, 75.0, "Bresenham lines,\n\nregular accuracy");
        text(
            &mut ras_aa,
            &mut ren_aa,
            (w / 2 - 50) as f64,
            75.0,
            "Bresenham lines,\n\nsubpixel accuracy",
        );
        text(&mut ras_aa, &mut ren_aa, 50., (h / 2 + 50) as f64, "Anti-aliased lines");
        text(
            &mut ras_aa,
            &mut ren_aa,
            (w / 2 - 50) as f64,
            (h / 2 + 50) as f64,
            "Scanline rasterizer",
        );
        text(
            &mut ras_aa,
            &mut ren_aa,
            (w - w / 5 - 50) as f64,
            (h / 2 + 50) as f64,
            "Arbitrary Image Pattern",
        );
    }

    // Remove alpha channel from data
    let data = ren_base.as_bytes();
    let mut out = vec![];
    #[allow(clippy::needless_range_loop)]
    for i in 0..data.len() {
        if i % 4 < 3 {
            out.push(data[i]);
        }
    }
    ren_base
        .pixf
        .drop_alpha()
        .to_file("tests/std/tmp/rasterizers2_pre.png")
        .unwrap();
    assert!(
        img_diff("tests/std/tmp/rasterizers2_pre.png", "tests/images/rasterizers2_pre.png",)
            .unwrap()
    );
}
