// agrega::interp::line_defs
//
//! Line interpolation, definitions.
//
// TOC
// - struct LineParameters
// - struct LineInterpolator
// - struct LineInterpolatorAA
// - struct AA0
// - struct AA1
// - struct AA2
// - struct AA3
// - struct DrawVars

use super::{DistanceInterpolator1, DistanceInterpolator2, DistanceInterpolator3};

/// Line Parameters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LineParameters {
    /// Starting x position
    pub x1: i64,
    /// Starting y position
    pub y1: i64,
    /// Ending x position
    pub x2: i64,
    /// Ending y position
    pub y2: i64,
    /// Distance from x1 to x2
    pub dx: i64,
    /// Distance from y1 to y2
    pub dy: i64,
    /// Direction of the x coordinate (positive or negative)
    pub sx: i64,
    /// Direction of the y coordinate (positive or negative)
    pub sy: i64,
    /// If line is more vertical than horizontal
    pub vertical: bool,
    /// Increment of the line, `sy` if vertical, else `sx`
    pub inc: i64,
    /// Length of the line
    pub len: i64,
    /// Identifier of which direction the line is headed
    ///   bit 1 - vertical
    ///   bit 2 - sx < 0
    ///   bit 3 - sy < 0
    ///  bits - V? | sx | sy | value | diag quadrant
    ///   000 - H    +    +     0         0
    ///   100 - V    +    +     1         1
    ///   010 - H    -    +     2         2
    ///   110 - V    -    +     3         1
    ///   001 - H    +    -     4         0
    ///   101 - V    +    -     5         3
    ///   011 - H    -    -     6         2
    ///   111 - V    -    -     7         3
    ///             1 <- diagonal quadrant
    ///        .  3 | 1  .
    ///          .  |  .
    ///       2    .|.   0 <- octant
    ///     2 ------+------ 0
    ///       6    .|.   4
    ///          .  |  .
    ///        .  7 | 5  .
    ///             3
    pub octant: usize,
}

/// Line Interpolator using a Digital differential analyzer (DDA)
///
/// Step through a range from numbers, from `y1` to `y2`, into `count` items
///
/// See [https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)]()
///
/// This is equivalent to dda2 in the original agg
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct LineInterpolator {
    /// Number of Segments
    pub count: i64,
    /// Minimum Step Size, Constant, (y2-y1)/count
    pub left: i64,
    /// Remainder, Constant, (y2-y1) % count
    pub rem: i64,
    /// Error term
    pub xmod: i64,
    /// Current y value
    pub y: i64,
}

/// Line Interpolator AA
#[derive(Clone, Debug)]
pub(crate) struct LineInterpolatorAA {
    /// Line Parameters
    pub lp: LineParameters,
    /// Line Interpolator
    pub li: LineInterpolator,
    /// Length of Line
    pub len: i64,
    /// Current x position of line in pixels
    pub x: i64,
    /// Current y position of line in pixels
    pub y: i64,
    /// Previous x position in pixels
    pub old_x: i64,
    /// Previous y position in pixels
    pub old_y: i64,
    /// Number of pixels from start to end points
    ///  in either the `y` or `x` direction
    pub count: i64,
    /// Width of line in subpixels width
    pub width: i64,
    /// Maximum width of line in pixels
    pub max_extent: i64,

    pub step: i64,
    //pub dist: [i64; MAX_HALF_WIDTH + 1],
    pub dist: Vec<i64>,
    //pub covers: [u64; MAX_HALF_WIDTH * 2 + 4],
    pub covers: Vec<u64>,
}

/// TODO
#[derive(Debug)]
pub(crate) struct AA3 {
    pub di: DistanceInterpolator3,
    pub li: LineInterpolatorAA,
}

/// Line Interpolator0
#[derive(Debug)]
pub(crate) struct AA0 {
    /// Distance Interpolator v1
    pub di: DistanceInterpolator1,
    /// Line Interpolator AA-version
    pub li: LineInterpolatorAA,
}
//
#[derive(Debug)]
pub(crate) struct AA1 {
    pub di: DistanceInterpolator2,
    pub li: LineInterpolatorAA,
}
//
#[derive(Debug)]
pub(crate) struct AA2 {
    pub di: DistanceInterpolator2,
    pub li: LineInterpolatorAA,
}

//
#[derive(Debug, Default)]
pub(crate) struct DrawVars {
    pub idx: usize,
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
    pub curr: LineParameters,
    pub next: LineParameters,
    pub lcurr: i64,
    pub lnext: i64,
    pub xb1: i64,
    pub yb1: i64,
    pub xb2: i64,
    pub yb2: i64,
    pub flags: u8,
}
