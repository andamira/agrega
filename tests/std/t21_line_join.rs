use agrega::{
    render_scanlines, GsvText, LineJoin, Path, Pixel, Pixfmt, RasterizerScanline, Render,
    RenderingBase, RenderingScanlineAASolid, Rgb8, Rgba8, Stroke,
};

#[test]
fn t21_line_join() {
    let pix = Pixfmt::<Rgb8>::new(300, 100);
    let mut ren_base = RenderingBase::new(pix);
    ren_base.clear(Rgba8::new(255, 255, 255, 255));

    let joins = [LineJoin::Miter, LineJoin::Round, LineJoin::Bevel];
    for (i, join) in joins.iter().enumerate() {
        let dx = 100.0 * i as f64;
        let mut path = Path::new();
        path.move_to(10.0 + dx, 70.0);
        path.line_to(50.0 + dx, 30.0);
        path.line_to(90.0 + dx, 70.0);

        let mut stroke = Stroke::new(path);
        stroke.width(25.0);
        stroke.line_join(*join);

        let mut ras = RasterizerScanline::new();
        ras.add_path(&stroke);

        let mut ren = RenderingScanlineAASolid::with_base(&mut ren_base);
        render_scanlines(&mut ras, &mut ren);
    }
    let mut ras = RasterizerScanline::new();
    let mut ren = RenderingScanlineAASolid::with_base(&mut ren_base);
    text(&mut ras, &mut ren, 29.0, 90.0, "Miter");
    text(&mut ras, &mut ren, 125.0, 90.0, "Round");
    text(&mut ras, &mut ren, 225.0, 90.0, "Bevel");

    ren_base.to_file("tests/std/tmp/line_join.png").unwrap();
    assert!(
        agrega::ppm::img_diff("tests/std/tmp/line_join.png", "tests/images/line_join.png").unwrap()
    );
}

fn text<T>(
    ras: &mut RasterizerScanline,
    ren: &mut RenderingScanlineAASolid<T>,
    x: f64,
    y: f64,
    txt: &str,
) where
    T: Pixel,
{
    let mut t = GsvText::new();
    t.size(12.0, 0.0);
    t.text(txt);
    t.start_point(x, y);
    t.flip(true);
    let mut stroke = Stroke::new(t);
    stroke.width(1.0);
    ras.add_path(&stroke);
    ren.color(Rgba8::new(0, 0, 0, 255));
    render_scanlines(ras, ren);
}
