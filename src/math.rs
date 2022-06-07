use std::io::Write;

use sfml::{graphics, system};

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

    pub fn add_line_no_graphics(&mut self, start: Point, end: Point) {
        self.last_point = Some(end);
        let line = Line::from_points(start, end);

        self.walls.push(line);
    }

    pub fn add_line_continuous_no_graphics(&mut self, new: Point) {
        if let Some(last) = self.last_point {
            self.add_line_no_graphics(last, new);
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Line {
    slope: f32,
    y_intercept: f32,
    min_x: f32,
    max_x: f32,
    left_to_right: bool
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

impl Line {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        Line::from_points((x1, y1).into(), (x2, y2).into())
    }

    pub fn cast_in_scene(&self, scene: &Scene) -> Line {
        let mut line = self.clone();
        for wall in &scene.walls {
            println!("--------------------");
            dbg!(&line);
            dbg!(wall);
            std::io::stderr().flush().unwrap();
            line = line.cast_to_line(wall);
        }
        line
    }

    pub fn from_points(start: Point, end: Point) -> Self {
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
            Line {
                slope,
                y_intercept,
                min_x: end_x,
                max_x: start_x,
                left_to_right: false,
            }
        } else {
            Line {
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

    pub fn offset(&mut self, x: f32, y: f32) {
        self.y_intercept += y - self.slope * x;
    }

    pub fn cast_to_line(&self, other: &Line) -> Line {
        if let Some(intersection) = self.point_of_intersection(other) {
            if self.left_to_right {
                Line::from_points(self.left_point(), intersection)
            } else {
                Line::from_points(self.right_point(), intersection)
            }
        } else {
            self.clone()
        }
    }

    pub fn point_of_intersection(&self, other: &Line) -> Option<Point> {
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

    pub fn renderable(&self, col: graphics::Color) -> (graphics::Vertex, graphics::Vertex) {
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
