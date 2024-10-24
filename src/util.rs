// agrega::util

#![allow(dead_code)]

pub(crate) const POLY_SUBPIXEL_SHIFT: i64 = 8;
pub(crate) const POLY_SUBPIXEL_SCALE: i64 = 1 << POLY_SUBPIXEL_SHIFT;
pub(crate) const POLY_SUBPIXEL_MASK: i64 = POLY_SUBPIXEL_SCALE - 1;
pub(crate) const POLY_MR_SUBPIXEL_SHIFT: i64 = 4;
pub(crate) const MAX_HALF_WIDTH: usize = 64;

/// Interpolates a value between two end points using fixed point math.
///
/// See agg_color_rgba.h:454 of agg version 2.4
#[inline]
#[must_use]
pub const fn lerp_u8(p: u8, q: u8, a: u8) -> u8 {
    let base_shift = 8;
    let base_msb = 1 << (base_shift - 1);
    let v = if p > q { 1 } else { 0 };
    let (q, p, a) = (q as i32, p as i32, a as i32);
    let t0: i32 = (q - p) * a + base_msb - v; // Signed multiplication
    let t1: i32 = ((t0 >> base_shift) + t0) >> base_shift;
    (p + t1) as u8
}

/// Interpolates a value between two end points pre-calculated by alpha.
///
/// Performs a pre-calculated linear interpolation between two end points
/// `p` and `q`, using `a`lpha as a proportion.
///
/// The operation is `p + q - (p * a)`, with `p * a` calculated using fixed point
/// math. If the sum of `p` and `q` exceeds 255 or the subtraction result is less
/// than 0, they wrap around in accordance with the properties of `u8`.
#[inline]
#[must_use]
pub const fn prelerp_u8(p: u8, q: u8, a: u8) -> u8 {
    p.wrapping_add(q).wrapping_sub(multiply_u8(p, a))
}

/// Multiplies two u8 values using fixed point math.
///
/// See agg_color_rgba.h:395
///
/// - <https://sestevenson.wordpress.com/2009/08/19/rounding-in-fixed-point-number-conversions/>
/// - <https://stackoverflow.com/questions/10067510/fixed-point-arithmetic-in-c-programming>
/// - <http://x86asm.net/articles/fixed-point-arithmetic-and-tricks/>
///
/// Still not sure where the value is added and shifted multiple times
#[inline]
#[must_use]
pub const fn multiply_u8(a: u8, b: u8) -> u8 {
    let base_shift = 8;
    let base_msb = 1 << (base_shift - 1);
    let (a, b) = (a as u32, b as u32);
    let t: u32 = a * b + base_msb;
    let tt: u32 = ((t >> base_shift) + t) >> base_shift;
    tt as u8
}

#[cfg(test)]
mod tests {
    use super::{lerp_u8, multiply_u8};
    // use super::prelerp_u8; // FIXME

    fn mu864(i: u8, j: u8) -> u8 {
        let i = i as f64 / 255.0;
        let j = j as f64 / 255.0;
        let c = i * j;
        (c * 255.0).round() as u8
    }
    fn lerp_u8_f64(p: u8, q: u8, a: u8) -> u8 {
        let p = p as f64 / 255.0;
        let q = q as f64 / 255.0;
        let a = a as f64 / 255.0;
        let v = a * (q - p) + p;
        (v * 255.0).round() as u8
    }

    // FIXME
    // fn prelerp_u8_f64(p: u8, q: u8, a: u8) -> u8 {
    //     let p = p as f64 / 255.0;
    //     let q = q as f64 / 255.0;
    //     let a = a as f64 / 255.0;
    //     let v = p + q - (p * a);
    //     (v * 255.0).round() as u8
    // }

    #[test]
    fn lerp_u8_test() {
        for p in 0..=255 {
            for q in 0..=255 {
                for a in 0..=255 {
                    let (p, q, a) = (p as u8, q as u8, a as u8);
                    let v = lerp_u8_f64(p, q, a);
                    assert_eq!(lerp_u8(p, q, a), v, "lerp({},{},{}) = {}", p, q, a, v);
                }
            }
        }
    }

    // #[test] // FIXME
    // fn prelerp_u8_test() {
    //     for p in 0..=255 {
    //         for q in 0..=255 {
    //             for a in 0..=255 {
    //                 let (p, q, a) = (p as u8, q as u8, a as u8);
    //                 let v = prelerp_u8_f64(p, q, a);
    //                 assert_eq!(prelerp_u8(p, q, a), v, "prelerp({},{},{}) = {}", p, q, a, v);
    //             }
    //         }
    //     }
    // }
    #[test]
    fn multiply_u8_test() {
        for i in 0..=255 {
            for j in 0..=255 {
                let v = mu864(i, j);
                assert_eq!(multiply_u8(i, j), v, "{} * {} = {}", i, j, v);
            }
        }
    }
}
