use agrega::{
    render_scanlines, GsvText, Path, PathCommand, PathOrientation, Pixel, RasterizerScanline,
    Render, RenderingScanlineAASolid, Rgb8, Srgba8, Stroke,
};
use std::fs;

/// 8 height text, 0.7 stroke_width
#[inline] #[rustfmt::skip]
pub(super) fn text_h8_sw07<T: Pixel>(
    ras: &mut RasterizerScanline, ren: &mut RenderingScanlineAASolid<T>,
    x: f64, y: f64, txt: &str,
) { text(ras, ren, x, y, txt, 8.0, 0.0, 0.7) }

/// 12 height text
#[inline] #[rustfmt::skip]
pub(super) fn text_h12<T: Pixel>(
    ras: &mut RasterizerScanline, ren: &mut RenderingScanlineAASolid<T>,
    x: f64, y: f64, txt: &str,
) { text(ras, ren, x, y, txt, 12.0, 0.0, 1.0) }
/// text with custom height, width, stroke_width
#[rustfmt::skip]
#[allow(clippy::too_many_arguments)]
pub(super) fn text<T: Pixel>(
    ras: &mut RasterizerScanline, ren: &mut RenderingScanlineAASolid<T>,
    x: f64, y: f64, txt: &str, height: f64, width: f64, stroke_width: f64,
) {
    let mut t = GsvText::new();
    t.size(height, width);
    t.text(txt);
    t.start_point(x, y);
    t.flip(true);
    let mut stroke = Stroke::new(t);
    stroke.width(stroke_width);
    ras.add_path(&stroke);
    ren.color(Rgb8::black());
    render_scanlines(ras, ren);
}

#[inline]
pub(super) fn parse_lion_reoriented() -> (Vec<Path>, Vec<Rgb8>) {
    let (mut paths, colors) = parse_lion();
    paths
        .iter_mut()
        .for_each(|p| p.arrange_orientations(PathOrientation::Clockwise));
    (paths, colors)
}

pub(super) fn parse_lion() -> (Vec<Path>, Vec<Rgb8>) {
    let txt = fs::read_to_string("tests/std/assets/lion.txt").unwrap();
    let mut paths = vec![];
    let mut colors = vec![];
    let mut path = Path::new();
    let mut color = Rgb8::black();
    let mut cmd = PathCommand::Stop;

    for line in txt.lines() {
        let v: Vec<_> = line.split_whitespace().collect();
        if v.len() == 1 {
            let n = 0;
            let hex = v[0];
            #[allow(clippy::identity_op)]
            let r = u8::from_str_radix(&hex[n + 0..n + 2], 16).unwrap();
            let g = u8::from_str_radix(&hex[n + 2..n + 4], 16).unwrap();
            let b = u8::from_str_radix(&hex[n + 4..n + 6], 16).unwrap();
            if !path.vertices.is_empty() {
                path.close_polygon();
                paths.push(path);
                colors.push(color);
            }
            path = Path::new();
            color = Rgb8::new(r, g, b);
        } else {
            for val in v {
                if val == "M" {
                    cmd = PathCommand::MoveTo;
                } else if val == "L" {
                    cmd = PathCommand::LineTo;
                } else {
                    let pts: Vec<_> = val.split(",").map(|x| x.parse::<f64>().unwrap()).collect();

                    match cmd {
                        PathCommand::LineTo => path.line_to(pts[0], pts[1]),
                        PathCommand::MoveTo => {
                            path.close_polygon();
                            path.move_to(pts[0], pts[1]);
                        }
                        _ => unreachable!("oh no !!!"),
                    }
                }
            }
        }
    }
    if !path.vertices.is_empty() {
        colors.push(color);
        path.close_polygon();
        paths.push(path);
    }
    assert_eq!(paths.len(), colors.len());
    (paths, colors)
}

// TODO: unify with previous
pub(super) fn parse_lion_reoriented_srgba() -> (Vec<Path>, Vec<Srgba8>) {
    let txt = fs::read_to_string("tests/std/assets/lion.txt").unwrap();
    let mut paths = vec![];
    let mut colors = vec![];
    let mut path = Path::new();
    let mut color = Srgba8::black();
    let mut cmd = PathCommand::Stop;

    for line in txt.lines() {
        let v: Vec<_> = line.split_whitespace().collect();
        if v.len() == 1 {
            let n = 0;
            let hex = v[0];
            #[allow(clippy::identity_op)]
            let r = u8::from_str_radix(&hex[n + 0..n + 2], 16).unwrap();
            let g = u8::from_str_radix(&hex[n + 2..n + 4], 16).unwrap();
            let b = u8::from_str_radix(&hex[n + 4..n + 6], 16).unwrap();
            if !path.vertices.is_empty() {
                path.close_polygon();
                paths.push(path);
                colors.push(color);
            }
            path = Path::new();
            let rgb = Rgb8::new(r, g, b);
            color = Srgba8::from_rgb(rgb);
            //color =  Rgba8::new(r,g,b,255);
        } else {
            for val in v {
                if val == "M" {
                    cmd = PathCommand::MoveTo;
                } else if val == "L" {
                    cmd = PathCommand::LineTo;
                } else {
                    let pts: Vec<_> = val.split(",").map(|x| x.parse::<f64>().unwrap()).collect();

                    match cmd {
                        PathCommand::LineTo => path.line_to(pts[0], pts[1]),
                        PathCommand::MoveTo => {
                            path.close_polygon();
                            path.move_to(pts[0], pts[1]);
                        }
                        _ => unreachable!("oh no !!!"),
                    }
                }
            }
        }
    }
    if !path.vertices.is_empty() {
        colors.push(color);
        path.close_polygon();
        paths.push(path);
    }
    assert_eq!(paths.len(), colors.len());
    paths
        .iter_mut()
        .for_each(|p| p.arrange_orientations(PathOrientation::Clockwise));
    (paths, colors)
}
