mod body;
mod command_line_flags;

use sfml::graphics::{RenderWindow, RenderTarget, Transformable, Color, CircleShape, Shape, Texture};
use sfml::window::{ContextSettings, Style, Event};
use sfml::system::Vector2f;
use crate::body::*;
use crate::command_line_flags::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let flags = get_command_line_flags(args.clone());
    let scenario_name = args[1].to_string();

    let mut window = RenderWindow::new(
        (1200, 800),
        "gravi-bodies",
        Style::CLOSE,
        &ContextSettings::default()
    );

    window.set_framerate_limit(60);

    let circle_binding = "scenarios/".to_owned() + &scenario_name + ".circles";
    let body_binding = "scenarios/".to_owned() + &scenario_name + ".bodies";
    let mut circle_vec = read_file_and_make_circles(&circle_binding);
    let mut body_vec = read_file_and_make_bodies(&body_binding);

    let total_points = flags[Flags::DrawTrails as usize] * body_vec.len();
    let mut lines_vec = Vec::with_capacity(total_points);
    lines_vec.resize(total_points, CircleShape::new(0.0, 0));
    let mut current_point: usize = 0;

    while window.is_open() {
        while let Some(event) =  window.poll_event() {
            match event {
                Event::Closed => {
                    let mut texture = Texture::new().expect("Texture was not formed properly");
                    let _ = texture.create(window.size().x, window.size().y);
                    unsafe {texture.update_from_render_window(&window, 0, 0);}
                    let screenshot_name_binding = scenario_name.clone() + ".png";
                    let _ = texture.copy_to_image().expect("Image was not able to be copied").save_to_file(&screenshot_name_binding);
                    window.close()
                }, _ => {}
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
            if flags[Flags::DrawTrails as usize] != 0 {
                let mut circle = CircleShape::new(1.0, 10);
                let color = circle_vec[i].fill_color();
                circle.set_fill_color(Color::rgb(color.r, color.g, color.b));
                circle.set_position(Vector2f::new(circle_vec[i].position().x, circle_vec[i].position().y));
                lines_vec[current_point] = circle;
                for line in lines_vec.iter() {
                    window.draw(line);
                }
                current_point += 1;
                if current_point == total_points {current_point = 0;}
            }
            i += 1;
        }

        window.display();
    }
}
