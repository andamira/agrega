use super::text_h12 as text;
use agrega::{
    img_diff, render_scanlines, InnerJoin, Path, Pixfmt, RasterizerScanline, RenderingBase,
    RenderingScanlineAASolid, Rgb8, Stroke,
};

#[test]
fn t22_inner_join() {
    let pix = Pixfmt::<Rgb8>::new(400, 100);
    let mut ren_base = RenderingBase::new(pix);
    ren_base.clear(Rgb8::white());

    let joins = [InnerJoin::Miter, InnerJoin::Round, InnerJoin::Bevel, InnerJoin::Jag];
    for (i, join) in joins.iter().enumerate() {
        let dx = 100.0 * i as f64;
        let mut path = Path::new();
        path.move_to(10.0 + dx, 70.0);
        path.line_to(50.0 + dx, 30.0);
        path.line_to(90.0 + dx, 70.0);

        let mut stroke = Stroke::new(path);
        stroke.width(25.0);
        stroke.inner_join(*join);

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
    text(&mut ras, &mut ren, 332.0, 90.0, "Jag");

    ren_base.to_file("tests/std/tmp/inner_join.png").unwrap();
    assert!(img_diff("tests/std/tmp/inner_join.png", "tests/images/inner_join.png").unwrap());
}
