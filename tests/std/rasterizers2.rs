use super::{Roundoff, Spiral};
use agrega::{
    img_diff, render_scanlines, Pixfmt, RasterizerOutline, RasterizerScanline, Render,
    RendererOutline, RenderingBase, RenderingScanlineAASolid, Rgb8, Stroke,
};

#[test]
fn rasterizers2() {
    let (w, h) = (500, 450);

    let pixf = Pixfmt::<Rgb8>::new(w, h);
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

    ren_base.to_file("tests/std/tmp/rasterizers2.png").unwrap();
    assert!(img_diff("tests/std/tmp/rasterizers2.png", "tests/images/rasterizers2.png").unwrap(),);
}
