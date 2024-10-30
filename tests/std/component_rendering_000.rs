use agrega::{
    img_diff, render_scanlines_aa_solid, Ellipse, Gray8, Pixfmt, PixfmtAlphaBlend,
    RasterizerScanline, RenderingBase, Rgb8,
};

type PixRgb8 = Pixfmt<Rgb8>;

#[test]
fn component_rendering_000() {
    let alpha = 0;
    let (w, h) = (320, 320);

    let pixf = Pixfmt::<Rgb8>::new(w, h);
    let mut ren_base = RenderingBase::new(pixf);
    ren_base.clear(Rgb8::white());
    let g8 = Gray8::new_with_alpha(0, alpha);

    let w2 = (w / 2) as f64;
    let h2 = (h / 2) as f64;
    let er = Ellipse::new(w2 - 0.87 * 50.0, h2 - 0.5 * 50., 100., 100., 100);
    let eg = Ellipse::new(w2 + 0.87 * 50.0, h2 - 0.5 * 50., 100., 100., 100);
    let eb = Ellipse::new(w2, h2 + 50., 100., 100., 100);

    let mut ras = RasterizerScanline::new();

    {
        let pfr = PixfmtAlphaBlend::<PixRgb8, Gray8>::new(&mut ren_base, 0);
        let mut rbr = RenderingBase::new(pfr);
        ras.add_path(&er);
        render_scanlines_aa_solid(&mut ras, &mut rbr, g8);
    }
    {
        let pfg = PixfmtAlphaBlend::<PixRgb8, Gray8>::new(&mut ren_base, 1);
        let mut rbg = RenderingBase::new(pfg);
        ras.add_path(&eg);
        render_scanlines_aa_solid(&mut ras, &mut rbg, g8);
    }
    {
        let pfb = PixfmtAlphaBlend::<PixRgb8, Gray8>::new(&mut ren_base, 2);
        let mut rbb = RenderingBase::new(pfb);
        ras.add_path(&eb);
        render_scanlines_aa_solid(&mut ras, &mut rbb, g8);
    }

    ren_base.to_file("tests/std/tmp/component_rendering_000.png").unwrap();
    assert!(img_diff(
        "tests/std/tmp/component_rendering_000.png",
        "tests/images/component_rendering_000.png"
    )
    .unwrap(),)
}
