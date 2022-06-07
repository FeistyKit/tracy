use crate::math::*;

#[test]
fn same_line_intersect() {
    let l1 = Line::new(0.0, 0.0, 1.0, 1.0);

    let l2 = Line::new(1.0, 1.0, 2.0, 2.0);

    assert_eq!(l1.point_of_intersection(&l2), Some((1.0, 1.0).into()));
}

#[test]
fn same_slope_no_intersect() {
    let l1 = Line::new(0.0, 1.0, 1.0, 2.0);

    let l2 = Line::new(1.0, 1.0, 2.0, 2.0);

    assert_eq!(l1.point_of_intersection(&l2), None);
}

#[test]
fn different_slope_intersect() {
    let l1 = Line::new(0.0, -1.0, 1.0, 1.0);

    let l2 = Line::new(0.0, 0.0, 2.0, 2.0);

    assert_eq!(l1.point_of_intersection(&l2), Some((1.0, 1.0).into()));
}

#[test]
fn would_intersect_but_too_short() {
    let l1 = Line::new(0.0, -1.0, 5.0, 9.0);

    let l2 = Line::new(0.0, 1.0, 1.0, 2.0);

    assert_eq!(l1.point_of_intersection(&l2), None);
}

#[test]
fn test_scene_ltr() {
    let mut scene = Scene::new();

    scene.add_line_no_graphics((0.0, 0.0).into(), (1.0, 1.0).into());
    scene.add_line_no_graphics((1.0, -1.0).into(), (2.0, 0.0).into());

    let line = Line::new(0.0, 1.0, 3.0, -2.0);

    let line_after_intersect = line.cast_in_scene(&scene);

    let expected = Line::new(0.0, 1.0, 0.5, 0.5);

    assert_eq!(line_after_intersect, expected);
}

#[test]
fn test_scene_rtl() {
    let mut scene = Scene::new();

    scene.add_line_no_graphics((1.0, -1.0).into(), (2.0, 0.0).into());
    scene.add_line_no_graphics((0.0, 0.0).into(), (1.0, 1.0).into());

    let line = Line::new(3.0, -2.0, 0.0, 1.0);

    let line_after_intersect = line.cast_in_scene(&scene);

    let expected = Line::new(3.0, -2.0, 1.5, -0.5);

    assert_eq!(line_after_intersect, expected);
}
