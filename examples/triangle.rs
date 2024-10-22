use agrega::{
    render_scanlines, Pixfmt, RasterizerScanline, Render, RenderingBase, RenderingScanlineAASolid,
    Rgb8, Rgba8,
};

fn main() {
    // Create a blank image 10x10 pixels
    let pix = Pixfmt::<Rgb8>::new(100, 100);
    let mut ren_base = RenderingBase::new(pix);
    ren_base.clear(Rgba8::white());

    // Draw a polygon from (10, 10) - (50, 90) - (90, 10)
    let mut ras = RasterizerScanline::new();
    ras.move_to(10.0, 10.0);
    ras.line_to(50.0, 90.0);
    ras.line_to(90.0, 10.0);

    // Render the line to the image
    let mut ren = RenderingScanlineAASolid::with_base(&mut ren_base);
    ren.color(Rgba8::black());
    render_scanlines(&mut ras, &mut ren);

    // Save the image to a file
    ren_base.to_file("examples/out/little_black_triangle.png").unwrap();
}
