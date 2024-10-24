//! Clipping Region

use crate::cell::RasterizerCell;
use core::cmp::PartialOrd;
use devela::iif;
#[allow(unused_imports)]
use devela::ExtFloat;

/// The sides of the clip region.
///
/// See [Liang Barsky](https://en.wikipedia.org/wiki/Liang-Barsky_algorithm)
/// and [Cyrus Beck](https://en.wikipedia.org/wiki/Cyrus-Beck_algorithm).
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ClipSide {
    /// Inside Region (Default).
    #[default]
    Inside = 0b0000,

    /// Left of Region.
    Left = 0b0000_0001,

    /// Right of Region.
    Right = 0b0000_0010,

    /// Below Region.
    Bottom = 0b0000_0100,

    /// Above Region.
    Top = 0b0000_1000,
}

impl ClipSide {
    /// The byte representation of [`Inside`][ClipSide::Inside].
    pub const INSIDE: u8 = ClipSide::Inside as u8;
    /// The byte representation of [`Left`][ClipSide::Left].
    pub const LEFT: u8 = ClipSide::Left as u8;
    /// The byte representation of [`Right`][ClipSide::Right].
    pub const RIGHT: u8 = ClipSide::Right as u8;
    /// The byte representation of [`Bottom`][ClipSide::Bottom].
    pub const BOTTOM: u8 = ClipSide::Bottom as u8;
    /// The byte representation of [`Top`][ClipSide::Top].
    pub const TOP: u8 = ClipSide::Top as u8;

    /// Determine the location of a point to a broken-down rectangle or range.
    ///
    /// Returned is an a u8 made up of the bits of `ClipSide` variants.
    #[inline]
    fn clip_flags<T: PartialOrd>(x: &T, y: &T, x1: &T, y1: &T, x2: &T, y2: &T) -> u8 {
        let mut code = ClipSide::Inside as u8;
        iif![x < x1; code |= ClipSide::Left as u8];
        iif![x > x2; code |= ClipSide::Right as u8];
        iif![y < y1; code |= ClipSide::Bottom as u8];
        iif![y > y2; code |= ClipSide::Top as u8];
        code
    }
}

/// A rectangle defined by 4 sorted.
#[derive(Debug, Copy, Clone)]
pub struct Rectangle<T: PartialOrd + Copy> {
    /// Minimum x value
    x1: T,
    /// Minimum y value
    y1: T,
    /// Maximum x value
    x2: T,
    /// Maximum y value
    y2: T,
}

impl<T: PartialOrd + Copy> Rectangle<T> {
    /// Creates a new Rectangle
    ///
    /// Values are sorted before storing.
    pub fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
        let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
        let (y1, y2) = if y1 > x2 { (y2, y1) } else { (y1, y2) };
        Self { x1, y1, x2, y2 }
    }

    /// Get location of point relative to rectangle
    ///
    /// Returned is an a u8 made up of the bits of `ClipSide` variants.
    pub fn clip_flags(&self, x: T, y: T) -> u8 {
        ClipSide::clip_flags(&x, &y, &self.x1, &self.y1, &self.x2, &self.y2)
    }

    /// Expand if the point (x,y) is outside
    pub fn expand(&mut self, x: T, y: T) {
        iif![x < self.x1; self.x1 = x];
        iif![x > self.x2; self.x2 = x];
        iif![y < self.y1; self.y1 = y];
        iif![y > self.y2; self.y2 = y];
    }

    /// Expand if the rectangle is outside
    pub fn expand_rect(&mut self, r: &Rectangle<T>) {
        self.expand(r.x1, r.y1);
        self.expand(r.x2, r.y2);
    }

    /// Returns `x1`.
    #[inline]
    pub fn x1(&self) -> T {
        self.x1
    }

    /// Returns `x1`.
    #[inline]
    pub fn x2(&self) -> T {
        self.x2
    }

    /// Returns `y1`.
    #[inline]
    pub fn y1(&self) -> T {
        self.y1
    }

    /// Returns `y2`.
    #[inline]
    pub fn y2(&self) -> T {
        self.y2
    }
}

