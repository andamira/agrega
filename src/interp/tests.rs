// agrega::interp::tests

use super::DistanceInterpolator;
use super::DistanceInterpolator1;
use crate::POLY_SUBPIXEL_MASK;

#[test]
fn test_di1() {
    let mut d = DistanceInterpolator1::new(
        10,
        10,
        30,
        10,
        10 & !POLY_SUBPIXEL_MASK,
        10 & !POLY_SUBPIXEL_MASK,
    );
    assert_eq!(d.dx(), 20 << 8);
    assert_eq!(d.dy(), 0 << 8);
    assert_eq!(d.dist, -2360);
    d.inc_x(1);
    assert_eq!(d.dist, -7480);
    d.inc_x(1);
    assert_eq!(d.dist, -12600);
    d.inc_x(1);
    assert_eq!(d.dist, -17720);

    #[allow(clippy::erasing_op, reason = "0 & !POLY_SUBPIXEL_MASK")]
    let mut d =
        DistanceInterpolator1::new(0, 0, 30, 0, 0 & !POLY_SUBPIXEL_MASK, 0 & !POLY_SUBPIXEL_MASK);
    assert_eq!(d.dx(), 7680); // 30 << 8
    assert_eq!(d.dy(), 0); //  0 << 8
    assert_eq!(d.dist, -3840);
    d.inc_x(1);
    assert_eq!(d.dist, -11520);
    d.inc_x(2);
    assert_eq!(d.dist, -19200);
    d.inc_x(87);
    assert_eq!(d.dist, -26880);
}

use super::LineInterpolatorAA;
use super::LineParameters;
#[test]
fn test_line_interpolator_aa() {
    let (x1, y1) = (0, 0);
    let (x2, y2) = (100, 50);
    let length = 100;
    let lp = LineParameters::new(x1, y1, x2, y2, length);
    let mut di = DistanceInterpolator1::new(
        lp.x1,
        lp.y1,
        lp.x2,
        lp.y2,
        lp.x1 & !POLY_SUBPIXEL_MASK,
        lp.y1 & !POLY_SUBPIXEL_MASK,
    );
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
