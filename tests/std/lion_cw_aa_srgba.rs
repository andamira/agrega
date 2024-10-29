use agrega::{Path, Render, Transform};
use std::fs;

fn parse_lion() -> (Vec<agrega::Path>, Vec<agrega::Srgba8>) {
    let txt = fs::read_to_string("tests/std/assets/lion.txt").unwrap();
    let mut paths = vec![];
    let mut colors = vec![];
    let mut path = agrega::Path::new();
    //let mut color = agrega::Srgba8::black();
    let mut color = agrega::Srgba8::new(0, 0, 0, 255);
    let mut cmd = agrega::PathCommand::Stop;

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
            path = agrega::Path::new();
            let rgb = agrega::Rgba8::new(r, g, b, 255);
            color = agrega::Srgba8::from_rgb(rgb);
            //color =  agrega::Rgba8::new(r,g,b,255);
        } else {
            for val in v {
                if val == "M" {
                    cmd = agrega::PathCommand::MoveTo;
                } else if val == "L" {
                    cmd = agrega::PathCommand::LineTo;
                } else {
                    let pts: Vec<_> = val.split(",").map(|x| x.parse::<f64>().unwrap()).collect();

                    match cmd {
                        agrega::PathCommand::LineTo => path.line_to(pts[0], pts[1]),
                        agrega::PathCommand::MoveTo => {
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
    paths
        .iter_mut()
        .for_each(|p| p.arrange_orientations(agrega::PathOrientation::Clockwise));
    (paths, colors)
}

#[test]
fn lion_cw_aa_srgba() {
    let (w, h) = (400, 400);

    let (paths, colors) = parse_lion();
    let pixf = agrega::Pixfmt::<agrega::Rgb8>::new(w, h);
    let mut ren_base = agrega::RenderingBase::new(pixf);
    //ren_base.clear( agrega::Srgba8::new([255, 255, 255, 255]) );
    ren_base.clear(agrega::Rgba8::new(255, 255, 255, 255));
    let mut ren = agrega::RenderingScanlineAASolid::with_base(&mut ren_base);
    //ren.color( &agrega::Srgba8::new([255,0,0,255]) );
    ren.color(agrega::Rgba8::new(255, 0, 0, 255));

    let mut ras = agrega::RasterizerScanline::new();

    if paths.len() == 0 {
        return;
    }
    let p = paths[0].vertices[0];
    let mut r = agrega::Rectangle::new(p.x, p.y, p.x, p.y);
    for p in &paths {
        if let Some(rp) = agrega::bounding_rect(p) {
            //eprintln!("dx,dy: {:?}", rp);
            r.expand_rect(&rp);
        }
    }
    //eprintln!("dx,dy: {:?}", r);
    let g_base_dx = (r.x2() - r.x1()) / 2.0;
    let g_base_dy = (r.y2() - r.y1()) / 2.0;
    let mut mtx = Transform::new();
    //eprintln!("dx,dy: {} {}", -g_base_dx, -g_base_dy);
    //eprintln!("dx,dy: {} {}", (w/2) as f64, (h/2) as f64);
    mtx.translate(-g_base_dx, -g_base_dy);
    mtx.translate((w / 2) as f64, (h / 2) as f64);
    //mtx.translate(0.0, 0.0);
    let t: Vec<Path> = paths.into_iter().map(|p| p.transformed(&mtx)).collect();
    println!("polygons: {}", t.len());

    agrega::render_all_paths(&mut ras, &mut ren, &t, &colors);

    ren.to_file("tests/std/tmp/lion_cw_aa_srgba.png").unwrap();

    assert_eq!(
        agrega::ppm::img_diff(
            "tests/std/tmp/lion_cw_aa_srgba.png",
            "tests/images/lion_cw_aa_srgba.png"
        )
        .unwrap(),
        true
    );
}
// compare -verbose -metric AE lion.ppm ./tests/std/lion.ppm blarg.ppm
