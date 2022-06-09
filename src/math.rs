use std::ops;

use sfml::{graphics, system};

macro_rules! line_impl {
    (mut $name:ident($($x:ident: $typ:ty),*) -> $return:ty) => {
        pub fn $name(&mut self, $($x: $typ,)*) -> $return {
            match self {
                Self::Vertical(v) => v.$name($($x)*),
                Self::Normal(v) => v.$name($($x)*),
            }
        }
    };

    (mut $name:ident($($x:ident: $typ:ty),*)) => {
        pub fn $name(&mut self, $($x: $typ,)*) {
            match self {
                Self::Vertical(v) => v.$name($($x,)*),
                Self::Normal(v) => v.$name($($x,)*),
            }
        }
    };

    (nonmut $name:ident($($x:ident: $typ:ty),*) -> $return:ty) => {
        pub fn $name(&self, $($x: $typ,)*) -> $return {
            match self {
                Self::Vertical(v) => v.$name($($x,)*),
                Self::Normal(v) => v.$name($($x,)*),
            }
        }
    };
    (nonmut $name:ident($($x:ident: $typ:ty),*)) => {
        pub fn $name(&self, $($x: $typ,)*) {
            match self {
                Self::Vertical(v) => v.$name($($x,)*),
                Self::Normal(v) => v.$name($($x,)*),
            }
        }
    };
}

macro_rules! line_inner {
    (mut $name:ident($($x:ident: $typ:ty),*) -> $return:ty) => {
        pub fn $name(&mut self, $($x: $typ,)*) -> $return {
            self.inner.$name($($x,)*)
        }
    };

    (mut $name:ident($($x:ident: $typ:ty),*)) => {
        pub fn $name(&mut self, $($x: $typ,)*) {
            self.inner.$name($($x,)*)
        }
    };

    (nonmut $name:ident($($x:ident: $typ:ty),*) -> $return:ty) => {
        pub fn $name(&self, $($x: $typ,)*) -> $return {
            self.inner.$name($($x,)*)
        }
    };
    (nonmut $name:ident($($x:ident: $typ:ty),*)) => {
        pub fn $name(&self, $($x: $typ,)*) {
            self.inner.$name($($x,)*)
        }
    };
}

#[derive(Debug)]
pub struct Scene {
    walls: Vec<Line>,
    last_point: Option<Point>
}

impl Scene {
    pub fn new() -> Self {
        Self {
            walls: Vec::new(),
            last_point: None
        }
    }

    #[allow(dead_code)]
    pub fn re_init_graphics(&self, arr: &mut graphics::VertexArray) {
        for line in &self.walls {
            let (start_v, end_v) = line.renderable(graphics::Color::GREEN);

            arr.append(&start_v);
            arr.append(&end_v);
        }
    }

    pub fn add_line(&mut self, start: Point, end: Point, arr: &mut graphics::VertexArray) {
        self.last_point = Some(end);
        let line = Line::from_points(start, end);
        let (start_v, end_v) = line.renderable(graphics::Color::GREEN);

        arr.append(&start_v);
        arr.append(&end_v);
        self.walls.push(line);
    }

    pub fn add_line_continuous(&mut self, new: Point, arr: &mut graphics::VertexArray) {
        if let Some(last) = self.last_point {
            self.add_line(last, new, arr);
        }
    }

    #[allow(dead_code)]
    pub fn add_line_no_graphics(&mut self, start: Point, end: Point) {
        self.last_point = Some(end);
        let line = Line::from_points(start, end);

        self.walls.push(line);
    }

