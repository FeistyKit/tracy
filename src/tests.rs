#![cfg(test)]

use crate::Line;

#[test]
fn same_line_intersect() {
    let l1 = Line {
        slope: 1.0,
        y_intercept: 0.0,
        min_x: 0.0,
        max_x: 1.0,
    };

    let l2 = Line {
        slope: 1.0,
        y_intercept: 0.0,
        min_x: 1.0,
        max_x: 2.0,
    };

    assert_eq!(l1.point_of_intersection(&l2), Some((1.0, 1.0).into()));
}

#[test]
fn same_slope_no_intersect() {
    let l1 = Line {
        slope: 1.0,
        y_intercept: 1.0,
        min_x: 0.0,
        max_x: 1.0,
    };

    let l2 = Line {
        slope: 1.0,
        y_intercept: 0.0,
        min_x: 1.0,
        max_x: 2.0,
    };

    assert_eq!(l1.point_of_intersection(&l2), None);
}

#[test]
fn different_slope_intersect() {
    let l1 = Line {
        slope: 2.0,
        y_intercept: -1.0,
        min_x: 0.0,
        max_x: 2.0,
    };

    let l2 = Line {
        slope: 1.0,
        y_intercept: 0.0,
        min_x: 0.0,
        max_x: 2.0,
    };

    assert_eq!(l1.point_of_intersection(&l2), Some((1.0, 1.0).into()));
}

#[test]
fn would_intersect_but_too_short() {
    let l1 = Line {
        slope: 2.0,
        y_intercept: -1.0,
        min_x: 0.0,
        max_x: 5.0,
    };

    let l2 = Line {
        slope: 1.0,
        y_intercept: 1.0,
        min_x: 0.0,
        max_x: 1.0,
    };

    assert_eq!(l1.point_of_intersection(&l2), None);
}
