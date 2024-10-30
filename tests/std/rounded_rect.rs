use agrega::{
    img_diff, render_scanlines, Ellipse, Pixfmt, RasterizerScanline, Render, RenderingBase,
    RenderingScanlineAASolid, Rgb8, RoundedRect, Stroke,
};

#[test]
fn rounded_rect() {
    let (w, h) = (600, 400);

    let m_x = [100., 500.];
    let m_y = [100., 350.];

    let pixf = Pixfmt::<Rgb8>::new(w, h);

    let mut ren_base = RenderingBase::new(pixf);

    ren_base.clear(Rgb8::white());

    let mut ren = RenderingScanlineAASolid::with_base(&mut ren_base);

    ren.color(Rgb8::new(255, 0, 0));

    let mut ras = RasterizerScanline::new();

    ren.color(Rgb8::new(54, 54, 54));
    for i in 0..2 {
        let e = Ellipse::new(m_x[i], m_y[i], 3., 3., 16);
        ras.add_path(&e);
        render_scanlines(&mut ras, &mut ren);
    }

    let d = 0.0f64;
    let mut r = RoundedRect::new(m_x[0] + d, m_y[0] + d, m_x[1] + d, m_y[1] + d, 36.0);
    r.normalize_radius();
    r.calc();
    let mut stroke = Stroke::new(r);
    stroke.width(7.0);
    ras.add_path(&stroke);
    ren.color(Rgb8::black());
    render_scanlines(&mut ras, &mut ren);

    ren.to_file("tests/std/tmp/rounded_rect.png").unwrap();
    assert!(img_diff("tests/std/tmp/rounded_rect.png", "tests/images/rounded_rect.png").unwrap(),);
}
