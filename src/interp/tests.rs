// agrega::interp::tests

use super::{
    DistanceInterpolator, DistanceInterpolator1, LineInterpolator, LineInterpolatorAA,
    LineParameters,
};
use crate::POLY_SUBPIXEL_MASK;
use alloc::vec;

#[test]
fn line_interpolator() {
    let mut lp = LineInterpolator::new(0 << 8, 10 << 8, 10 << 8);
    for i in 0..=10 {
        assert_eq!(lp.y, i);
        lp.inc();
    }
    let mut lp = LineInterpolator::new(0, 100, 2);
    for &i in [0, 50, 100, 150].iter() {
        assert_eq!(lp.y, i);
        lp.inc();
    }
    let mut lp = LineInterpolator::new(0, 10, 3);
    let y0 = vec![0, 3, 6, 10];
    let left0 = vec![3, 3, 3, 3];
    let xmod0 = vec![-2, -1, 0, -2];
    let rem0 = vec![1, 1, 1, 1];
    let mut left = vec![];
    let mut xmod = vec![];
    let mut rem = vec![];
    let mut y = vec![];
    for _ in 0..4 {
        left.push(lp.left());
        y.push(lp.y);
        xmod.push(lp.xmod());
        rem.push(lp.rem());
        lp.inc();
    }
    assert_eq!(left0, left);
    assert_eq!(xmod0, xmod);
    assert_eq!(rem0, rem);
    assert_eq!(y0, y);

    let mut lp = LineInterpolator::new(0, 10, 4);
    let y0 = vec![0, 2, 5, 7, 10];
    let left0 = vec![2, 2, 2, 2, 2];
    let xmod0 = vec![-2, 0, -2, 0, -2];
    let rem0 = vec![2, 2, 2, 2, 2];
    let mut left = vec![];
    let mut xmod = vec![];
    let mut rem = vec![];
    let mut y = vec![];
    for _ in 0..5 {
        left.push(lp.left());
        y.push(lp.y);
        xmod.push(lp.xmod());
        rem.push(lp.rem());
        lp.inc();
    }
    assert_eq!(left0, left);
    assert_eq!(xmod0, xmod);
    assert_eq!(rem0, rem);
    assert_eq!(y0, y);
}

#[test]
#[rustfmt::skip]
#[allow(clippy::erasing_op, reason = "0 & !POLY_SUBPIXEL_MASK")]
fn test_di1() {
    let mut d = DistanceInterpolator1::new(10, 10, 30, 10,
        10 & !POLY_SUBPIXEL_MASK, 10 & !POLY_SUBPIXEL_MASK);
    assert_eq!(d.dx, 20 << 8);
    assert_eq!(d.dy, 0 << 8);
    assert_eq!(d.dist, -2360);
    d.inc_x(1);
    assert_eq!(d.dist, -7480);
    d.inc_x(1);
    assert_eq!(d.dist, -12600);
    d.inc_x(1);
    assert_eq!(d.dist, -17720);

    let mut d =
        DistanceInterpolator1::new(0, 0, 30, 0, 0 & !POLY_SUBPIXEL_MASK, 0 & !POLY_SUBPIXEL_MASK);
    assert_eq!(d.dx, 7680); // 30 << 8
    assert_eq!(d.dy, 0); //  0 << 8
    assert_eq!(d.dist, -3840);
    d.inc_x(1);
    assert_eq!(d.dist, -11520);
    d.inc_x(2);
    assert_eq!(d.dist, -19200);
    d.inc_x(87);
    assert_eq!(d.dist, -26880);
}

#[test]
#[rustfmt::skip]
fn test_line_interpolator_aa() {
    let (x1, y1) = (0, 0);
    let (x2, y2) = (100, 50);
    let length = 100;
    let lp = LineParameters::new(x1, y1, x2, y2, length);
    let mut di = DistanceInterpolator1::new(lp.x1, lp.y1, lp.x2, lp.y2,
        lp.x1 & !POLY_SUBPIXEL_MASK, lp.y1 & !POLY_SUBPIXEL_MASK);
    let mut aa = LineInterpolatorAA::new(lp, 10 << 8);
    let v = aa.step_hor_base(&mut di);
    assert_eq!(v, 64);
    let v = aa.step_hor_base(&mut di);
    assert_eq!(v, 192);
    let v = aa.step_hor_base(&mut di);
    assert_eq!(v, 64);
    let v = aa.step_hor_base(&mut di);
    assert_eq!(v, 192);
}
