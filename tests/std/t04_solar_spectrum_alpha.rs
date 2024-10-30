use agrega::{img_diff, AlphaMaskAdaptor, Gray8, Pixel, Pixfmt, Rgb8};

#[test]
#[cfg(any(feature = "std", all(feature = "no_std", feature = "alloc")))]
fn t04_solar_spectrum_alpha() {
    let mut pix = Pixfmt::<Rgb8>::new(320, 200);
    pix.clear();

    let mut alpha = Pixfmt::<Gray8>::new(320, 200);

    let w = pix.width();
    let h = pix.height();

    #[allow(clippy::needless_range_loop)]
    for i in 0..h {
        let v = (255 * i / h) as u8;
        alpha.copy_hline(0, i, w, Gray8::new(v));
    }

    let mut span = vec![Rgb8::white(); w];
    #[allow(clippy::needless_range_loop)]
    for i in 0..w {
        span[i] = Rgb8::from_wavelength_gamma(380.0 + 400.0 * i as f64 / w as f64, 0.8);
    }

    let mut mix = AlphaMaskAdaptor::new(pix, alpha);

    for i in 0..h {
        mix.blend_color_hspan(0, i, w, &span, 0);
    }
    mix.rgb.to_file("tests/std/tmp/agg_test_04.png").unwrap();

    assert!(img_diff("tests/std/tmp/agg_test_04.png", "tests/images/agg_test_04.png").unwrap(),);
}
