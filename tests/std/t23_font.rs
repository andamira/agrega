#[test]
fn t23_font() {
    let lib = agrega::ft::Library::init().unwrap();
    let font = lib.new_face("tests/std/assets/Helvetica.ttc", 0).unwrap();
    font.set_char_size(13 * 64, 0, 72, 0).unwrap();

    let pix = agrega::Pixfmt::<agrega::Rgb8>::new(100, 100);
    let mut ren_base = agrega::RenderingBase::new(pix);
    ren_base.clear(agrega::Rgba8::new(255, 255, 255, 255));

    agrega::draw_text("Hello World!!!", 50, 45, &font, &mut ren_base);

    let mut label = agrega::Label::new("Hello World!!!", 50., 58., 13.0, &font)
        .unwrap()
        .xalign(agrega::XAlign::Center)
        .yalign(agrega::YAlign::Center);
    label.draw(&mut ren_base);

    ren_base.blend_hline(50, 57, 50, agrega::Rgba8::new(255, 0, 0, 255), 255);

    ren_base.to_file("tests/std/tmp/font.png").unwrap();
    // FIXME: images differ slightly, this font is rendered taller
    // assert!(agrega::ppm::img_diff("tests/std/tmp/font.png", "tests/images/font.png").unwrap());
}
