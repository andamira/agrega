#[test]
fn t05_solar_spectrum_alpha() {
    use agrega::Pixel;

    let mut pix = agrega::Pixfmt::<agrega::Rgb8>::new(320, 200);
    pix.clear();
    pix.fill(agrega::Rgb8::black());
    let mut alpha = agrega::Pixfmt::<agrega::Gray8>::new(320, 200);

    let w = pix.width();
    let h = pix.height();

    for i in 0..h {
        let v = (255 * i / h) as u8;
        alpha.copy_hline(0, i, w, agrega::Gray8::new(v));
    }

    let mut span = vec![agrega::Rgb8::white(); w];
    for i in 0..w {
        span[i] = agrega::Rgb8::from_wavelength_gamma(380.0 + 400.0 * i as f64 / w as f64, 0.8);
    }

    let mut mix = agrega::AlphaMaskAdaptor::new(pix, alpha);

    for i in 0..h {
        mix.blend_color_hspan(0, i, w, &span, 0);
    }
    mix.rgb.to_file("tests/std/tmp/agg_test_05.png").unwrap();

    assert!(
        agrega::ppm::img_diff("tests/std/tmp/agg_test_05.png", "tests/images/agg_test_05.png")
            .unwrap(),
        "{}",
        true
    );
}
