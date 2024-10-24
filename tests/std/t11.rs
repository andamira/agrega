#[test]
fn t11_full() {
    use agrega::Render;

    let (w, h) = (100, 100);

    let pixf = agrega::Pixfmt::<agrega::Rgb8>::new(w, h);

    let mut ren_base = agrega::RenderingBase::new(pixf);

    ren_base.clear(agrega::Rgba8::new(255, 255, 255, 255));

    let mut ren = agrega::RenderingScanlineAASolid::with_base(&mut ren_base);

    ren.color(agrega::Rgba8::new(255, 0, 0, 255));

    let mut ras = agrega::RasterizerScanline::new();

    ras.move_to(10.0, 10.0);
    ras.line_to(50.0, 90.0);
    ras.line_to(90.0, 10.0);

    agrega::render_scanlines(&mut ras, &mut ren);

    ren.to_file("tests/std/tmp/agg_test_11.png").unwrap();

    assert_eq!(
        agrega::ppm::img_diff("tests/std/tmp/agg_test_11.png", "tests/images/agg_test_11.png")
            .unwrap(),
        true
    );
}
