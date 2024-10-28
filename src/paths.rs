// agrega::paths

use crate::{Rectangle, Transform, VertexSource};
use alloc::{vec, vec::Vec};
use core::f64::consts::PI;
use devela::iif;
#[allow(unused_imports)]
use devela::ExtFloat;

/// Commands for path drawing behavior.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PathCommand {
    /// Ends the path or subpath without any drawing.
    Stop,

    /// Moves the cursor to a new point `(x, y)` without drawing. (default)
    #[default]
    MoveTo,

    /// Draws a line from the current cursor position to (x, y).
    LineTo,

    /// Closes the current path or subpath by connecting the last point to the first.
    Close,
    //Curve3,
    //Curve4,
    //CurveN,
    //Catrom,
    //UBSpline,
    //EndPoly,
}

/// A vertex in the path with coordinates `(x, y)` and an associated [`PathCommand`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vertex<T> {
    pub x: T,
    pub y: T,
    pub cmd: PathCommand,
}

impl<T> Vertex<T> {
    /// Creates a new vertex with a specific `command`.
    #[inline]
    pub fn new(x: T, y: T, command: PathCommand) -> Self {
        Self { x, y, cmd: command }
    }
    /// Creates a vertex at `(x, y)` with `Stop` command.
    #[inline]
    pub fn xy(x: T, y: T) -> Self {
        Self { x, y, cmd: PathCommand::Stop }
    }
    /// Moves the cursor to `(x, y)` without drawing.
    #[inline]
    pub fn move_to(x: T, y: T) -> Self {
        Self { x, y, cmd: PathCommand::MoveTo }
    }
    /// Draws a line to `(x, y)` from the current position.
    #[inline]
    pub fn line_to(x: T, y: T) -> Self {
        Self { x, y, cmd: PathCommand::LineTo }
    }
    /// Closes the current path by connecting back to the start.
    #[inline]
    pub fn close_polygon(x: T, y: T) -> Self {
        Self { x, y, cmd: PathCommand::Close }
    }
}

/// Compute length between two points.
#[inline]
pub fn len(a: &Vertex<f64>, b: &Vertex<f64>) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

/// Compute cross product of three points
///
/// Returns the z-value of the 2D points, positive is counter-clockwise
/// negative is clockwise (or the ordering of the basis)
///
/// Because the input are 2D, this assumes the z-value is 0
/// the value is the length and direction of the cross product in the
/// z direction, or k-hat
pub const fn cross(p1: &Vertex<f64>, p2: &Vertex<f64>, p: &Vertex<f64>) -> f64 {
    (p.x - p2.x) * (p2.y - p1.y) - (p.y - p2.y) * (p2.x - p1.x)
}

/// Represents a path of connected vertices, each with an associated command.
///
/// The `Path` struct contains a list of vertices that together form shapes or
/// paths, where each vertex defines a position and a drawing command.
//
//  typedef path_base<vertex_block_storage<double> > path_storage;
#[derive(Debug, Default)]
pub struct Path {
    pub vertices: Vec<Vertex<f64>>,
}

impl VertexSource for Path {
    /// Converts the path vertices into a vector of `Vertex<f64>` for processing.
    fn xconvert(&self) -> Vec<Vertex<f64>> {
        self.vertices.clone()
    }
}

impl Path {
    /// Creates a new, empty path.
    #[inline]
    pub fn new() -> Self {
        Self { vertices: vec![] }
    }
    /// Clears all vertices in the path, removing any shapes or paths.
    #[inline]
    pub fn remove_all(&mut self) {
        self.vertices.clear();
    }
    /// Starts a new subpath at the given coordinates `(x, y)` without drawing.
    #[inline]
    pub fn move_to(&mut self, x: f64, y: f64) {
        self.vertices.push(Vertex::move_to(x, y));
    }
    /// Draws a line from the current position to the given coordinates `(x, y)`.
    #[inline]
    pub fn line_to(&mut self, x: f64, y: f64) {
        self.vertices.push(Vertex::line_to(x, y));
    }
    /// Closes the current polygon, connecting the last point to the starting point.
    pub fn close_polygon(&mut self) {
        iif![self.vertices.is_empty(); return];
        let n = self.vertices.len();
        let last = self.vertices[n - 1];
        if last.cmd == PathCommand::LineTo {
            self.vertices.push(Vertex::close_polygon(last.x, last.y));
        }
    }
    /// Adjusts the orientation of all polygons in the path to the specified direction.
    #[inline]
    pub fn arrange_orientations(&mut self, dir: PathOrientation) {
        arrange_orientations(self, dir);
    }

    /// Applies the transformation to each vertex in the path in place.
    pub fn transform(&mut self, trans: &Transform) {
        for vertex in &mut self.vertices {
            let (x, y) = trans.transform(vertex.x, vertex.y);
            vertex.x = x;
            vertex.y = y;
        }
    }

    /// Returns a new path with the transformation applied to each vertex,
    /// leaving the original path unaltered.
    #[must_use]
    pub fn transformed(&self, trans: &Transform) -> Path {
        let transformed_vertices = self
            .vertices
            .iter()
            .map(|v| {
                let (x, y) = trans.transform(v.x, v.y);
                Vertex::new(x, y, v.cmd)
            })
            .collect();

        Path { vertices: transformed_vertices }
    }
}

/// Represents the orientation of a polygon path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PathOrientation {
    Clockwise,
    CounterClockwise,
}

