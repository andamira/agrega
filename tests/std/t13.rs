use agrega::Render;

#[test]
fn t13_aliased() {
    let (w, h) = (100, 100);

    let pixf = agrega::Pixfmt::<agrega::Rgb8>::new(w, h);

    let mut ren_base = agrega::RenderingBase::new(pixf);

    ren_base.clear(agrega::Rgba8::new(255, 255, 255, 255));

    let mut ren = agrega::RenderingScanlineBinSolid::with_base(&mut ren_base);

    ren.color(agrega::Rgba8::new(255, 0, 0, 255));

    let mut ras = agrega::RasterizerScanline::new();

    ras.clip_box(40.0, 0.0, w as f64 - 40.0, h as f64);

    ras.move_to(10.0, 10.0);
    ras.line_to(50.0, 90.0);
    ras.line_to(90.0, 10.0);

    agrega::render_scanlines(&mut ras, &mut ren);

    ren.to_file("tests/std/tmp/agg_test_13.png").unwrap();

    assert_eq!(
        agrega::ppm::img_diff("tests/std/tmp/agg_test_13.png", "tests/images/agg_test_13.png")
            .unwrap(),
        true
    );
}