/// Clip Region
///
/// Clipping for Rasterizers
#[derive(Debug)]
pub struct Clip {
    /// Current x Point
    x1: i64,
    /// Current y Point
    y1: i64,
    /// Rectangle to clip on
    clip_box: Option<Rectangle<i64>>,
    /// Current clip flag for point (x1,y1)
    clip_flag: u8,
}

impl Default for Clip {
    fn default() -> Self {
        Self::new()
    }
}

fn mul_div(a: i64, b: i64, c: i64) -> i64 {
    let (a, b, c) = (a as f64, b as f64, c as f64);
    (a * b / c).round() as i64
}
impl Clip {
    /// Create new Clipping region
    pub fn new() -> Self {
        Self { x1: 0, y1: 0, clip_box: None, clip_flag: ClipSide::INSIDE }
    }

    /// Clip a line along the top and bottom of the region
    #[expect(clippy::too_many_arguments)]
    fn line_clip_y(
        &self,
        ras: &mut RasterizerCell,
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        f1: u8,
        f2: u8,
    ) {
        let b = match self.clip_box {
            None => return,
            Some(ref b) => b,
        };
        let f1 = f1 & (ClipSide::TOP | ClipSide::BOTTOM);
        let f2 = f2 & (ClipSide::TOP | ClipSide::BOTTOM);
        // Fully Visible in y
        if f1 == ClipSide::INSIDE && f2 == ClipSide::INSIDE {
            ras.line(x1, y1, x2, y2);
        } else {
            // Both points above or below clip box
            if f1 == f2 {
                return;
            }
            let (mut tx1, mut ty1, mut tx2, mut ty2) = (x1, y1, x2, y2);
            if f1 == ClipSide::BOTTOM {
                tx1 = x1 + mul_div(b.y1 - y1, x2 - x1, y2 - y1);
                ty1 = b.y1;
            }
            if f1 == ClipSide::TOP {
                tx1 = x1 + mul_div(b.y2 - y1, x2 - x1, y2 - y1);
                ty1 = b.y2;
            }
            if f2 == ClipSide::BOTTOM {
                tx2 = x1 + mul_div(b.y1 - y1, x2 - x1, y2 - y1);
                ty2 = b.y1;
            }
            if f2 == ClipSide::TOP {
                tx2 = x1 + mul_div(b.y2 - y1, x2 - x1, y2 - y1);
                ty2 = b.y2;
            }
            ras.line(tx1, tx2, ty1, ty2);
        }
    }

