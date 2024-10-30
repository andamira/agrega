use crate::{Pixel, PixelSource, Pixfmt, Rgb8, Rgba32, Rgba8, Rgba8pre, Srgba8};

#[test]
fn pixfmt_test() {
    let mut p = Pixfmt::<Rgb8>::new(10, 10);
    assert_eq!(p.rbuf.data.len(), 300);

    p.copy_pixel(0, 0, Rgb8::black());
    assert_eq!(p.get((0, 0)), Rgba8::black());

    assert_ne!(p.get((1, 0)), Rgba8::white());
    p.copy_pixel(1, 0, Rgb8::white());
    assert_eq!(p.get((1, 0)), Rgba8::white());

    let red = Rgba8::new(255, 0, 0, 128);
    p.copy_hline(0, 1, 10, red);
    for i in 0..10 {
        assert_eq!(p.get((i, 1)), Rgba8::new(255, 0, 0, 255));
    }
    let yellow = Srgba8::new(128, 255, 0, 128);
    p.copy_hline(0, 2, 10, yellow);
    for i in 0..10 {
        assert_eq!(p.get((i, 2)), Rgba8::new(55, 255, 0, 255));
    }
    let fuchsia = Rgba32::new(0.0, 1.0, 1.0, 0.5);
    p.copy_hline(0, 3, 10, fuchsia);
    for i in 0..10 {
        assert_eq!(p.get((i, 3)), Rgba8::new(0, 255, 255, 255));
    }
    p.clear();
    assert_eq!(p.get((0, 3)), Rgba8::new(255, 255, 255, 255));

    let red = Rgba8::new(255, 0, 0, 128);
    p.copy_vline(1, 0, 10, red);
    for i in 0..10 {
        assert_eq!(p.get((1, i)), Rgba8::new(255, 0, 0, 255));
    }
    let yellow = Srgba8::new(128, 255, 0, 128);
    p.copy_vline(2, 0, 10, yellow);
    for i in 0..10 {
        assert_eq!(p.get((2, i)), Rgba8::new(55, 255, 0, 255));
    }
    let fuchsia = Rgba32::new(0.0, 1.0, 1.0, 0.5);
    p.copy_vline(3, 0, 10, fuchsia);
    for i in 0..10 {
        assert_eq!(p.get((3, i)), Rgba8::new(0, 255, 255, 255));
    }

    p.clear();
    p.copy_pixel(11, 11, Rgb8::black());
    for i in 0..10 {
        for j in 0..10 {
            assert_eq!(p.get((i, j)), Rgba8::white());
        }
    }
    p.copy_hline(0, 0, 20, Rgb8::black());
    for i in 0..10 {
        assert_eq!(p.get((i, 0)), Rgba8::black());
    }
    p.copy_hline(5, 1, 20, Rgb8::black());
    for i in 5..10 {
        assert_eq!(p.get((i, 1)), Rgba8::black());
    }

    p.clear();
    p.copy_vline(0, 0, 20, Rgb8::black());
    for i in 0..10 {
        assert_eq!(p.get((0, i)), Rgba8::black());
    }

    p.clear();
    p.copy_vline(1, 5, 20, Rgb8::black());
    for i in 0..5 {
        assert_eq!(p.get((1, i)), Rgba8::white(), "pix({},{}): {:?}", 1, i, p.get((1, i)));
    }
    for i in 5..10 {
        assert_eq!(p.get((1, i)), Rgba8::black(), "pix({},{}): {:?}", 1, i, p.get((1, i)));
    }
    p.copy_vline(2, 3, 5, Rgb8::black());
    for i in 0..3 {
        assert_eq!(p.get((2, i)), Rgba8::white(), "pix({},{}): {:?}", 2, i, p.get((2, i)));
    }
    for i in 3..8 {
        assert_eq!(p.get((2, i)), Rgba8::black(), "pix({},{}): {:?}", 2, i, p.get((2, i)));
    }
    for i in 8..10 {
        assert_eq!(p.get((2, i)), Rgba8::white(), "pix({},{}): {:?}", 2, i, p.get((2, i)));
    }
}

#[test]
fn pixfmt_rgb8_test() {
    let mut pix = Pixfmt::<Rgb8>::new(1, 1);
    let black = Rgba8::black();
    let white = Rgba8::white();

    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, 255));
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 255); // Copy Pixel
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), white);

    let (alpha, beta, cover) = (255, 255, 0); // Do Nothing, No Coverage
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 0, 255); // Do Nothing, Transparent
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(128, 128, 128, 255));

    let (alpha, beta, cover) = (255, 128, 255); // Full Coverage, Alpha Color
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(128, 128, 128, 255));

    let (alpha, beta, cover) = (128, 128, 255); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::new(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::new(255, 255, 255, 255)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(127, 127, 127, 255));

    let (alpha, beta, cover) = (128, 128, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::new(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::new(255, 255, 255, 255)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(191, 191, 191, 255));
}

#[test]
fn pixfmt_rgba8_test() {
    let mut pix = Pixfmt::<Rgba8>::new(1, 1);
    let black = Rgba8::black();
    let white = Rgba8::white();

    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, 255));
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 255); // Copy Pixel
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), white);

    let (alpha, beta, cover) = (255, 255, 0); // Do Nothing, No Coverage
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 0, 255); // Do Nothing, Transparent
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(128, 128, 128, 255));

    let (alpha, beta, cover) = (255, 128, 255); // Full Coverage, Alpha Color
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(128, 128, 128, 255));

    let (alpha, beta, cover) = (128, 128, 255); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::new(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::new(255, 255, 255, 128));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(127, 127, 127, 192));

    let (alpha, beta, cover) = (128, 128, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::new(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::new(255, 255, 255, 128)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(191, 191, 191, 160));
}

#[test]
fn pixfmt_rgba8pre_test() {
    let mut pix = Pixfmt::<Rgba8pre>::new(1, 1);
    let black = Rgba8::black();
    let white = Rgba8::white();

    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, 255));
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 255); // Copy Pixel
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), white);

    let (alpha, beta, cover) = (255, 255, 0); // Do Nothing, No Coverage
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 0, 255); // Do Nothing, Transparent
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(128, 128, 128, 255));

    let (alpha, beta, cover) = (255, 128, 255); // Full Coverage, Alpha Color
    pix.copy_pixel(0, 0, Rgba8::new(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(255, 255, 255, 255));

    let (alpha, beta, cover) = (128, 128, 255); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::new(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::new(255, 255, 255, 128));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(127, 127, 127, 192));

    let (alpha, beta, cover) = (128, 128, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::new(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::new(255, 255, 255, 128)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::new(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::new(191, 191, 191, 160));
}
