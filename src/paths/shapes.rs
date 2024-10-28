// agrega::paths::shapes

use super::{Path, PathCommand, VertexSource, Vertex};
use devela::iif;
use core::f64::consts::PI;

/// Represents an ellipse shape with a center, radii, scale, and vertex approximation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ellipse {
    x: f64,
    y: f64,
    rx: f64,
    ry: f64,
    scale: f64,
    num: usize,
    //step: usize,
    cw: bool,
    path: Path,
}

impl VertexSource for Ellipse {
    fn xconvert(&self) -> Vec<Vertex<f64>> {
        self.path.vertices.clone()
    }
}

impl Ellipse {
    /// Creates a new ellipse centered at `(x, y)` with radii `rx` and `ry`,
    /// and `num` vertices.
    pub fn new(x: f64, y: f64, rx: f64, ry: f64, num: usize) -> Self {
        let mut s = Self {
            x: 0.0,
            y: 0.0,
            rx: 1.0,
            ry: 1.0,
            scale: 1.0,
            num: 4,
            cw: false,
            path: Path::new(),
        };
        s.x = x;
        s.y = y;
        s.rx = rx;
        s.ry = ry;
        s.num = num;
        s.cw = false;
        iif![num == 0; s.calc_num_steps()];
        s.calc();
        s
    }

    /// Calculates the number of steps required for a smooth ellipse based on the current scale.
    pub fn calc_num_steps(&mut self) {
        let ra = (self.rx.abs() + self.ry.abs()) / 2.0;
        let da = (ra / (ra + 0.125 / self.scale)).acos() * 2.0;
        self.num = (2.0 * PI / da).round() as usize;
    }

    /// Computes the vertices for the ellipse, populating its `path`.
    pub fn calc(&mut self) {
        self.path = Path::new();
        for i in 0..self.num {
            let angle = i as f64 / self.num as f64 * 2.0 * PI;
            let angle = if self.cw { 2.0 * PI - angle } else { angle };
            let x = self.x + angle.cos() * self.rx;
            let y = self.y + angle.sin() * self.ry;
            let v = iif![i == 0; Vertex::move_to(x, y); Vertex::line_to(x, y)];
            self.path.vertices.push(v);
        }
        let v = self.path.vertices[0];
        self.path.vertices.push(Vertex::close_polygon(v.x, v.y));
    }

    /// Returns the path.
    #[inline] #[must_use] #[rustfmt::skip]
    pub fn into_path(self) -> Path { self.path }
}

/// Represents a rounded rectangle with specified corner radii and vertices.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoundedRect {
    x: [f64; 2],
    y: [f64; 2],
    rx: [f64; 4],
    ry: [f64; 4],
    path: Path,
}

impl VertexSource for RoundedRect {
    #[inline]
    fn xconvert(&self) -> Vec<Vertex<f64>> {
        self.path.vertices.clone()
    }
}

