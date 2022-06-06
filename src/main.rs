mod tests;

use sfml::{
    graphics::{RenderWindow, self, RenderTarget},
    window::{Style, self, mouse}, system,
};

#[derive(Debug)]
struct Scene {
    walls: Vec<Line>
}

#[derive(Debug)]
pub struct Line {
    slope: f32,
    y_intercept: f32,
    min_x: f32,
    max_x: f32
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn min_by_x(self, other: Point) -> Point {
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
            }
        } else {
            Line {
                slope,
                y_intercept,
                min_x: start_x,
                max_x: end_x,
            }
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

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let mut window = RenderWindow::new(
        (WINDOW_HEIGHT, WINDOW_WIDTH),
        "Tracy!",
        Style::CLOSE,
        &Default::default(),
    );

    let main_line = Line {
        slope: 1.0,
        y_intercept: 0.0,
        min_x: 0.0,
        max_x: (WINDOW_WIDTH / 2) as f32,
    };

    let mut vertexarr = graphics::VertexArray::new(graphics::PrimitiveType::LINES, 2);

    let (start, end) = main_line.renderable(graphics::Color::GREEN);

    vertexarr.append(&start);
    vertexarr.append(&end);


    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                window::Event::Closed => window.close(),
                window::Event::KeyPressed {
                    code: window::Key::ESCAPE,
                    alt: _,
                    ctrl: _,
                    shift: _,
                    system: _,
                } => window.close(),
                window::Event::KeyPressed {
                    code: window::Key::C,
                    alt: _,
                    ctrl: _,
                    shift: _,
                    system: _,
                } => {
                    vertexarr = graphics::VertexArray::new(graphics::PrimitiveType::LINES, 2);

                    let (start, end) = main_line.renderable(graphics::Color::GREEN);

                    vertexarr.append(&start);
                    vertexarr.append(&end);
                },
                window::Event::MouseButtonPressed {
                    button: _,
                    x,
                    y
                } => {
                    let x = x as f32;
                    let y = y as f32;
                    let click_point: Point = (x, y).into();

                    let start_point: Point = (0.0, WINDOW_HEIGHT as f32).into();

                    let full_line = Line::from_points(start_point, click_point);

                    let line_to_render = if let Some(intersection) = full_line.point_of_intersection(&main_line) {
                        Line::from_points(start_point, click_point.min_by_x(intersection))
                    } else {
                        full_line
                    };

                    let (start_vertex, end_vertex) = line_to_render.renderable(graphics::Color::RED);

                    vertexarr.append(&start_vertex);
                    vertexarr.append(&end_vertex);

                }
                _ => {}
            }
        }
        window.set_active(true);
        window.clear(graphics::Color::BLACK);

        window.draw(&vertexarr);
        window.display();
    }
}
