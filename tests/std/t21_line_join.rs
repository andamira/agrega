use super::text_h12 as text;
use agrega::{
    img_diff, render_scanlines, LineJoin, Path, Pixfmt, RasterizerScanline, RenderingBase,
    RenderingScanlineAASolid, Rgb8, Stroke,
};

#[test]
fn t21_line_join() {
    let pix = Pixfmt::<Rgb8>::new(300, 100);
    let mut ren_base = RenderingBase::new(pix);
    ren_base.clear(Rgb8::white());

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
    assert!(img_diff("tests/std/tmp/line_join.png", "tests/images/line_join.png").unwrap());
}
