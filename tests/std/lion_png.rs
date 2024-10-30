use agrega::{
    bounding_rect, img_diff, render_all_paths, Path, Pixfmt, RasterizerScanline, Rectangle, Render,
    RenderingBase, RenderingScanlineBinSolid, Rgb8, Transform,
};

#[test]
fn lion_png() {
    let (w, h) = (400, 400);

    let (paths, colors) = super::parse_lion();
    let pixf = Pixfmt::<Rgb8>::new(w, h);
    let mut ren_base = RenderingBase::new(pixf);
    ren_base.clear(Rgb8::white());
    let mut ren = RenderingScanlineBinSolid::with_base(&mut ren_base);
    ren.color(Rgb8::new(255, 0, 0));

    let mut ras = RasterizerScanline::new();

    if paths.is_empty() {
        return;
    }
    let p = paths[0].vertices[0];
    let mut r = Rectangle::new(p.x, p.y, p.x, p.y);
    for p in &paths {
        if let Some(rp) = bounding_rect(p) {
            r.expand_rect(&rp);
        }
    }
    let g_base_dx = (r.x2() - r.x1()) / 2.0;
    let g_base_dy = (r.y2() - r.y1()) / 2.0;

    let mut mtx = Transform::new();
    mtx.translate(-g_base_dx, -g_base_dy);
    mtx.translate((w / 2) as f64, (h / 2) as f64);
    let t: Vec<Path> = paths.into_iter().map(|p| p.transformed(&mtx)).collect();

    render_all_paths(&mut ras, &mut ren, &t, &colors);

    ren.to_file("tests/std/tmp/lion.png").unwrap();

    assert![img_diff("tests/std/tmp/lion.png", "tests/images/lion.png").unwrap()]
}
