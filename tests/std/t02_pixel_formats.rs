use agrega::{img_diff, Pixel, Pixfmt, Rgb8};

fn draw_black_frame(pix: &mut Pixfmt<Rgb8>) {
    let w = pix.width();
    let h = pix.height();
    println!("w,h: {} {}", w, h);
    let black = Rgb8::black();
    for i in 0..h {
        pix.copy_pixel(0, i, black);
        pix.copy_pixel(w - 1, i, black);
    }
    for &k in [0, h - 1].iter() {
        for i in 0..w {
            pix.copy_pixel(i, k, black);
        }
    }
}

#[test]
fn t02_pixel_formats() {
    //let rbuf = RenderingBuffer::new(320, 220, 3);
    let mut pix = Pixfmt::<Rgb8>::new(320, 220);
    pix.clear();
    draw_black_frame(&mut pix);

    for i in 0..pix.height() / 2 {
        let c = Rgb8::new(127, 200, 98);
        pix.copy_pixel(i, i, c);
    }

    pix.to_file("tests/std/tmp/agg_test_02.png").unwrap();
    assert!(img_diff("tests/std/tmp/agg_test_02.png", "tests/images/agg_test_02.png").unwrap(),);
}
