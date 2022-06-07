#[cfg(test)]
mod tests;
mod math;

use math::{Scene, Point, Line};
use sfml::{
    graphics::{RenderWindow, self, RenderTarget},
    window::{Style, self, mouse},
};

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 1200;

fn main() {
    let mut window = RenderWindow::new(
        (WINDOW_HEIGHT, WINDOW_WIDTH),
        "Tracy!",
        Style::CLOSE,
        &Default::default(),
    );

    let mut vertexarr = graphics::VertexArray::new(graphics::PrimitiveType::LINES, 2);

    let mut scene = Scene::new();
    // TODO: Support vertical lines
    scene.add_line(((WINDOW_WIDTH / 2) as f32, 0.0).into(), ((WINDOW_WIDTH / 2) as f32 + 0.1, (WINDOW_HEIGHT / 2) as f32).into(), &mut vertexarr);

    let mut from_bottom_left = true;

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
                    vertexarr.clear();
                    scene = Scene::new();
                    scene.add_line(((WINDOW_WIDTH / 2) as f32, 0.0).into(), ((WINDOW_WIDTH / 2) as f32 + 0.1, (WINDOW_HEIGHT / 2) as f32).into(), &mut vertexarr);
                },
                window::Event::KeyPressed {
                    code: window::Key::S,
                    alt: _,
                    ctrl: _,
                    shift: _,
                    system: _,
                } => {
                    scene.re_init_graphics(&mut vertexarr);

                    from_bottom_left = !from_bottom_left;
                },
                window::Event::MouseButtonPressed {
                    button: b,
                    x,
                    y
                } => {
                    let x = x as f32;
                    let y = y as f32;
                    let click_point: Point = (x, y).into();

                    match b {
                        mouse::Button::LEFT => {
                            let start_point: Point = if from_bottom_left {
                                (0.0, WINDOW_HEIGHT as f32).into()
                            } else {
                                (WINDOW_WIDTH as f32, 0.0).into()
                            };

                            let full_line = Line::from_points(start_point, click_point);

                            let line_to_render = full_line.cast_in_scene(&scene);

                            let (start_vertex, end_vertex) = line_to_render.renderable(graphics::Color::RED);

                            vertexarr.append(&start_vertex);
                            vertexarr.append(&end_vertex);
                        },
                        mouse::Button::RIGHT => scene.add_line_continuous(click_point, &mut vertexarr),
                        _ => {}
                    }

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
