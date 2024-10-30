use agrega::{img_diff, Path, Pixfmt, RasterizerOutline, RendererPrimitives, RenderingBase, Rgb8};

#[test]
fn t24_outline_basic_render() {
    let pix = Pixfmt::<Rgb8>::new(100, 100);
    let mut ren_base = RenderingBase::new(pix);
    ren_base.clear(Rgb8::white());

    let mut ren = RendererPrimitives::with_base(&mut ren_base);
    ren.line_color(Rgb8::black());

    let mut path = Path::new();
    path.move_to(10.0, 10.0);
    path.line_to(50.0, 90.0);
    path.line_to(90.0, 10.0);

    let mut ras = RasterizerOutline::with_primitive(&mut ren);
    ras.add_path(&path);
    ren_base.to_file("tests/std/tmp/primitive.png").unwrap();

    assert!(img_diff("tests/std/tmp/primitive.png", "tests/images/primitive.png").unwrap());
}
