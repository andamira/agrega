use agrega::{
    img_diff, DrawOutline, Path, Pixfmt, RasterizerOutlineAA, RendererOutlineAA, RenderingBase,
    Rgb8,
};

#[test]
fn t20_outline_render() {
    let pix = Pixfmt::<Rgb8>::new(100, 100);
    let mut ren_base = RenderingBase::new(pix);
    ren_base.clear(Rgb8::white());
    let mut ren = RendererOutlineAA::with_base(&mut ren_base);
    ren.color(Rgb8::black());
    ren.width(20.0);

    let mut path = Path::new();
    path.move_to(10.0, 10.0);
    path.line_to(50.0, 90.0);
    path.line_to(90.0, 10.0);

    let mut ras = RasterizerOutlineAA::with_renderer(&mut ren);
    ras.round_cap(true);
    ras.add_path(&path);
    ren_base.to_file("tests/std/tmp/outline_aa.png").unwrap();

    assert!(img_diff("tests/std/tmp/outline_aa.png", "tests/images/outline_aa.png").unwrap());
}
