// agrega::paths
//
// TOC
// - enum PathOrientation
// - enum PathCommand
// - trait VertexSource
// - struct Vertex
// - struct Path
// - fn arrange_orientations
// - fn invert_polygon
// - fn perceive_polygon_orientation
// - fn bounding_rect

use alloc::{vec, vec::Vec};
use devela::iif;
#[allow(unused_imports)]
use devela::ExtFloat;

mod clip;
mod transform;
pub use {clip::*, transform::*};

#[cfg(any(feature = "std", feature = "no_std"))]
crate::items! {
    mod shapes;
    mod stroke;
    pub use {shapes::*, stroke::*};
}

/// Represents the orientation of a polygon path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PathOrientation {
    Clockwise,
    CounterClockwise,
}

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

/// A source of vertex points.
pub trait VertexSource {
    // /// Rewind the vertex source (unused)
    // fn rewind(&self) {}

    /// Get the cloned values from the source.
    ///
    /// This could be turned into an iterator
    #[must_use]
    fn xconvert(&self) -> alloc::vec::Vec<Vertex<f64>>;
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
    #[must_use]
    pub const fn new(x: T, y: T, command: PathCommand) -> Self {
        Self { x, y, cmd: command }
    }
    /// Creates a vertex at `(x, y)` with `Stop` command.
    #[inline]
    #[must_use]
    pub const fn xy(x: T, y: T) -> Self {
        Self { x, y, cmd: PathCommand::Stop }
    }
    /// Moves the cursor to `(x, y)` without drawing.
    #[inline]
    #[must_use]
    pub const fn move_to(x: T, y: T) -> Self {
        Self { x, y, cmd: PathCommand::MoveTo }
    }
    /// Draws a line to `(x, y)` from the current position.
    #[inline]
    #[must_use]
    pub const fn line_to(x: T, y: T) -> Self {
        Self { x, y, cmd: PathCommand::LineTo }
    }
    /// Closes the current path by connecting back to the start.
    #[inline]
    #[must_use]
    pub const fn close_polygon(x: T, y: T) -> Self {
        Self { x, y, cmd: PathCommand::Close }
    }
}
impl Vertex<f64> {
    /// Compute length between two points.
    #[inline]
    #[must_use]
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
    #[inline]
    #[must_use]
    pub const fn cross(p1: &Vertex<f64>, p2: &Vertex<f64>, p: &Vertex<f64>) -> f64 {
        (p.x - p2.x) * (p2.y - p1.y) - (p.y - p2.y) * (p2.x - p1.x)
    }

    /// Split vertices of a path into individual segments at MoveTo boundaries.
    #[must_use]
    pub fn split(vertices: &[Vertex<f64>]) -> Vec<(usize, usize)> {
        let (mut start, mut end) = (None, None);
        let mut pairs = vec![];
        for (i, v) in vertices.iter().enumerate() {
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
}

/// Represents a path of connected vertices, each with an associated command.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Path {
    /// A list of vertices that together form shapes or paths,
    /// where each vertex defines a position and a drawing command.
    pub vertices: Vec<Vertex<f64>>,
}

impl VertexSource for Path {
    #[inline]
    fn xconvert(&self) -> Vec<Vertex<f64>> {
        self.vertices.clone()
    }
}

impl Path {
    /// Creates a new, empty path.
    #[inline]
    #[must_use]
    pub fn new() -> Path {
        Self { vertices: vec![] }
    }

    /// Creates a new path with the given `vertices`.
    #[inline]
    #[must_use]
    pub fn with(vertices: Vec<Vertex<f64>>) -> Path {
        Self { vertices }
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

    /// Split path's vertices into individual segments at MoveTo boundaries.
    #[inline]
    #[must_use]
    pub fn split(&self) -> Vec<(usize, usize)> {
        Vertex::split(self.vertices.as_ref())
    }
}

// Adjusts the orientation of all polygons in the path to match the specified direction.
//
// This function detects the orientation of each polygon in the path and
// inverts any that do not match the given `PathOrientation`.
fn arrange_orientations(path: &mut Path, dir: PathOrientation) {
    let pairs = path.split();
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
#[must_use]
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
#[must_use]
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
