#[test]
fn t24_outline_basic_render() {
    use agrega::{Pixfmt, Rgb8, Rgba8};
    use agrega::{RasterizerOutline, RendererPrimitives};
    let pix = Pixfmt::<Rgb8>::new(100, 100);
    let mut ren_base = agrega::RenderingBase::new(pix);
    ren_base.clear(Rgba8::new(255, 255, 255, 255));

    let mut ren = RendererPrimitives::with_base(&mut ren_base);
    ren.line_color(agrega::Rgba8::new(0, 0, 0, 255));

    let mut path = agrega::Path::new();
    path.move_to(10.0, 10.0);
    path.line_to(50.0, 90.0);
    path.line_to(90.0, 10.0);

    let mut ras = RasterizerOutline::with_primitive(&mut ren);
    ras.add_path(&path);
    ren_base.to_file("tests/std/tmp/primitive.png").unwrap();

    //assert!(agrega::ppm::img_diff("tests/std/tmp/primitive.png",
    //                           "tests/images/primitive.png").unwrap());
}