impl RoundedRect {
    /// Creates a new rounded rectangle defined by corners `(x1, y1)` to `(x2, y2)`
    /// with uniform corner radius `r`.
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, r: f64) -> Self {
        let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
        let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
        Self { x: [x1, x2], y: [y1, y2], rx: [r; 4], ry: [r; 4], path: Path::new() }
    }

    /// Calculates the vertices for the rounded rectangle, including curved corners.
    pub fn calc(&mut self) {
        let vx = [1.0, -1.0, -1.0, 1.0];
        let vy = [1.0, 1.0, -1.0, -1.0];
        let x = [self.x[0], self.x[1], self.x[1], self.x[0]];
        let y = [self.y[0], self.y[0], self.y[1], self.y[1]];
        let a = [PI, PI + PI * 0.5, 0.0, 0.5 * PI];
        let b = [PI + PI * 0.5, 0.0, PI * 0.5, PI];
        for i in 0..4 {
            let arc = Arc::init(
                x[i] + self.rx[i] * vx[i],
                y[i] + self.ry[i] * vy[i],
                self.rx[i],
                self.ry[i],
                a[i],
                b[i],
            );
            let mut verts = arc.xconvert();
            for vi in verts.iter_mut() {
                vi.cmd = PathCommand::LineTo;
            }
            self.path.vertices.extend(verts);
        }
        if let Some(first) = self.path.vertices.first_mut() {
            first.cmd = PathCommand::MoveTo;
        }
        let first = self.path.vertices[0];
        self.path.vertices.push(Vertex::close_polygon(first.x, first.y));
    }

    /// Normalizes the corner radii based on the dimensions of the rectangle.
    pub fn normalize_radius(&mut self) {
        let dx = (self.y[1] - self.y[0]).abs();
        let dy = (self.x[1] - self.x[0]).abs();

        let mut k = 1.0f64;
        let ts = [
            dx / (self.rx[0] + self.rx[1]),
            dx / (self.rx[2] + self.rx[3]),
            dy / (self.rx[0] + self.rx[1]),
            dy / (self.rx[2] + self.rx[3]),
        ];
        for &t in ts.iter() {
            iif![t < k; k = t];
        }
        if k < 1.0 {
            for v in &mut self.rx {
                *v *= k;
            }
            for v in &mut self.ry {
                *v *= k;
            }
        }
    }

    /// Returns the path.
    #[inline] #[must_use] #[rustfmt::skip]
    pub fn into_path(self) -> Path { self.path }
}

/// Represents an arc defined by center, radii, angles, and direction.
#[derive(Clone, Debug, PartialEq)]
pub struct Arc {
    x: f64,
    y: f64,
    rx: f64,
    ry: f64,
    start: f64,
    end: f64,
    scale: f64,
    ccw: bool,
    da: f64,
    path: Path,
}

impl VertexSource for Arc {
    #[inline]
    fn xconvert(&self) -> Vec<Vertex<f64>> {
        self.path.vertices.clone()
    }
}

impl Arc {
    /// Initializes a new arc with center `(x, y)`, radii `rx`, `ry`, and angles `a1` to `a2`.
    pub fn init(x: f64, y: f64, rx: f64, ry: f64, a1: f64, a2: f64) -> Self {
        let mut a = Self {
            x,
            y,
            rx,
            ry,
            scale: 1.0,
            ccw: true,
            start: 0.0,
            end: 0.0,
            da: 0.0,
            path: Path::new(),
        };
        a.normalize(a1, a2, true);
        a.calc();
        a
    }

    /// Computes the vertices along the arc based on the specified start and end angles.
    pub fn calc(&mut self) {
        let mut angle: Vec<_> = (0..)
            .map(|i| self.start + self.da * f64::from(i))
            .take_while(|x| {
                if self.da > 0.0 {
                    x < &self.end
                } else {
                    x > &self.end
                }
            })
            .collect();
        angle.push(self.end);
        for a in &angle {
            let x = self.x + a.cos() * self.rx;
            let y = self.y + a.sin() * self.ry;
            self.path.vertices.push(Vertex::line_to(x, y));
        }
        if let Some(first) = self.path.vertices.first_mut() {
            first.cmd = PathCommand::MoveTo;
        }
        if let Some(last) = self.path.vertices.last_mut() {
            last.cmd = PathCommand::Close;
        }
        //repeat_last_point(&mut self.vertices);
    }

    /// Normalizes the start and end angles based on direction (clockwise or counterclockwise).
    pub fn normalize(&mut self, a1: f64, a2: f64, ccw: bool) {
        let ra = (self.rx.abs() + self.ry.abs()) / 2.0;
        self.da = (ra / (ra + 0.125 / self.scale)).acos() * 2.0;
        let mut a1 = a1;
        let mut a2 = a2;
        if ccw {
            while a2 < a1 {
                a2 += 2.0 * PI;
            }
        } else {
            while a1 < a2 {
                a1 += 2.0 * PI;
            }
            self.da = -self.da;
        }
        self.ccw = ccw;
        self.start = a1;
        self.end = a2;
    }

    /// Returns the path.
    #[inline] #[must_use] #[rustfmt::skip]
    pub fn into_path(self) -> Path { self.path }
}
