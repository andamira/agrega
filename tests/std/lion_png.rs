use std::fs;

use agrega::{Path, PathCommand, Render, RenderingBase, Rgba8, Transform};

fn parse_lion() -> (Vec<Path>, Vec<Rgba8>) {
    let txt = fs::read_to_string("tests/std/assets/lion.txt").unwrap();
    let mut paths = vec![];
    let mut colors = vec![];
    let mut path = Path::new();
    let mut color = Rgba8::black();
    let mut cmd = PathCommand::Stop;

    for line in txt.lines() {
        let v: Vec<_> = line.split_whitespace().collect();
        if v.len() == 1 {
            let n = 0;
            let hex = v[0];
            let r = u8::from_str_radix(&hex[n + 0..n + 2], 16).unwrap();
            let g = u8::from_str_radix(&hex[n + 2..n + 4], 16).unwrap();
            let b = u8::from_str_radix(&hex[n + 4..n + 6], 16).unwrap();
            if path.vertices.len() > 0 {
                path.close_polygon();
                paths.push(path);
                colors.push(color);
            }
            path = Path::new();
            color = Rgba8::new(r, g, b, 255);
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
    if path.vertices.len() > 0 {
        colors.push(color);
        path.close_polygon();
        paths.push(path);
    }
    assert_eq!(paths.len(), colors.len());
    (paths, colors)
}

#[test]
fn lion_png() {
    let (w, h) = (400, 400);

    let (paths, colors) = parse_lion();
    let pixf = agrega::Pixfmt::<agrega::Rgb8>::new(w, h);
    let mut ren_base = RenderingBase::new(pixf);
    ren_base.clear(Rgba8::new(255, 255, 255, 255));
    let mut ren = agrega::RenderingScanlineBinSolid::with_base(&mut ren_base);
    ren.color(Rgba8::new(255, 0, 0, 255));

    let mut ras = agrega::RasterizerScanline::new();

    if paths.len() == 0 {
        return;
    }
    let p = paths[0].vertices[0];
    let mut r = agrega::Rectangle::new(p.x, p.y, p.x, p.y);
    for p in &paths {
        if let Some(rp) = agrega::bounding_rect(p) {
            r.expand_rect(&rp);
        }
    }
    let g_base_dx = (r.x2() - r.x1()) / 2.0;
    let g_base_dy = (r.y2() - r.y1()) / 2.0;

    let mut mtx = Transform::new();
    mtx.translate(-g_base_dx, -g_base_dy);
    mtx.translate((w / 2) as f64, (h / 2) as f64);
    let t: Vec<Path> = paths.into_iter().map(|p| p.transformed(&mtx)).collect();

    agrega::render_all_paths(&mut ras, &mut ren, &t, &colors);

    ren.to_file("tests/std/tmp/lion.png").unwrap();

    if !agrega::ppm::img_diff("tests/std/tmp/lion.png", "tests/images/lion.png").unwrap() {
        panic!("PNG Images differ");
    }
}
