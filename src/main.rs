use std::{f32::consts::PI, ops::Sub};

use sfml::{
    graphics::{
        CircleShape, Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
    },
    system::Vector2f,
    window::{Event, Key, Style},
};

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 600;
const RADIUS: f32 = WINDOW_HEIGHT as f32 / 25.0;
fn main() {
    let mut window = RenderWindow::new(
        (WINDOW_HEIGHT, WINDOW_WIDTH),
        "Tracy!",
        Style::CLOSE,
        &Default::default(),
    );

    let mut circle = CircleShape::new(RADIUS, 25);
    let mut pos = Vector2f::new((WINDOW_HEIGHT / 2) as f32, (WINDOW_WIDTH / 2) as f32);
    let mut pointer = RectangleShape::new();
    pointer.set_fill_color(Color::RED);
    pointer.set_size((50.0, 10.0));

    circle.set_position(pos.sub(RADIUS));

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed {
                    code: Key::ESCAPE,
                    alt: _,
                    ctrl: _,
                    shift: _,
                    system: _,
                } => window.close(),
                _ => {}
            }
        }
        window.set_active(true);
        window.clear(Color::BLACK);

        let mut mouse_pos = Vector2f::new(
            window.mouse_position().x as f32,
            window.mouse_position().y as f32,
        );
        let mut angle = ((mouse_pos.y - pos.y) / (mouse_pos.x - pos.x))
            .atan()
            .to_degrees();
        if mouse_pos.x < pos.x {
            angle += PI.to_degrees();
        }

        pointer.set_rotation(angle);
        let (mouse_x, mouse_y, pos_x, pos_y, rect_width, rect_len) = (
            mouse_pos.x,
            mouse_pos.y,
            pos.x,
            pos.y,
            pointer.size().x as f32 / 2.0,
            pointer.size().y as f32,
        );
        let x = rect_len * (mouse_x - pos_x) / rect_width;
        let y = rect_len * (mouse_y - pos_y) / rect_width;
        pointer.set_position(pos + Vector2f::new(x, y));

        window.draw(&circle);
        window.draw(&pointer);
        window.display();
    }
}
