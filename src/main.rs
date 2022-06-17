mod math;
#[cfg(test)]
mod tests;

use std::f32::consts::PI;

use math::{Line, Point, Scene};
use sfml::{
    graphics::{self, RenderTarget, RenderWindow},
    window::{self, mouse, Style}, system,
};

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 1200;

const FRAG_SHADER: &str = include_str!("./frag.frag");

fn main() {
    let mut window = RenderWindow::new(
        (WINDOW_HEIGHT, WINDOW_WIDTH),
        "Tracy!",
        Style::RESIZE,
        &Default::default(),
    );

    let mut scene_vertices = graphics::VertexArray::new(graphics::PrimitiveType::LINES, 2);

    let mut scene = Scene::new();

    // TODO: Support vertical lines
    scene.add_line(
        ((WINDOW_WIDTH / 2) as f32, 0.0).into(),
        ((WINDOW_WIDTH / 2) as f32 + 0.1, (WINDOW_HEIGHT / 2) as f32).into(),
        &mut scene_vertices,
    );

    const LINE_LENGTH: f32 = 500.0;

    let mut rays = (0..(360 * 4))
        .into_iter()
        .map(|x| (x / 4) as f32 * PI / 180.0)
        .map(|x| Line::new(0.0, 0.0, x.cos() * LINE_LENGTH, x.sin() * LINE_LENGTH))
        .collect::<Vec<_>>();

    let mut prev_x = 0;
    let mut prev_y = 0;

    let mut moving = true;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                window::Event::Closed => window.close(),
                window::Event::KeyPressed {
                    code: c,
                    alt: _,
                    ctrl: _,
                    shift: _,
                    system: _,
                } => match c {
                    window::Key::C => {
                        scene_vertices.clear();
                        scene = Scene::new();
                        scene.add_line(
                            ((WINDOW_WIDTH / 2) as f32, 0.0).into(),
                            ((WINDOW_WIDTH / 2) as f32 + 0.1, (WINDOW_HEIGHT / 2) as f32).into(),
                            &mut scene_vertices,
                        );
                    }
                    window::Key::ESCAPE => window.close(),
                    window::Key::S => {
                        moving = !moving;
                    }
                    _ => {}
                },
                window::Event::MouseButtonPressed { button: _, x, y } => {
                    let x = x as f32;
                    let y = y as f32;
                    let click_point: Point = (x, y).into();

                    scene.add_line_continuous(click_point, &mut scene_vertices);
                }
                window::Event::MouseMoved { x, y } => {
                    if moving {
                        let dx = x - prev_x;
                        let dy = y - prev_y;

                        for ray in &mut rays {
                            ray.offset(dx as f32, dy as f32);
                        }
                        prev_x = x;
                        prev_y = y;
                    }
                }

                _ => {}
            }
        }
        window.set_active(true);
        window.clear(graphics::Color::BLACK);

        let mut rays_arr = graphics::VertexArray::new(graphics::PrimitiveType::TRIANGLE_STRIP, 0);
        for ray in rays.iter() {
            let collided_ray = ray.cast_in_scene(&scene);

            let (start_v, end_v) = collided_ray.renderable(graphics::Color::WHITE);
            rays_arr.append(&start_v);
            rays_arr.append(&end_v);
        }

        window.draw(&scene_vertices);

        if let Some(mut shader) = graphics::Shader::from_memory(None, None, Some(FRAG_SHADER)) {
            shader.set_uniform_vec2("Centre", (prev_x as f32, prev_y as f32).into());
            shader.set_uniform_vec3("Colour", (255.0, 255.0, 255.0).into());
            shader.set_uniform_float("Radius", LINE_LENGTH);

            let mut states = graphics::RenderStates::default();
            states.set_shader(Some(&shader));

            window.draw_with_renderstates(&rays_arr, &states)
        } else {
            panic!("Could not use shader!");
        }

        window.display();
    }
}
