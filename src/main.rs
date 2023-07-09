use sfml::graphics::{RenderWindow, RenderTarget, CircleShape, Transformable, Shape, Color};
use sfml::window::{ContextSettings, Style, Event};
use sfml::system::{Vector2f};

fn main() {
    let mut window = RenderWindow::new(
        (1200, 800),
        "TEST",
        Style::CLOSE,
        &ContextSettings::default()
    );

    window.set_framerate_limit(60);

    let mut circle = CircleShape::new(300f32, 10);
    circle.set_position(Vector2f::new(100.0, 100.0));
    circle.set_fill_color(Color::rgb(0,0,255));

    while window.is_open() {
        while let Some(event) =  window.poll_event() {
            match event {
                Event::Closed => window.close(), _ => {}
            }
        }
        window.clear(Color::rgb(0,0,0));
        circle.move_(Vector2f::new(1.0,1.0));
        window.draw(&circle);
        window.display();
    }
}
