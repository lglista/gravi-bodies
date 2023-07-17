mod body;

use std::vec;

use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Transformable, Shape, Color};
use sfml::window::{ContextSettings, Style, Event};
use sfml::system::Vector2f;
use crate::body::*;

fn main() {
    let mut window = RenderWindow::new(
        (1200, 800),
        "TEST",
        Style::CLOSE,
        &ContextSettings::default()
    );

    window.set_framerate_limit(60);

    let mut circle_vec = vec![CircleShape::new(10f32, 10), CircleShape::new(10f32, 10)];
    let mut body_vec = vec![Body::new(Vector2f::new(0.0,0.0), 100.0, BodyType::NEUTRAL), Body::new(Vector2f::new(0.0,0.0), 100.0, BodyType::NEUTRAL)];

    circle_vec[0].set_position(Vector2f::new(400.0, 100.0));
    circle_vec[0].set_fill_color(Color::rgb(0,0,255));
    circle_vec[1].set_position(Vector2f::new(200.0, 100.0));
    circle_vec[1].set_fill_color(Color::rgb(255,0,0));

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
