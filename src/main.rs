mod body;
mod command_line_flags;

use sfml::graphics::{RenderWindow, RenderTarget, Transformable, Color};
use sfml::window::{ContextSettings, Style, Event};
use sfml::system::Vector2f;
use crate::body::*;
use crate::command_line_flags::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let flags = get_command_line_flags(args);

    let mut window = RenderWindow::new(
        (1200, 800),
        "TEST",
        Style::CLOSE,
        &ContextSettings::default()
    );

    window.set_framerate_limit(60);

    let mut circle_vec = read_file_and_make_circles("scenarios/float_away.circles");
    let mut body_vec = read_file_and_make_bodies("scenarios/float_away.bodies");

    while window.is_open() {
        while let Some(event) =  window.poll_event() {
            match event {
                Event::Closed => window.close(), _ => {}
            }
        }
        window.clear(Color::rgb(0,0,0));

        let mut i = 0;
        while i < circle_vec.len() {
            let mut j = i + 1;
            while j < circle_vec.len() {
                let direction_vector = Vector2f::new(circle_vec[j].position().x - circle_vec[i].position().x, circle_vec[j].position().y - circle_vec[i].position().y);
                let distance = length_of_vector(direction_vector);
                let normalized_direction_vector = normalize(direction_vector, distance);
                let gravitational_force = calculate_force_of_gravity(body_vec[i].mass, body_vec[j].mass, distance);
                let body_type_modifier = body_type_multiplier(body_vec[i].body_type, body_vec[j].body_type);
                body_vec[i].velocity += normalized_direction_vector * gravitational_force * body_type_modifier as f32;
                body_vec[j].velocity += normalized_direction_vector * gravitational_force * body_type_modifier as f32 * -1_f32; // negate this since force is opposite direction
                j += 1;
            }
            circle_vec[i].move_(body_vec[i].velocity);
            window.draw(&circle_vec[i]);
            i += 1;
        }

        window.display();
    }
}
