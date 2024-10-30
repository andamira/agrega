use agrega::{
    img_diff, render_scanlines, render_scanlines_aa_solid, Path, Pixfmt, RasterizerScanline,
    Render, RenderingBase, RenderingScanlineAASolid, Rgb8, Stroke,
};

#[test]
fn t15_path_stroke() {
    let (w, h) = (100, 100);

    let pixf = Pixfmt::<Rgb8>::new(w, h);

    let mut ren_base = RenderingBase::new(pixf);

    ren_base.clear(Rgb8::white());

    let mut ren = RenderingScanlineAASolid::with_base(&mut ren_base);

    ren.color(Rgb8::new(255, 0, 0));

    let mut ras = RasterizerScanline::new();

    ras.clip_box(40.0, 0.0, w as f64 - 40.0, h as f64);

    ras.reset();
    ras.move_to(10.0, 10.0);
    ras.line_to(50.0, 90.0);
    ras.line_to(90.0, 10.0);

    render_scanlines(&mut ras, &mut ren);

    let mut ps = Path::new();
    ps.remove_all();
    ps.move_to(10.0, 10.0);
    ps.line_to(50.0, 90.0);
    ps.line_to(90.0, 10.0);
    ps.line_to(10.0, 10.0);

    let mut pg = Stroke::new(ps);

    pg.width(2.0);
    ras.add_path(&pg);

    render_scanlines_aa_solid(&mut ras, &mut ren_base, Rgb8::black());

    ren_base.to_file("tests/std/tmp/agg_test_15.png").unwrap();

    assert!(img_diff("tests/std/tmp/agg_test_15.png", "tests/images/agg_test_15.png").unwrap(),);
}