    #[allow(dead_code)]
    pub fn add_line_continuous_no_graphics(&mut self, new: Point) {
        if let Some(last) = self.last_point {
            self.add_line_no_graphics(last, new);
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct NormalLine {
    slope: f32,
    y_intercept: f32,
    min_x: f32,
    max_x: f32,
    left_to_right: bool
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct VerticalLine {
    x: f32,
    max_y: f32,
    min_y: f32,
    down_to_up: bool
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum LineInner {
    Normal(NormalLine),
    Vertical(VerticalLine)
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Line {
    inner: LineInner,
}

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        let inner = LineInner::from_points((x1, y1).into(), (x2, y2).into());
        Self {
            inner
        }
    }

    pub fn from_points(start: Point, end: Point) -> Line {
        let inner = LineInner::from_points(start, end);
        Self {
            inner
        }
    }


    line_inner!(mut offset(x: f32, y: f32));

    line_inner!(nonmut renderable(col: graphics::Color) -> (graphics::Vertex, graphics::Vertex));

    pub fn point_of_intersection(&self, other: &Line) -> Option<Point> {
        self.inner.point_of_intersection(&other.inner)
    }

    pub fn cast_in_scene(&self, scene: &Scene) -> Self {
        let inner = self.inner.cast_in_scene(scene);
        Self {
            inner
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32
}

impl Point {
    pub fn min_by_x(self, other: Point) -> Point {
        if self.x < other.x {
            self
        } else {
            other
        }
    }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        assert!(!x.is_nan());
        assert!(!y.is_nan());
        Self {
            x,
            y
        }
    }
}

fn is_between(num: f32, min: f32, max: f32) -> bool {
    min <= num && num <= max
}


impl LineInner {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> LineInner {
        LineInner::from_points((x1, y1).into(), (x2, y2).into())
    }

    pub fn from_points(start: Point, end: Point) -> LineInner {
        if start.x == end.x {
            assert_eq!(start.x, end.x);
            Self::Vertical(VerticalLine::from_points(start, end))
        } else {
            assert_ne!(start.x, end.x);
            Self::Normal(NormalLine::from_points(start, end))
        }
    }

    line_impl!(mut offset(x: f32, y: f32));

    line_impl!(nonmut renderable(col: graphics::Color) -> (graphics::Vertex, graphics::Vertex));

    line_impl!(nonmut point_of_intersection(other: &LineInner) -> Option<Point>);

    pub fn cast_in_scene(&self, scene: &Scene) -> LineInner {
        match self.clone() {
            Self::Vertical(mut v) => {
                for wall in &scene.walls {
                    v = v.cast_to_line(&wall.inner);
                }
                Self::Vertical(v)
            },
            Self::Normal(mut v) => {
                for wall in &scene.walls {
                    v = v.cast_to_line(&wall.inner);
                }
                Self::Normal(v)
            },
        }
    }
}

impl VerticalLine {
    fn from_points(start: Point, end: Point) -> VerticalLine {
        assert_eq!(start.x, end.x);

        let start_y = start.y;
        let x = start.x;
        let end_y = end.y;

        assert!(!start_y.is_nan());
        assert!(!end_y.is_nan());
        // x cannot be NAN because the assert_eq would
        // have failed if it was

        let down_to_up = start_y < end_y;

        VerticalLine {
            x,
            max_y: start_y.max(end_y),
            min_y: start_y.min(end_y),
            down_to_up,
        }
    }
    fn offset(&mut self, x: f32, y: f32) {
        self.x += x;
        self.max_y += y;
        self.min_y += y;
    }

    fn renderable(&self, col: graphics::Color) -> (graphics::Vertex, graphics::Vertex) {
        let first_point = graphics::Vertex::new(
            system::Vector2f::new(self.x, self.min_y),
            col,
            system::Vector2f::new(0.0, 0.0),
        );
        let second_point = graphics::Vertex::new(
            system::Vector2f::new(self.x, self.max_y),
            col,
            system::Vector2f::new(0.0, 0.0),
        );
        (first_point, second_point)
    }

    fn bottom_point(&self) -> Point {
        (self.x, self.min_y).into()
    }

    fn top_point(&self) -> Point {
        (self.x, self.max_y).into()
    }

    fn cast_to_line(&self, other: &LineInner) -> VerticalLine {
        if let Some(intersection) = self.point_of_intersection(other) {
            if self.down_to_up {
                Self::from_points(self.bottom_point(), intersection)
            } else {
                Self::from_points(self.top_point(), intersection)
            }
        } else {
            self.clone()
        }
    }

    fn point_of_intersection(&self, other: &LineInner) -> Option<Point> {
        match other {
            LineInner::Vertical(v) => self.point_of_intersection_vert(v),
            LineInner::Normal(n) => self.point_of_intersection_normal(n),
        }
    }

    fn point_of_intersection_normal(&self, other: &NormalLine) -> Option<Point> {
        other.point_of_intersection_vertical(self)
    }

    fn point_of_intersection_vert(&self, other: &VerticalLine) -> Option<Point> {
        if self.x != other.x {
            return None
        }

        // TODO: This does not actually follow the line up/down
        // So if the first line is (0, 10) -> (0, 0) and the second
        // is (0, 1) -> (0, 2), this algorithm will still say
        // that the first point of intersection is (0, 1), when it
        // should be (0, 2)
        if is_between(other.min_y, self.min_y, self.max_y) {
            Some((self.x, other.min_y).into())
        } else if is_between(other.max_y, self.min_y, self.max_y) {
            Some((self.x, other.max_y).into())
        } else {
            None
        }
    }
}

impl NormalLine {
    fn from_points(start: Point, end: Point) -> Self {
        let start_x = start.x;
        let start_y = start.y;
        let end_x = end.x;
        let end_y = end.y;

        assert!(!start_x.is_nan());
        assert!(!start_y.is_nan());
        assert!(!end_x.is_nan());
        assert!(!end_y.is_nan());

        assert_ne!(start_x, end_x);

        let slope = (end_y - start_y) / (end_x - start_x);

        let y_intercept = (-slope * start_x) + start_y;

        if start_x > end_x {
            NormalLine {
                slope,
                y_intercept,
                min_x: end_x,
                max_x: start_x,
                left_to_right: false,
            }
        } else {
            NormalLine {
                slope,
                y_intercept,
                min_x: start_x,
                max_x: end_x,
                left_to_right: true,
            }
        }
    }

    fn left_point(&self) -> Point {
        self.point_at(self.min_x).unwrap()
    }

    fn right_point(&self) -> Point {
        self.point_at(self.max_x).unwrap()
    }

    fn offset(&mut self, x: f32, y: f32) {
        let offset = (x, y).into();
        assert_ne!(self.min_x, self.max_x);
        let lhs = self.left_point() + offset;
        let rhs = self.right_point() + offset;

        if self.left_to_right {
            *self = NormalLine::from_points(lhs, rhs);
        } else {
            *self = NormalLine::from_points(rhs, lhs);
        }
    }

    fn cast_to_line(&self, other: &LineInner) -> NormalLine {
        if let Some(intersection) = self.point_of_intersection(other) {
            if self.left_to_right {
                NormalLine::from_points(self.left_point(), intersection)
            } else {
                NormalLine::from_points(self.right_point(), intersection)
            }
        } else {
            self.clone()
        }
    }

    fn point_of_intersection(&self, other: &LineInner) -> Option<Point> {
        match other {
            LineInner::Vertical(v) => self.point_of_intersection_vertical(v),
            LineInner::Normal(n) => self.point_of_intersection_normal(n),
        }
    }

    fn point_of_intersection_vertical(&self, other: &VerticalLine) -> Option<Point> {
        if !is_between(other.x, self.min_x, self.max_x) {
            return None;
        }

        let y = self.y_at(other.x).unwrap();

        if !is_between(y, other.min_y, other.max_y) {
            return None;
        }

        Some((other.x, y).into())
    }

    fn point_of_intersection_normal(&self, other: &NormalLine) -> Option<Point> {
        if self.slope == other.slope {

            if self.y_intercept != other.y_intercept {
                return None
            }

            if is_between(self.min_x, other.min_x, other.max_x) {
                return self.point_at(self.min_x)
            }

            if is_between(self.max_x, other.min_x, other.max_x) {
                return self.point_at(self.max_x)
            }
        }

        let x = (other.y_intercept - self.y_intercept) / (self.slope - other.slope);

        if is_between(x, other.min_x, other.max_x) {
            self.point_at(x)
        } else {
            None
        }
    }

    fn renderable(&self, col: graphics::Color) -> (graphics::Vertex, graphics::Vertex) {
        let first_point = graphics::Vertex::new(
            system::Vector2f::new(self.min_x, self.y_at(self.min_x).unwrap()),
            col,
            system::Vector2f::new(0.0, 0.0),
        );
        let second_point = graphics::Vertex::new(
            system::Vector2f::new(self.max_x, self.y_at(self.max_x).unwrap()),
            col,
            system::Vector2f::new(0.0, 0.0),
        );
        (first_point, second_point)
    }

    fn point_at(&self, x: f32) -> Option<Point> {
        if is_between(x, self.min_x, self.max_x) {
            Some((x, self.y_at(x).unwrap()).into())
        } else {
            None
        }
    }

    fn y_at(&self, x: f32) -> Option<f32> {
        assert!(!x.is_nan());

        if is_between(x, self.min_x, self.max_x) {
            Some(self.slope * x + self.y_intercept)
        } else {
            None
        }
    }
}