    /// Draw a line from (x1,y1) to (x2,y2) into a RasterizerCell
    ///
    /// Final point (x2,y2) is saved internally as (x1,y1))
    pub(crate) fn line_to(&mut self, ras: &mut RasterizerCell, x2: i64, y2: i64) {
        if let Some(ref b) = self.clip_box {
            let f2 = b.clip_flags(x2, y2);
            // Both points above or below clip box
            let fy1 = (ClipSide::TOP | ClipSide::BOTTOM) & self.clip_flag;
            let fy2 = (ClipSide::TOP | ClipSide::BOTTOM) & f2;
            if fy1 != ClipSide::INSIDE && fy1 == fy2 {
                self.x1 = x2;
                self.y1 = y2;
                self.clip_flag = f2;
                return;
            }
            let (x1, y1, f1) = (self.x1, self.y1, self.clip_flag);
            match (f1 & (ClipSide::LEFT | ClipSide::RIGHT), f2 & (ClipSide::LEFT | ClipSide::RIGHT))
            {
                (ClipSide::INSIDE, ClipSide::INSIDE) => {
                    self.line_clip_y(ras, x1, y1, x2, y2, f1, f2)
                }
                (ClipSide::INSIDE, ClipSide::RIGHT) => {
                    let y3 = y1 + mul_div(b.x2 - x1, y2 - y1, x2 - x1);
                    let f3 = b.clip_flags(b.x2, y3);
                    self.line_clip_y(ras, x1, y1, b.x2, y3, f1, f3);
                    self.line_clip_y(ras, b.x2, y3, b.x2, y2, f3, f2);
                }
                (ClipSide::RIGHT, ClipSide::INSIDE) => {
                    let y3 = y1 + mul_div(b.x2 - x1, y2 - y1, x2 - x1);
                    let f3 = b.clip_flags(b.x2, y3);
                    self.line_clip_y(ras, b.x2, y1, b.x2, y3, f1, f3);
                    self.line_clip_y(ras, b.x2, y3, x2, y2, f3, f2);
                }
                (ClipSide::INSIDE, ClipSide::LEFT) => {
                    let y3 = y1 + mul_div(b.x1 - x1, y2 - y1, x2 - x1);
                    let f3 = b.clip_flags(b.x1, y3);
                    self.line_clip_y(ras, x1, y1, b.x1, y3, f1, f3);
                    self.line_clip_y(ras, b.x1, y3, b.x1, y2, f3, f2);
                }
                (ClipSide::RIGHT, ClipSide::LEFT) => {
                    let y3 = y1 + mul_div(b.x2 - x1, y2 - y1, x2 - x1);
                    let y4 = y1 + mul_div(b.x1 - x1, y2 - y1, x2 - x1);
                    let f3 = b.clip_flags(b.x2, y3);
                    let f4 = b.clip_flags(b.x1, y4);
                    self.line_clip_y(ras, b.x2, y1, b.x2, y3, f1, f3);
                    self.line_clip_y(ras, b.x2, y3, b.x1, y4, f3, f4);
                    self.line_clip_y(ras, b.x1, y4, b.x1, y2, f4, f2);
                }
                (ClipSide::LEFT, ClipSide::INSIDE) => {
                    let y3 = y1 + mul_div(b.x1 - x1, y2 - y1, x2 - x1);
                    let f3 = b.clip_flags(b.x1, y3);
                    self.line_clip_y(ras, b.x1, y1, b.x1, y3, f1, f3);
                    self.line_clip_y(ras, b.x1, y3, x2, y2, f3, f2);
                }
                (ClipSide::LEFT, ClipSide::RIGHT) => {
                    let y3 = y1 + mul_div(b.x1 - x1, y2 - y1, x2 - x1);
                    let y4 = y1 + mul_div(b.x2 - x1, y2 - y1, x2 - x1);
                    let f3 = b.clip_flags(b.x1, y3);
                    let f4 = b.clip_flags(b.x2, y4);
                    self.line_clip_y(ras, b.x1, y1, b.x1, y3, f1, f3);
                    self.line_clip_y(ras, b.x1, y3, b.x2, y4, f3, f4);
                    self.line_clip_y(ras, b.x2, y4, b.x2, y2, f4, f2);
                }
                (ClipSide::LEFT, ClipSide::LEFT) => {
                    self.line_clip_y(ras, b.x1, y1, b.x1, y2, f1, f2)
                }
                (ClipSide::RIGHT, ClipSide::RIGHT) => {
                    self.line_clip_y(ras, b.x2, y1, b.x2, y2, f1, f2)
                }

                (_, _) => unreachable!("f1,f2 {:?} {:?}", f1, f2),
            }
            self.clip_flag = f2;
        } else {
            ras.line(self.x1, self.y1, x2, y2);
        }
        self.x1 = x2;
        self.y1 = y2;
    }
    /// Move to point (x2,y2)
    ///
    /// Point is saved internally as (x1,y1)
    pub(crate) fn move_to(&mut self, x2: i64, y2: i64) {
        self.x1 = x2;
        self.y1 = y2;
        if let Some(ref b) = self.clip_box {
            self.clip_flag = ClipSide::clip_flags(&x2, &y2, &b.x1, &b.y1, &b.x2, &b.y2);
        }
    }
    /// Define the clipping region
    pub fn clip_box(&mut self, x1: i64, y1: i64, x2: i64, y2: i64) {
        self.clip_box = Some(Rectangle::new(x1, y1, x2, y2));
    }
}
