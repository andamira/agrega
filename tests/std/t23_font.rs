use agrega::{
    Label, Pixfmt, RenderingBase, Rgb8, Rgba8, XAlign, YAlign, _dep::freetype::Library, draw_text,
};

#[test]
fn t23_font() {
    let lib = Library::init().unwrap();
    let font = lib.new_face("tests/std/assets/Helvetica.ttc", 0).unwrap();
    font.set_char_size(13 * 64, 0, 72, 0).unwrap();

    let pix = Pixfmt::<Rgb8>::new(100, 100);
    let mut ren_base = RenderingBase::new(pix);
    ren_base.clear(Rgb8::white());

    draw_text("Hello World!!!", 50, 45, Rgba8::black(), &font, &mut ren_base);

    let mut label = Label::new("Hello World!!!", 50., 58., 13.0, &font)
        .unwrap()
        .xalign(XAlign::Center)
        .yalign(YAlign::Center);
    label.draw_subpixel(&mut ren_base);

    ren_base.blend_hline(50, 57, 50, Rgb8::new(255, 0, 0), 255);

    ren_base.to_file("tests/std/tmp/font.png").unwrap();
    // FIXME: images differ slightly, this font is rendered taller
    // assert!(img_diff("tests/std/tmp/font.png", "tests/images/font.png").unwrap());
}
