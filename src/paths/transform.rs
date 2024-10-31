// agrega::paths::transform
//
//! Transformations

use core::ops::Mul;
#[allow(unused_imports)]
use devela::ExtFloat;

/// A 2D affine transformation matrix that supports translation, scaling, rotation, and skewing.
///
/// The matrix is represented by six components that allow transformations
/// in two dimensions, useful in graphics and geometry manipulation.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Transform {
    /// Scale in the x-direction
    pub sx: f64,
    /// Scale in the y-direction
    pub sy: f64,
    /// Shear in the x-direction
    pub shx: f64,
    /// Shear in the y-direction
    pub shy: f64,
    /// Translation in the x-direction
    pub tx: f64,
    /// Translation in the y-direction
    pub ty: f64,
}

impl Transform {
    /// Creates a new, identity `Transform` with no scaling, rotation, or translation.
    #[inline]
    #[must_use]
    pub const fn new() -> Transform {
        Self { sx: 1.0, sy: 1.0, shx: 0.0, shy: 0.0, tx: 0.0, ty: 0.0 }
    }

    /// Creates a scaling transformation with factors `sx` and `sy`.
    #[inline]
    #[must_use]
    pub fn scaling(sx: f64, sy: f64) -> Transform {
        let mut t = Self::new();
        t.set_scale(sx, sy);
        t
    }

    /// Creates a translation transformation that offsets by `(tx, ty)`.
    #[inline]
    #[must_use]
    pub fn translating(tx: f64, ty: f64) -> Transform {
        let mut t = Self::new();
        t.set_translate(tx, ty);
        t
    }
    /// Creates a rotation transformation by `angle` in radians.
    #[inline]
    #[must_use]
    pub fn rotating(angle: f64) -> Transform {
        let mut t = Self::new();
        t.set_rotate(angle);
        t
    }

    /// Adds a translation offset by `(dx, dy)` to the transform.
    #[inline]
    pub fn set_translate(&mut self, dx: f64, dy: f64) {
        self.tx += dx;
        self.ty += dy;
    }

    /// Applies a scaling factor in both x and y directions.
    #[inline] #[rustfmt::skip]
    pub fn set_scale(&mut self, sx: f64, sy: f64) {
        self.sx *= sx; self.shx *= sx; self.tx *= sx;
        self.sy *= sy; self.shy *= sy; self.ty *= sy;
    }

    /// Applies a rotation (in radians) around the origin to the transform.
    pub fn set_rotate(&mut self, angle: f64) {
        let (ca, sa) = (angle.cos(), angle.sin());
        let t0 = self.sx * ca - self.shy * sa;
        let t2 = self.shx * ca - self.sy * sa;
        let t4 = self.tx * ca - self.ty * sa;
        self.shy = self.sx * sa + self.shy * ca;
        self.sy = self.shx * sa + self.sy * ca;
        self.ty = self.tx * sa + self.ty * ca;
        self.sx = t0;
        self.shx = t2;
        self.tx = t4;
    }

    /// Returns a new `Transform` with an added translation by `(dx, dy)`.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn translate(mut self, dx: f64, dy: f64) -> Self {
        self.tx += dx; self.ty += dy; self
    }
    /// Returns a new `Transform` with an additional scale by `sx` and `sy`.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn scale(mut self, sx: f64, sy: f64) -> Self {
        self.sx *= sx; self.shx *= sx; self.tx *= sx;
        self.sy *= sy; self.shy *= sy; self.ty *= sy;
        self
    }
    /// Returns a new `Transform` with an added rotation by `angle` radians.
    #[inline]
    #[must_use]
    pub fn rotate(mut self, angle: f64) -> Self {
        let (ca, sa) = (angle.cos(), angle.sin());
        let t0 = self.sx * ca - self.shy * sa;
        let t2 = self.shx * ca - self.sy * sa;
        let t4 = self.tx * ca - self.ty * sa;
        self.shy = self.sx * sa + self.shy * ca;
        self.sy = self.shx * sa + self.sy * ca;
        self.ty = self.tx * sa + self.ty * ca;
        self.sx = t0;
        self.shx = t2;
        self.tx = t4;
        self
    }
    // Alternative compile-time friendly version of `rotate`.
    // WAIT: 1.83 & devela update
    // pub const fn const_rotate(mut self, angle: f64) -> Self {/*…*/}

    /// Applies the transformation to a point `(x, y)`, returning the transformed coordinates.
    #[inline]
    #[must_use]
    pub const fn transform(&self, x: f64, y: f64) -> (f64, f64) {
        (x * self.sx + y * self.shx + self.tx, x * self.shy + y * self.sy + self.ty)
    }

    /// Calculates the determinant of the transformation matrix, indicating its scaling factor.
    #[inline]
    #[must_use]
    const fn determinant(&self) -> f64 {
        self.sx * self.sy - self.shy * self.shx
    }

    /// Inverts the transform if possible, effectively reversing its effect.
    ///
    /// If the transform is not invertible (determinant is zero),
    /// this function will produce a transform with `NaN` values.
    pub fn invert(&mut self) {
        let d = 1.0 / self.determinant();
        let t0 = self.sy * d;
        self.sy = self.sx * d;
        self.shy = -self.shy * d;
        self.shx = -self.shx * d;
        let t4 = -self.tx * t0 - self.ty * self.shx;
        self.ty = -self.tx * self.shy - self.ty * self.sy;
        self.sx = t0;
        self.tx = t4;
    }

    /// Multiplies this transform by another, combining their transformations.
    pub const fn mul_transform(&self, m: &Transform) -> Self {
        let t0 = self.sx * m.sx + self.shy * m.shx;
        let t2 = self.shx * m.sx + self.sy * m.shx;
        let t4 = self.tx * m.sx + self.ty * m.shx + m.tx;
        let shy = self.sx * m.shy + self.shy * m.sy;
        let sy = self.shx * m.shy + self.sy * m.sy;
        let ty = self.tx * m.shy + self.ty * m.sy + m.ty;
        let sx = t0;
        let shx = t2;
        let tx = t4;
        Transform { sx, sy, tx, ty, shx, shy }
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;
    fn mul(self, rhs: Transform) -> Self {
        self.mul_transform(&rhs)
    }
}