/// Split Path into Individual Segments at MoveTo Boundaries.
pub fn split(path: &[Vertex<f64>]) -> Vec<(usize, usize)> {
    let (mut start, mut end) = (None, None);
    let mut pairs = vec![];
    for (i, v) in path.iter().enumerate() {
        match (start, end) {
            (None, None) => match v.cmd {
                PathCommand::MoveTo => {
                    start = Some(i);
                }
                PathCommand::LineTo | PathCommand::Close | PathCommand::Stop => {}
            },
            (Some(_), None) => match v.cmd {
                PathCommand::MoveTo => {
                    start = Some(i);
                }
                PathCommand::LineTo => {
                    end = Some(i);
                }
                PathCommand::Close | PathCommand::Stop => end = Some(i),
            },
            (Some(s), Some(e)) => match v.cmd {
                PathCommand::MoveTo => {
                    pairs.push((s, e));
                    start = Some(i);
                    end = None;
                }
                PathCommand::LineTo | PathCommand::Close | PathCommand::Stop => end = Some(i),
            },
            (None, Some(_)) => unreachable!("oh on bad state!"),
        }
    }
    iif![let (Some(s), Some(e)) = (start, end); pairs.push((s, e))];
    pairs
}

// Adjusts the orientation of all polygons in the path to match the specified direction.
//
// This function detects the orientation of each polygon in the path and
// inverts any that do not match the given `PathOrientation`.
fn arrange_orientations(path: &mut Path, dir: PathOrientation) {
    let pairs = split(&path.vertices);
    for (s, e) in pairs {
        let pdir = perceive_polygon_orientation(&path.vertices[s..=e]);
        iif![pdir != dir; invert_polygon(&mut path.vertices[s..=e])];
    }
}

/// Inverts the vertex order of a polygon, effectively reversing its orientation.
///
/// This function reverses the order of vertices in a polygon and adjusts the
/// starting and ending commands accordingly to maintain path integrity.
pub fn invert_polygon(v: &mut [Vertex<f64>]) {
    let n = v.len();
    v.reverse();
    let tmp = v[0].cmd;
    v[0].cmd = v[n - 1].cmd;
    v[n - 1].cmd = tmp;
}

/// Determines the orientation of a polygon using the signed area method.
///
/// This function calculates the signed area of the polygon defined by `vertices`
/// and returns `Clockwise` if the area is negative, indicating clockwise orientation,
/// or `CounterClockwise` if positive.
pub fn perceive_polygon_orientation(vertices: &[Vertex<f64>]) -> PathOrientation {
    let n = vertices.len();
    let p0 = vertices[0];
    let mut area = 0.0;
    for (i, p1) in vertices.iter().enumerate() {
        let p2 = vertices[(i + 1) % n];
        let (x1, y1) = iif![p1.cmd == PathCommand::Close; (p0.x, p0.y); (p1.x, p1.y)];
        let (x2, y2) = iif![p2.cmd == PathCommand::Close; (p0.x, p0.y); (p2.x, p2.y)];
        area += x1 * y2 - y1 * x2;
    }
    iif![area < 0.0; PathOrientation::Clockwise; PathOrientation::CounterClockwise]
}

/// Calculates the bounding rectangle of the provided path.
///
/// This function iterates over all vertices in the path to find the minimum
/// and maximum coordinates, returning the smallest rectangle that contains all vertices.
pub fn bounding_rect<VS: VertexSource>(path: &VS) -> Option<Rectangle<f64>> {
    let pts = path.xconvert();
    if pts.is_empty() {
        None
    } else {
        let mut r = Rectangle::new(pts[0].x, pts[0].y, pts[0].x, pts[0].y);
        for p in pts {
            r.expand(p.x, p.y);
        }
        Some(r)
    }
}

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
    vertices: Vec<Vertex<f64>>,
}

impl VertexSource for Ellipse {
    fn xconvert(&self) -> Vec<Vertex<f64>> {
        self.vertices.clone()
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
            vertices: vec![],
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

    /// Computes the vertices for the ellipse, populating the `vertices` vector.
    pub fn calc(&mut self) {
        self.vertices = vec![];
        for i in 0..self.num {
            let angle = i as f64 / self.num as f64 * 2.0 * PI;
            let angle = if self.cw { 2.0 * PI - angle } else { angle };
            let x = self.x + angle.cos() * self.rx;
            let y = self.y + angle.sin() * self.ry;
            let v = iif![i == 0; Vertex::move_to(x, y); Vertex::line_to(x, y)];
            self.vertices.push(v);
        }
        let v = self.vertices[0];
        self.vertices.push(Vertex::close_polygon(v.x, v.y));
    }
}

/// Represents a rounded rectangle with specified corner radii and vertices.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoundedRect {
    x: [f64; 2],
    y: [f64; 2],
    rx: [f64; 4],
    ry: [f64; 4],
    vertices: Vec<Vertex<f64>>,
}

impl VertexSource for RoundedRect {
    fn xconvert(&self) -> Vec<Vertex<f64>> {
        self.vertices.clone()
    }
}

impl RoundedRect {
    /// Creates a new rounded rectangle defined by corners `(x1, y1)` to `(x2, y2)`
    /// with uniform corner radius `r`.
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, r: f64) -> Self {
        let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
        let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
        Self { x: [x1, x2], y: [y1, y2], rx: [r; 4], ry: [r; 4], vertices: vec![] }
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
            self.vertices.extend(verts);
        }
        if let Some(first) = self.vertices.first_mut() {
            first.cmd = PathCommand::MoveTo;
        }
        let first = self.vertices[0];
        self.vertices.push(Vertex::close_polygon(first.x, first.y));
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
    vertices: Vec<Vertex<f64>>,
}

impl VertexSource for Arc {
    fn xconvert(&self) -> Vec<Vertex<f64>> {
        self.vertices.clone()
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
            vertices: vec![],
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
            self.vertices.push(Vertex::line_to(x, y));
        }
        if let Some(first) = self.vertices.first_mut() {
            first.cmd = PathCommand::MoveTo;
        }
        if let Some(last) = self.vertices.last_mut() {
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
}
