use agrega::{
    bounding_rect, img_diff, render_all_paths, Path, Pixfmt, RasterizerScanline, Rectangle, Render,
    RenderingBase, RenderingScanlineAASolid, Rgb8, Stroke, Transform,
};

#[test]
fn lion_outline_width1() {
    let (w, h) = (400, 400);

    let (paths, colors) = super::parse_lion_reoriented_srgba();
    let pixf = Pixfmt::<Rgb8>::new(w, h);
    let mut ren_base = RenderingBase::new(pixf);
    ren_base.clear(Rgb8::white());
    let mut ren = RenderingScanlineAASolid::with_base(&mut ren_base);
    ren.color(Rgb8::new(255, 0, 0));

    let mut ras = RasterizerScanline::new();

    if paths.is_empty() {
        return;
    }
    let p = paths[0].vertices[0];
    let mut r = Rectangle::new(p.x, p.y, p.x, p.y);
    for p in &paths {
        if let Some(rp) = bounding_rect(p) {
            //eprintln!("dx,dy: {:?}", rp);
            r.expand_rect(&rp);
        }
    }
    //eprintln!("dx,dy: {:?}", r);
    let g_base_dx = (r.x2() - r.x1()) / 2.0;
    let g_base_dy = (r.y2() - r.y1()) / 2.0;
    let mut mtx = Transform::new();
    eprintln!("dx,dy: {} {}", -g_base_dx, -g_base_dy);
    eprintln!("dx,dy: {} {}", (w / 2) as f64, (h / 2) as f64);
    mtx.set_translate(-g_base_dx, -g_base_dy);
    mtx.set_translate((w / 2) as f64, (h / 2) as f64);
    //mtx.set_translate(0.0, 0.0);

    let t: Vec<Path> = paths.into_iter().map(|p| p.transformed(&mtx)).collect();
    println!("polygons: {}", t.len());

    let mut stroke: Vec<_> = t.into_iter().map(Stroke::new).collect();
    stroke.iter_mut().for_each(|p| p.width(1.0));
    render_all_paths(&mut ras, &mut ren, &stroke, &colors);

    ren.to_file("tests/std/tmp/lion_outline_width1.png").unwrap();
    assert!(img_diff(
        "tests/std/tmp/lion_outline_width1.png",
        "tests/images/lion_outline_width1.png"
    )
    .unwrap());
}
// compare -verbose -metric AE lion.ppm ./tests/lion.ppm diff.ppm
